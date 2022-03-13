//! This module contains the functionality for generating reports.

use crate::config::DisplayMode;
use crate::error::Error;
use crate::result::Result;
use crate::status::Status;
use git2::{ErrorCode, Reference, Repository, StatusOptions};
use log::{debug, trace};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::Path;

mod target;

const HEAD: &str = "HEAD";

/// This type represents a [`BTreeMap`] using an optional [`String`] for keys, which represents the
/// parent directory for a group of reports ([`Vec<Report>`]). The values corresponding to those keys
/// are the actual groups of reports.
// NOTE: We use a BTreeMap over a HashMap for sorted keys.
pub type Reports = BTreeMap<Option<String>, Vec<Report>>;

/// A collection of results for a Git repository at a given path.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Report {
    /// The directory name of the Git repository.
    pub name: String,
    /// The name of the current, open branch.
    pub branch: String,
    /// The [`Status`] of the working tree.
    pub status: Status,

    /// The parent directory of the `path` field. The value will be `None` if a parent is not found.
    pub parent: Option<String>,
    /// The remote origin URL. The value will be `None` if the URL cannot be found.
    pub url: Option<String>,

    /// The "user.email" of a Git config that's only collected when using [`DisplayMode::Standard`].
    pub email: Option<String>,
}

impl Report {
    fn new(
        path: &Path,
        branch: &str,
        status: &Status,
        url: Option<String>,
        email: Option<String>,
    ) -> Result<Self> {
        Ok(Self {
            name: match path.file_name() {
                Some(s) => match s.to_str() {
                    Some(s) => s.to_string(),
                    None => return Err(Error::FileNameToStrConversionFailure(path.to_path_buf())),
                },
                None => return Err(Error::FileNameNotFound(path.to_path_buf())),
            },
            branch: (*branch).into(),
            status: *status,
            parent: match path.parent() {
                Some(s) => match s.to_str() {
                    Some(s) => Some(s.to_string()),
                    None => return Err(Error::PathToStrConversionFailure(s.to_path_buf())),
                },
                None => None,
            },
            url,
            email,
        })
    }
}

/// Generate [`Reports`] for a given path and its children. The [`DisplayMode`] is required because
/// any two display modes can require differing ammounts of data to be collected.
pub fn generate_reports(path: &Path, display_mode: &DisplayMode) -> Result<Reports> {
    let include_email = match display_mode {
        DisplayMode::Standard | DisplayMode::Json => true,
        DisplayMode::Classic => false,
    };

    let unprocessed = target::recursive_target_gen(path)?
        .par_iter()
        .map(|path| generate_report(path, include_email))
        .collect::<Vec<Result<Report>>>();

    let mut processed = Reports::new();
    for wrapped_report in unprocessed {
        match wrapped_report {
            Ok(report) => {
                if let Some(mut v) = processed.insert(report.parent.clone(), vec![report.clone()]) {
                    v.push(report.clone());
                    processed.insert(report.parent, v);
                }
            }
            Err(e) => return Err(e),
        }
    }
    Ok(processed)
}

/// Generates a report with a given path.
fn generate_report(repo_path: &Path, include_email: bool) -> Result<Report> {
    debug!("attemping to generate report for path: {:?}", repo_path);
    let repo = Repository::open(repo_path)?;
    let head = match repo.head() {
        Ok(head) => Some(head),
        Err(ref e) if e.code() == ErrorCode::UnbornBranch || e.code() == ErrorCode::NotFound => {
            None
        }
        Err(e) => return Err(Error::Git2Rs(e)),
    };
    let branch = match &head {
        Some(head) => head
            .shorthand()
            .ok_or(Error::GitReferenceShorthandInvalid)?,
        None => HEAD,
    };

    // We'll include all untracked files and directories in the status options.
    let mut opts = StatusOptions::new();
    opts.include_untracked(true).recurse_untracked_dirs(true);

    // If "head" is "None" and statuses are empty, then the repository must be clean because there
    // are no commits to push.
    let status = match repo.statuses(Some(&mut opts)) {
        Ok(v) if v.is_empty() => match &head {
            Some(head) => match is_unpushed(&repo, head)? {
                true => Status::Unpushed,
                false => Status::Clean,
            },
            None => Status::Clean,
        },
        Ok(_) => Status::Unclean,
        Err(e) if e.code() == ErrorCode::BareRepo => Status::Bare,
        Err(e) => return Err(e.into()),
    };

    let url = match repo.find_remote("origin") {
        Ok(origin) => origin.url().map(|s| s.to_string()),
        Err(e) if e.code() == ErrorCode::NotFound => None,
        Err(e) => return Err(Error::Git2Rs(e)),
    };
    let email = match include_email {
        true => get_email(&repo),
        false => None,
    };
    debug!(
        "generating report for repository at {:?} on branch {} with status {:?}, url {:?}, and email {:?}",
        &repo_path, &branch, &status, &url, &email
    );

    Report::new(repo_path, branch, &status, url, email)
}

/// Checks if local commit(s) on the current branch have not yet been pushed to the remote.
fn is_unpushed(repo: &Repository, head: &Reference) -> Result<bool> {
    let local_head = head.peel_to_commit()?;
    let remote = format!(
        "origin/{}",
        match head.shorthand() {
            Some(v) => v,
            None => {
                trace!("assuming unpushed; could not determine shorthand for head");
                return Ok(true);
            }
        }
    );
    let remote_head = repo
        .resolve_reference_from_short_name(&remote)?
        .peel_to_commit()?;
    Ok(
        matches!(repo.graph_ahead_behind(local_head.id(), remote_head.id()), Ok(number_unique_commits) if number_unique_commits.0 > 0),
    )
}

/// Find the "user.email" value in the local or global Git config. The
/// [`git2::Repository::config()`] method will look for a local config first and fallback to
/// global, as needed. Absorb and log any and all errors as the email field is non-critical to
/// the final results.
fn get_email(repository: &Repository) -> Option<String> {
    let config = match repository.config() {
        Ok(v) => v,
        Err(e) => {
            trace!("ignored error: {}", e);
            return None;
        }
    };
    let entries = match config.entries(None) {
        Ok(v) => v,
        Err(e) => {
            trace!("ignored error: {}", e);
            return None;
        }
    };

    // Greedily find our "user.email" value. Return the first result found.
    for entry in &entries {
        match entry {
            Ok(entry) => {
                if let Some(name) = entry.name() {
                    if name == "user.email" {
                        if let Some(value) = entry.value() {
                            return Some(value.to_string());
                        }
                    }
                }
            }
            Err(e) => debug!("ignored error: {}", e),
        }
    }
    None
}

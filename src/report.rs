use crate::config::DisplayMode;
use crate::error::Error;
use crate::status::Status;
use anyhow::Result;
use git2::{ErrorCode, Reference, Repository, StatusOptions};
use log::{debug, error, trace, warn};
use rayon::prelude::*;
use std::collections::BTreeMap;
use std::fs::DirEntry;
use std::path::{Path, PathBuf};
use std::{fs, io};

// Use a BTreeMap over a HashMap for sorted keys.
pub type Reports = BTreeMap<Option<String>, Vec<Report>>;

#[derive(Clone, Debug)]
pub struct Report {
    pub path: String,
    pub branch: String,
    pub status: Status,

    // Fields that can report NONE with impeding execution.
    pub parent: Option<String>,
    pub url: Option<String>,

    // Optional field that's only used in DisplayMode::Standard.
    pub email: Option<String>,
}

impl Report {
    pub fn new(
        path: &str,
        branch: &str,
        status: &Status,
        parent: Option<String>,
        url: Option<String>,
        email: Option<String>,
    ) -> Self {
        Self {
            path: (*path).into(),
            branch: (*branch).into(),
            status: *status,
            parent,
            url,
            email,
        }
    }

    pub fn generate_reports(path: &Path, display_mode: &DisplayMode) -> Result<Reports> {
        let include_email = match display_mode {
            DisplayMode::Standard => true,
            DisplayMode::Classic => false,
        };

        let unprocessed = recursive_target_gen(path)?
            .par_iter()
            .map(|path| generate_report(path, include_email))
            .collect::<Vec<Result<Report>>>();

        let mut processed = Reports::new();
        for wrapped_report in unprocessed {
            match wrapped_report {
                Ok(report) => {
                    if let Some(mut v) =
                        processed.insert(report.parent.clone(), vec![report.clone()])
                    {
                        v.push(report.clone());
                        processed.insert(report.parent, v);
                    }
                }
                Err(e) => return Err(e),
            }
        }
        Ok(processed)
    }
}

type Target = io::Result<Option<Vec<PathBuf>>>;

fn recursive_target_gen(path: &Path) -> io::Result<Vec<PathBuf>> {
    let entries: Vec<DirEntry> = match fs::read_dir(&path) {
        Ok(o) => o.filter_map(|r| r.ok()).collect(),
        Err(e) if e.kind() == io::ErrorKind::PermissionDenied => {
            warn!("{}: {}", e, &path.display());
            return Ok(vec![]);
        }
        Err(e) => {
            error!("{}: {}", e, &path.display());
            return Ok(vec![]);
        }
    };

    let targets = entries
        .par_iter()
        .map(process_entry)
        .collect::<Vec<Target>>();

    let mut results = vec![];
    for target in targets {
        match target {
            Ok(v) => {
                if let Some(mut v) = v {
                    if !v.is_empty() {
                        results.append(&mut v);
                    }
                }
            }
            Err(e) => return Err(e),
        }
    }
    Ok(results)
}

// Ensure the entry is a directory and is not hidden. Then, check if a Git sub directory exists,
// which will indicate if the entry is a repository. Finally, generate targets based on that
// repository.
fn process_entry(entry: &DirEntry) -> Target {
    match entry.file_type()?.is_dir()
        && !entry
            .file_name()
            .to_str()
            .map(|file_name| file_name.starts_with('.'))
            .unwrap_or(false)
    {
        true => {
            let path = entry.path();
            let git_sub_directory = path.join(".git");
            match git_sub_directory.exists() && git_sub_directory.is_dir() {
                true => Ok(Some(vec![path])),
                false => Ok(Some(recursive_target_gen(&path)?)),
            }
        }
        false => Ok(None),
    }
}

fn generate_report(repo_path: &Path, include_email: bool) -> Result<Report> {
    let repo = Repository::open(repo_path)?;
    let head = repo.head()?;
    let branch = head
        .shorthand()
        .ok_or(Error::GitReferenceShorthandInvalid)?;

    let mut opts = StatusOptions::new();
    let status = match repo.statuses(Some(&mut opts)) {
        Ok(v) if v.is_empty() => match is_unpushed(&repo, &head)? {
            true => Status::Unpushed,
            false => Status::Clean,
        },
        Ok(_) => Status::Unclean,
        Err(e) if e.code() == ErrorCode::BareRepo => Status::Bare,
        Err(e) => return Err(e.into()),
    };

    debug!(
        "generating report for repository at {:?} on branch {} with status: {:?}",
        &repo_path, &branch, &status
    );
    let origin = repo.find_remote("origin")?;
    Ok(Report::new(
        match repo_path.file_name() {
            Some(s) => s
                .to_str()
                .ok_or_else(|| Error::FileNameStrConversionFailure(repo_path.to_path_buf()))?,
            None => return Err(Error::FileNameNotFound(repo_path.to_path_buf()).into()),
        },
        branch,
        &status,
        match repo_path.parent() {
            Some(s) => match s.to_str() {
                Some(s) => Some(s.to_string()),
                None => return Err(Error::PathToStrConversionFailure(s.to_path_buf()).into()),
            },
            None => None,
        },
        origin.url().map(|s| s.to_string()),
        match include_email {
            true => get_email(&repo),
            false => None,
        },
    ))
}

fn is_unpushed(repo: &Repository, head: &Reference) -> Result<bool> {
    let local_head = head.peel_to_commit()?;
    let remote = format!(
        "origin/{}",
        head.shorthand()
            .ok_or(Error::GitReferenceShorthandInvalid)?
    );
    let remote_head = repo
        .resolve_reference_from_short_name(&remote)?
        .peel_to_commit()?;
    Ok(
        matches!(repo.graph_ahead_behind(local_head.id(), remote_head.id()), Ok(number_unique_commits) if number_unique_commits.0 > 0),
    )
}

// Find the "user.email" value in the local or global Git config. The "config" method for a
// "git2::Repository" object will look for a local config first and fallback to global, as needed.
// Absorb and log any and all errors as the email field is non-critical to our final results.
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

//! This module contains [`RepositoryView`], which provides the [`Status`]
//! and general overview of the state of a given Git repository.

use std::path::Path;

use anyhow::{Result, anyhow, bail};
use git2::Repository;
use log::{debug, trace};
use serde::{Deserialize, Serialize};
use submodule_view::SubmoduleView;

use crate::status::Status;

mod submodule_view;

/// A collection of results for a Git repository at a given path.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct RepositoryView {
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

    /// The email used in either the local or global config for the repository.
    pub email: Option<String>,
    /// Views of submodules found within the repository.
    pub submodules: Vec<SubmoduleView>,
}

impl RepositoryView {
    /// Generates a collector for a given path.
    pub fn new(
        repo_path: &Path,
        include_email: bool,
        include_submodules: bool,
    ) -> Result<RepositoryView> {
        debug!(
            "attempting to generate collector for repository_view at path: {}",
            repo_path.display()
        );

        let repo = Repository::open(repo_path)?;
        let (status, head, remote) = Status::find(&repo)?;

        let submodules = if include_submodules {
            SubmoduleView::list(&repo)?
        } else {
            Vec::with_capacity(0)
        };

        let branch = match &head {
            Some(head) => head
                .shorthand()
                .ok_or(anyhow!("full shorthand for Git reference is invalid UTF-8"))?,
            None => "HEAD",
        };

        let url = match remote {
            Some(remote) => remote.url().map(|s| s.to_string()),
            None => None,
        };

        let email = match include_email {
            true => Self::get_email(&repo),
            false => None,
        };

        debug!(
            "finalized collector collection for repository_view at path: {}",
            repo_path.display()
        );
        RepositoryView::finalize(
            repo_path,
            Some(branch.to_string()),
            status,
            url,
            email,
            submodules,
        )
    }

    /// Assemble a [`RepositoryView`] with metadata for a given repository.
    pub fn finalize(
        path: &Path,
        branch: Option<String>,
        status: Status,
        url: Option<String>,
        email: Option<String>,
        submodules: Vec<SubmoduleView>,
    ) -> Result<Self> {
        let name = match path.file_name() {
            Some(s) => match s.to_str() {
                Some(s) => s.to_string(),
                None => bail!("could not convert file name (&OsStr) to &str: {path:?}"),
            },
            None => bail!("received None (Option<&OsStr>) for file name: {path:?}"),
        };
        let parent = match path.parent() {
            Some(s) => match s.to_str() {
                Some(s) => Some(s.to_string()),
                None => bail!("could not convert path (Path) to &str: {s:?}"),
            },
            None => None,
        };
        let branch = match branch {
            Some(branch) => branch,
            None => "unknown".to_string(),
        };

        Ok(Self {
            name,
            branch,
            status,
            parent,
            url,
            email,
            submodules,
        })
    }

    /// Find the "user.email" value in the local or global Git config. The
    /// [`Repository::config()`] method will look for a local config first and fallback to
    /// global, as needed. Absorb and log any and all errors as the email field is non-critical to
    /// the final results.
    fn get_email(repository: &Repository) -> Option<String> {
        let config = match repository.config() {
            Ok(v) => v,
            Err(e) => {
                trace!("ignored error: {e}");
                return None;
            }
        };
        let mut entries = match config.entries(None) {
            Ok(v) => v,
            Err(e) => {
                trace!("ignored error: {e}");
                return None;
            }
        };

        // Greedily find our "user.email" value. Return the first result found.
        while let Some(entry) = entries.next() {
            match entry {
                Ok(entry) => {
                    if let Some(name) = entry.name()
                        && name == "user.email"
                        && let Some(value) = entry.value()
                    {
                        return Some(value.to_string());
                    }
                }
                Err(e) => debug!("ignored error: {e}"),
            }
        }
        None
    }
}

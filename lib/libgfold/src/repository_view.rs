//! This module contains [`RepositoryView`], which provides the [`Status`]
//! and general overview of the state of a given Git repository.

use git2::Repository;
use log::{debug, error, trace};
use serde::{Deserialize, Serialize};
use std::io;
use std::path::{Path, PathBuf};
use submodule_view::SubmoduleView;
use thiserror::Error;

use crate::repository_view::submodule_view::SubmoduleError;
use crate::status::{Status, StatusError};

mod submodule_view;

#[allow(missing_docs)]
#[remain::sorted]
#[derive(Error, Debug)]
pub enum RepositoryViewError {
    #[error("received None (Option<&OsStr>) for file name: {0}")]
    FileNameNotFound(PathBuf),
    #[error("could not convert file name (&OsStr) to &str: {0}")]
    FileNameToStrConversionFailure(PathBuf),
    #[error(transparent)]
    FromGit2(#[from] git2::Error),
    #[error(transparent)]
    FromStatus(#[from] StatusError),
    #[error(transparent)]
    FromStdIo(#[from] io::Error),
    #[error(transparent)]
    FromSubmodule(#[from] SubmoduleError),
    #[error("full shorthand for Git reference is invalid UTF-8")]
    GitReferenceShorthandInvalid,
    #[error("could not convert path (Path) to &str: {0}")]
    PathToStrConversionFailure(PathBuf),
}

#[allow(missing_docs)]
pub type RepositoryViewResult<T> = Result<T, RepositoryViewError>;

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
    ) -> RepositoryViewResult<RepositoryView> {
        debug!(
            "attempting to generate collector for repository_view at path: {:?}",
            repo_path
        );

        let repo = match Repository::open(repo_path) {
            Ok(repo) => repo,
            Err(e) if e.message() == "unsupported extension name extensions.worktreeconfig" => {
                error!("skipping error ({e}) until upstream libgit2 issue is resolved: https://github.com/libgit2/libgit2/issues/6044");
                let unknown_report = RepositoryView::finalize(
                    repo_path,
                    None,
                    Status::Unknown,
                    None,
                    None,
                    Vec::with_capacity(0),
                )?;
                return Ok(unknown_report);
            }
            Err(e) => return Err(e.into()),
        };
        let (status, head, remote) = Status::find(&repo)?;

        let submodules = if include_submodules {
            SubmoduleView::list(&repo)?
        } else {
            Vec::with_capacity(0)
        };

        let branch = match &head {
            Some(head) => head
                .shorthand()
                .ok_or(RepositoryViewError::GitReferenceShorthandInvalid)?,
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
            "finalized collector collection for repository_view at path: {:?}",
            repo_path
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
    ) -> Result<Self, RepositoryViewError> {
        let name = match path.file_name() {
            Some(s) => match s.to_str() {
                Some(s) => s.to_string(),
                None => {
                    return Err(RepositoryViewError::FileNameToStrConversionFailure(
                        path.to_path_buf(),
                    ));
                }
            },
            None => return Err(RepositoryViewError::FileNameNotFound(path.to_path_buf())),
        };
        let parent = match path.parent() {
            Some(s) => match s.to_str() {
                Some(s) => Some(s.to_string()),
                None => {
                    return Err(RepositoryViewError::PathToStrConversionFailure(
                        s.to_path_buf(),
                    ));
                }
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
                trace!("ignored error: {}", e);
                return None;
            }
        };
        let mut entries = match config.entries(None) {
            Ok(v) => v,
            Err(e) => {
                trace!("ignored error: {}", e);
                return None;
            }
        };

        // Greedily find our "user.email" value. Return the first result found.
        while let Some(entry) = entries.next() {
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
}

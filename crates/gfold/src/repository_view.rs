use git2::{ErrorCode, Reference, Remote, Repository, StatusOptions};
use log::{debug, error, trace};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use submodule_view::SubmoduleView;
use thiserror::Error;

use crate::status::Status;

mod submodule_view;

#[derive(Error, Debug)]
pub enum RepositoryViewError {
    #[error("received None (Option<&OsStr>) for file name: {0}")]
    FileNameNotFound(PathBuf),
    #[error("could not convert file name (&OsStr) to &str: {0}")]
    FileNameToStrConversionFailure(PathBuf),
    #[error("full shorthand for Git reference is invalid UTF-8")]
    GitReferenceShorthandInvalid,
    #[error("could not convert path (Path) to &str: {0}")]
    PathToStrConversionFailure(PathBuf),
}

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

    /// The "user.email" of a Git config that's only collected when using
    /// [`DisplayMode::Standard`](crate::config::DisplayMode::Standard)
    /// and [`DisplayMode::Json`](crate::config::DisplayMode::Json).
    pub email: Option<String>,
    /// The submodules of a repository_view that are only collected when using
    /// [`DisplayMode::Json`](crate::config::DisplayMode::Json).
    pub submodules: Vec<SubmoduleView>,
}

impl RepositoryView {
    /// Generates a collector for a given path.
    pub fn new(
        repo_path: &Path,
        include_email: bool,
        include_submodules: bool,
    ) -> anyhow::Result<RepositoryView> {
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
        let (status, head, remote) = RepositoryView::find_status(&repo)?;

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
        Ok(RepositoryView::finalize(
            repo_path,
            Some(branch.to_string()),
            status,
            url,
            email,
            submodules,
        )?)
    }

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
                    ))
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
                    ))
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

    /// Find the [`Status`] for a given [`Repository`](git2::Repository). The
    /// [`head`](Option<git2::Reference>) and [`remote`](Option<git2::Remote>) are also returned.
    pub fn find_status(
        repo: &Repository,
    ) -> anyhow::Result<(Status, Option<Reference>, Option<Remote>)> {
        let head = match repo.head() {
            Ok(head) => Some(head),
            Err(ref e)
                if e.code() == ErrorCode::UnbornBranch || e.code() == ErrorCode::NotFound =>
            {
                None
            }
            Err(e) => return Err(e.into()),
        };

        // Greedily chooses a remote if "origin" is not found.
        let (remote, remote_name) = match repo.find_remote("origin") {
            Ok(origin) => (Some(origin), Some("origin".to_string())),
            Err(e) if e.code() == ErrorCode::NotFound => Self::choose_remote_greedily(repo)?,
            Err(e) => return Err(e.into()),
        };

        // We'll include all untracked files and directories in the status options.
        let mut opts = StatusOptions::new();
        opts.include_untracked(true).recurse_untracked_dirs(true);

        // If "head" is "None" and statuses are empty, then the repository_view must be clean because there
        // are no commits to push.
        let status = match repo.statuses(Some(&mut opts)) {
            Ok(v) if v.is_empty() => match &head {
                Some(head) => match remote_name {
                    Some(remote_name) => {
                        match RepositoryView::is_unpushed(repo, head, &remote_name)? {
                            true => Status::Unpushed,
                            false => Status::Clean,
                        }
                    }
                    None => Status::Clean,
                },
                None => Status::Clean,
            },
            Ok(_) => Status::Unclean,
            Err(e) if e.code() == ErrorCode::BareRepo => Status::Bare,
            Err(e) => return Err(e.into()),
        };

        Ok((status, head, remote))
    }

    fn choose_remote_greedily(
        repository: &Repository,
    ) -> Result<(Option<Remote>, Option<String>), git2::Error> {
        let remotes = repository.remotes()?;
        Ok(match remotes.get(0) {
            Some(remote_name) => (
                Some(repository.find_remote(remote_name)?),
                Some(remote_name.to_string()),
            ),
            None => (None, None),
        })
    }

    /// Checks if local commit(s) on the current branch have not yet been pushed to the remote.
    fn is_unpushed(
        repo: &Repository,
        head: &Reference,
        remote_name: &str,
    ) -> Result<bool, git2::Error> {
        let local_head = head.peel_to_commit()?;
        let remote = format!(
            "{}/{}",
            remote_name,
            match head.shorthand() {
                Some(v) => v,
                None => {
                    debug!("assuming unpushed; could not determine shorthand for head");
                    return Ok(true);
                }
            }
        );
        let remote_head = match repo.resolve_reference_from_short_name(&remote) {
            Ok(reference) => reference.peel_to_commit()?,
            Err(e) => {
                debug!("assuming unpushed; could not resolve remote reference from short name (ignored error: {})", e);
                return Ok(true);
            }
        };
        Ok(
            matches!(repo.graph_ahead_behind(local_head.id(), remote_head.id()), Ok(number_unique_commits) if number_unique_commits.0 > 0),
        )
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

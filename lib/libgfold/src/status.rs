//! This module contains the [`crate::status::Status`] type.

use git2::{ErrorCode, Reference, Remote, Repository, StatusOptions};
use log::debug;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[allow(missing_docs)]
#[remain::sorted]
#[derive(Error, Debug)]
pub enum StatusError {
    #[error(transparent)]
    FromGit2(#[from] git2::Error),
}

#[allow(missing_docs)]
pub type StatusResult<T> = Result<T, StatusError>;

/// A summarized interpretation of the status of a Git working tree.
#[remain::sorted]
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Status {
    /// Corresponds to a "bare" working tree.
    Bare,
    /// Corresponds to a "clean" working tree.
    Clean,
    /// Corresponds to an "unclean" working tree.
    Unclean,
    /// Provided if the state of the working tree could neither be found nor determined.
    Unknown,
    /// Indicates that there is at least one commit not pushed to the remote from the working tree.
    Unpushed,
}

impl Status {
    /// Converts the enum into a borrowed, static `str`.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Bare => "bare",
            Self::Clean => "clean",
            Self::Unclean => "unclean",
            Self::Unknown => "unknown",
            Self::Unpushed => "unpushed",
        }
    }

    /// Find the [`Status`] for a given [`Repository`]. The
    /// [`head`](Option<git2::Reference>) and [`remote`](Option<git2::Remote>) are also returned.
    pub fn find(
        repo: &Repository,
    ) -> StatusResult<(Status, Option<Reference<'_>>, Option<Remote<'_>>)> {
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
                    Some(remote_name) => match Self::is_unpushed(repo, head, &remote_name)? {
                        true => Status::Unpushed,
                        false => Status::Clean,
                    },
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

    // Checks if local commit(s) on the current branch have not yet been pushed to the remote.
    fn is_unpushed(
        repo: &Repository,
        head: &Reference<'_>,
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

    fn choose_remote_greedily(
        repository: &Repository,
    ) -> Result<(Option<Remote<'_>>, Option<String>), git2::Error> {
        let remotes = repository.remotes()?;
        Ok(match remotes.get(0) {
            Some(remote_name) => (
                Some(repository.find_remote(remote_name)?),
                Some(remote_name.to_string()),
            ),
            None => (None, None),
        })
    }
}

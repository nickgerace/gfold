//! This module contains the ability to gather information on submodules for a given [`Repository`].

use git2::Repository;
use log::error;
use serde::Deserialize;
use serde::Serialize;
use std::io;
use thiserror::Error;

use crate::status::{Status, StatusError};

#[remain::sorted]
#[derive(Error, Debug)]
pub enum SubmoduleError {
    #[error(transparent)]
    FromGit2(#[from] git2::Error),
    #[error(transparent)]
    FromStatus(#[from] StatusError),
    #[error(transparent)]
    FromStdIo(#[from] io::Error),
    #[error("submodule name is invalid UTF-8")]
    SubmoduleNameInvalid,
}

type SubmoduleResult<T> = Result<T, SubmoduleError>;

/// The view of a submodule with a [`Repository`].
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SubmoduleView {
    pub name: String,
    pub status: Status,
}

impl SubmoduleView {
    /// Generate a list of [`submodule view(s)`](Self) for a given [`Repository`].
    pub fn list(repo: &Repository) -> SubmoduleResult<Vec<Self>> {
        let mut submodules = Vec::new();
        for submodule in repo.submodules()? {
            match submodule.open() {
                Ok(subrepo) => {
                    let (status, _, _) = Status::find(&subrepo)?;
                    let name = submodule
                        .name()
                        .ok_or(SubmoduleError::SubmoduleNameInvalid)?;

                    submodules.push(Self {
                        name: name.to_string(),
                        status,
                    });
                }
                Err(e) => error!("could not open submodule as repository: {e}"),
            }
        }
        Ok(submodules)
    }
}

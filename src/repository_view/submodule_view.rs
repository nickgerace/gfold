//! This module contains the ability to gather information on submodules for a given [`Repository`].

use anyhow::{anyhow, Result};
use git2::Repository;
use log::error;
use serde::Deserialize;
use serde::Serialize;

use crate::status::Status;

/// The view of a submodule with a [`Repository`].
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SubmoduleView {
    pub name: String,
    pub status: Status,
}

impl SubmoduleView {
    /// Generate a list of [`submodule view(s)`](Self) for a given [`Repository`].
    pub fn list(repo: &Repository) -> Result<Vec<Self>> {
        let mut submodules = Vec::new();
        for submodule in repo.submodules()? {
            match submodule.open() {
                Ok(subrepo) => {
                    let (status, _, _) = Status::find(&subrepo)?;
                    let name = submodule
                        .name()
                        .ok_or(anyhow!("submodule name is invalid UTF-8"))?;

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

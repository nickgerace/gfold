//! This module contains the [`crate::status::Status`] type.

use serde::{Deserialize, Serialize};

/// A summarized interpretation of the status of a Git working tree.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Status {
    Bare,
    Clean,
    Unclean,
    Unpushed,
}

impl Status {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Bare => "bare",
            Self::Clean => "clean",
            Self::Unclean => "unclean",
            Self::Unpushed => "unpushed",
        }
    }
}

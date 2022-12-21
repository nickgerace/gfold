//! This module contains the [`crate::error::Error`] type.

use std::path::PathBuf;
use thiserror::Error;

// Type aliases for external results.
pub type AnyhowResult<T> = anyhow::Result<T>;
pub type IoResult<T> = std::io::Result<T>;
pub type LibGitResult<T> = std::result::Result<T, git2::Error>;
pub type SerdeJsonResult<T> = serde_json::error::Result<T>;
pub type TomlResult<T> = std::result::Result<T, toml::ser::Error>;

// Type alias for internal errors.
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("received None (Option<&OsStr>) for file name: {0}")]
    FileNameNotFound(PathBuf),
    #[error("invalid color mode provided (run \"--help\" for options): {0}")]
    InvalidColorMode(String),
    #[error("invalid display mode provided (run \"--help\" for options): {0}")]
    InvalidDisplayMode(String),

    #[error("could not convert file name (&OsStr) to &str: {0}")]
    FileNameToStrConversionFailure(PathBuf),
    #[error("could not convert path (Path) to &str: {0}")]
    PathToStrConversionFailure(PathBuf),

    #[error("full shorthand for Git reference is invalid UTF-8")]
    GitReferenceShorthandInvalid,
    #[error("submodule name is invalid UTF-8")]
    SubmoduleNameInvalid,
    #[error("could not find home directory")]
    HomeDirNotFound,
}

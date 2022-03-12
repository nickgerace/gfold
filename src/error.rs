//! This module contains the [`crate::error::Error`] type.

use std::path::PathBuf;
use thiserror::Error;

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
    #[error("could not find home directory")]
    HomeDirNotFound,

    #[error("git2::Error")]
    Git2Rs(#[from] git2::Error),
    #[error("serde_json::Error")]
    SerdeJson(#[from] serde_json::Error),
    #[error("std::io::Error")]
    StdIo(#[from] std::io::Error),
    #[error("toml::de::Error")]
    TomlDe(#[from] toml::de::Error),
    #[error("toml::ser::Error")]
    TomlSe(#[from] toml::ser::Error),
}

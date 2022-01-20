use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("received None (Option<&OsStr>) for file name: {0}")]
    FileNameNotFound(PathBuf),
    #[error("could not convert file name (&OsStr) to &str: {0}")]
    FileNameStrConversionFailure(PathBuf),
    #[error("could not convert path (Path) to &str: {0}")]
    PathToStrConversionFailure(PathBuf),
    #[error("could not find home directory")]
    HomeDirNotFound,
    #[error("full shorthand for Git reference is invalid UTF-8")]
    GitReferenceShorthandInvalid,
}

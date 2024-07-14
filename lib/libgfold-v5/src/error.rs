use std::path::PathBuf;

#[remain::sorted]
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("received None (Option<&OsStr>) for file name: {0}")]
    FileNameNotFound(PathBuf),
    #[error("could not convert file name (&OsStr) to &str: {0}")]
    FileNameToStrConversionFailure(PathBuf),
    #[error("git2 error: {0}")]
    FromGit2(#[from] git2::Error),
    #[error("full shorthand for Git reference is invalid UTF-8")]
    GitReferenceShorthandInvalid,
    #[error("could not convert path (Path) to &str: {0}")]
    PathToStrConversionFailure(PathBuf),
    #[error("submodule name is invalid UTF-8")]
    SubmoduleNameInvalid,
}

pub type Result<T> = std::result::Result<T, Error>;

use crate::config::Config;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("could not strip newline from String: {0}")]
    StripNewLineFromStringFailure(String),
    #[error("received None (Option<&OsStr>) for file name: {0}")]
    FileNameNotFound(PathBuf),
    #[error("could not convert file name (&OsStr) to &str: {0}")]
    FileNameStrConversionFailure(PathBuf),
    #[error("could not convert path (Path) to &str: {0}")]
    PathToStrConversionFailure(PathBuf),
    #[error("found empty option in config: {0:?}")]
    EmptyConfigOption(Config),
    #[error("could not find home directory")]
    HomeDirNotFound,
}

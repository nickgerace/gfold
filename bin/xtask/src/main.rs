//! This crate is an example of the [`cargo-xtask`](https://github.com/matklad/cargo-xtask) pattern.

mod task;

use clap::{Parser, Subcommand};
use std::io;
use std::process::Output;
use strum::Display;
use thiserror::Error;

pub use task::TaskHarness;
pub use task::TaskRunner;

#[remain::sorted]
#[derive(Error, Debug)]
pub enum TaskError {
    #[error("cargo command failed")]
    CargoCommandFailed,
    #[error("could not determine repository root")]
    CouldNotDetermineRepositoryRoot,
    #[error("home directory not found")]
    HomeDirectoryNotFound,
    #[error("invalid task provided: {0}")]
    InvalidTaskProvided(String),
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error("repository does not have parent directory (this should be impossible)")]
    RepositoryDoesNotHaveParentDirectory,
    #[error("command during loose bench was not successful: {0:?}")]
    UnsuccessfulCommandDuringLooseBench(Output),
}

pub type TaskResult<T> = Result<T, TaskError>;

#[derive(Parser)]
#[command(long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Task,
}

#[remain::sorted]
#[derive(Display, Subcommand)]
#[strum(serialize_all = "kebab-case")]
pub enum Task {
    /// Scan for potential bloat
    Bloat,
    /// Build all targets
    Build,
    /// Build all targets, scan and check binary size
    BuildRelease,
    /// Run the ci suite
    Ci,
    /// Perform loose bench
    LooseBench,
    /// Run update, and baseline lints and checks
    Prepare,
    /// Scan for vulnerabilities and unused dependencies
    Scan,
    /// Get the release binary size (and compare to the installed binary size from "crates.io", if
    /// it exists)
    Size,
}

fn main() -> TaskResult<()> {
    let cli = Cli::parse();
    let mut harness = TaskHarness::new()?;
    harness.task(cli.command)?;
    Ok(())
}

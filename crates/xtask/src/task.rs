mod bloat;
mod build;
mod build_release;
mod ci;
mod loose_bench;
mod prepare;
mod scan;
mod size;

use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use termcolor::{ColorChoice, ColorSpec, StandardStream, WriteColor};

use crate::task::bloat::RunBloat;
use crate::task::build::RunBuild;
use crate::task::build_release::RunBuildRelease;
use crate::task::ci::RunCi;
use crate::task::loose_bench::RunLooseBench;
use crate::task::prepare::RunPrepare;
use crate::task::scan::RunScan;
use crate::task::size::RunSize;
use crate::{Task, TaskError, TaskResult};

const CARGO_MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");
const CRATE: &str = "gfold";
const ROOT_PREFIX: &str = "xtask";

pub struct TaskHarness {
    root: PathBuf,
    prefix: Vec<String>,
}

pub trait TaskRunner {
    fn run(harness: &mut TaskHarness) -> TaskResult<()>;
}

enum WriteKind {
    Stdout,
    Stderr,
}

impl TaskHarness {
    pub fn new() -> TaskResult<Self> {
        let root = match Path::new(CARGO_MANIFEST_DIR).ancestors().nth(2) {
            Some(found_root) => found_root.to_path_buf(),
            None => return Err(TaskError::CouldNotDetermineRepositoryRoot),
        };
        Ok(Self {
            root,
            prefix: vec![ROOT_PREFIX.to_string()],
        })
    }

    pub fn task(&mut self, task: Task) -> TaskResult<()> {
        self.prefix.push(task.to_string());

        match task {
            Task::Bloat => RunBloat::run(self)?,
            Task::Build => RunBuild::run(self)?,
            Task::BuildRelease => RunBuildRelease::run(self)?,
            Task::Ci => RunCi::run(self)?,
            Task::Prepare => RunPrepare::run(self)?,
            Task::Scan => RunScan::run(self)?,
            Task::Size => RunSize::run(self)?,
            Task::LooseBench => RunLooseBench::run(self)?,
        }

        self.prefix.pop();
        Ok(())
    }

    pub fn stdout(&self, contents: impl AsRef<str>) -> TaskResult<()> {
        self.write(contents, WriteKind::Stdout)
    }

    pub fn stderr(&self, contents: impl AsRef<str>) -> TaskResult<()> {
        self.write(contents, WriteKind::Stderr)
    }

    fn write(&self, contents: impl AsRef<str>, kind: WriteKind) -> TaskResult<()> {
        let mut buffer = match kind {
            WriteKind::Stdout => StandardStream::stdout(ColorChoice::Auto),
            WriteKind::Stderr => StandardStream::stderr(ColorChoice::Auto),
        };

        buffer.set_color(ColorSpec::new().set_bold(true))?;
        write!(&mut buffer, "{} ", self.prefix.join(":"))?;

        buffer.reset()?;
        writeln!(&mut buffer, "{}", contents.as_ref())?;
        Ok(())
    }

    fn cargo(
        &self,
        args: &'static str,
        env: Option<(&'static str, &'static str)>,
    ) -> TaskResult<()> {
        let mut cmd = Command::new("cargo");
        if let Some((key, value)) = env {
            cmd.env(key, value);
            self.stdout(format!("{key}={value} cargo {args}"))?;
        } else {
            self.stdout(format!("cargo {args}"))?;
        }
        match cmd
            .current_dir(&self.root)
            .env("CARGO_TERM_COLOR", "always")
            .args(args.trim().split(' '))
            .status()?
            .success()
        {
            true => Ok(()),
            false => Err(TaskError::CargoCommandFailed),
        }
    }
}

use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{env, fs};

use anyhow::{Result, bail};
use clap::{CommandFactory, Parser, Subcommand};
use clap_mangen::Man;

const CARGO_MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: CliCommand,
}

#[remain::sorted]
#[derive(Subcommand)]
enum CliCommand {
    /// Runs "cargo audit".
    Audit,
    /// Performs a loose benchmark against gfold in "PATH".
    Bench,
    /// Runs "cargo bloat".
    Bloat,
    /// Builds with release optimizations.
    BuildRelease,
    /// Performs checks for CI.
    Ci,
    /// Formats all code.
    Format,
    /// Generates a manpage.
    Mangen,
    /// Runs "cargo outdated".
    Outdated,
    /// Formats and prepares all code for a PR.
    Prepare,
    /// Runs with increased verbositry.
    Run,
    /// Run with the help flag.
    RunHelp,
    /// Compares the size of gfold in "PATH" to the local release build.
    Size,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let runner = TaskRunner::new()?;

    match cli.command {
        CliCommand::Audit => runner.task_audit(),
        CliCommand::Bench => runner.task_bench(),
        CliCommand::Bloat => runner.task_bloat(),
        CliCommand::BuildRelease => runner.task_build_release(),
        CliCommand::Ci => runner.task_ci(),
        CliCommand::Format => runner.task_format(),
        CliCommand::Mangen => runner.task_mangen(),
        CliCommand::Outdated => runner.task_outdated(),
        CliCommand::Prepare => runner.task_prepare(),
        CliCommand::Run => runner.task_run(false),
        CliCommand::RunHelp => runner.task_run(true),
        CliCommand::Size => runner.task_size(),
    }
}

struct TaskRunner {
    root: PathBuf,
}

impl TaskRunner {
    pub fn new() -> Result<Self> {
        let root = match Path::new(CARGO_MANIFEST_DIR).ancestors().nth(1) {
            Some(found_root) => found_root.to_path_buf(),
            None => bail!("could not determine repo root"),
        };
        Ok(Self { root })
    }

    fn exec(&self, cmd: &'static str, args: &'static str) -> Result<()> {
        self.stdout(format!("{cmd} {args}"));

        let mut cmd = Command::new(cmd);
        if !cmd
            .current_dir(&self.root)
            .args(args.trim().split(" "))
            .status()?
            .success()
        {
            bail!("command failed");
        }

        Ok(())
    }

    fn exec_bench(&self) -> Result<()> {
        let parent = match self.root.parent() {
            Some(parent) => parent,
            None => bail!("no parent directory found"),
        };
        self.stdout(format!(
            "hyperfine --warmup 5 'target/release/gfold {}' 'gfold {}'",
            parent.display(),
            parent.display()
        ));
        let mut cmd = Command::new("hyperfine");
        if !cmd
            .current_dir(&self.root)
            .arg("--warmup")
            .arg("5")
            .arg(format!("target/release/gfold {}", parent.display()))
            .arg(format!("gfold {}", parent.display()))
            .status()?
            .success()
        {
            bail!("command failed");
        }
        Ok(())
    }

    fn find_in_path(cmd: &str) -> Option<PathBuf> {
        let path_os_string = env::var_os("PATH")?;
        for path_split in env::split_paths(&path_os_string) {
            let path_to_cmd = path_split.join(cmd);
            if path_to_cmd.is_file() {
                return Some(path_to_cmd);
            }
            #[cfg(windows)]
            {
                let path_to_cmd_exe = path_split.join(format!("{cmd}.exe"));
                if path_to_cmd_exe.is_file() {
                    return Some(path_to_cmd_exe);
                }
            }
        }
        None
    }

    fn stdout(&self, contents: impl AsRef<str>) {
        let contents = contents.as_ref();
        println!("\x1b[1m{contents}\x1b[0m");
    }

    pub fn task_audit(&self) -> Result<()> {
        self.exec("cargo", "audit")?;
        Ok(())
    }

    pub fn task_bench(&self) -> Result<()> {
        self.task_build_release()?;
        self.exec_bench()?;
        Ok(())
    }

    pub fn task_bloat(&self) -> Result<()> {
        self.exec("cargo", "bloat --release")?;
        self.exec("cargo", "bloat --release --crates")?;
        Ok(())
    }

    pub fn task_build_release(&self) -> Result<()> {
        self.exec("cargo", "build --release")?;
        Ok(())
    }

    pub fn task_ci(&self) -> Result<()> {
        self.exec("cargo", "fmt --all -- --check")?;
        self.exec("cargo", "check --all-targets --all-features --workspace")?;
        self.exec(
            "cargo",
            "clippy --all-targets --all-features --no-deps --workspace -- -D warnings",
        )?;
        self.exec("cargo", "doc --all --no-deps")?;
        self.exec("cargo", "test --all-targets --workspace")?;
        self.exec("cargo", "build --locked --all-targets")?;
        Ok(())
    }

    pub fn task_format(&self) -> Result<()> {
        self.exec("taplo", "format Cargo.toml")?;
        self.exec("taplo", "format Cross.toml")?;
        self.exec("taplo", "format .cargo/config.toml")?;
        self.exec("cargo", "fmt")?;
        Ok(())
    }

    pub fn task_mangen(&self) -> Result<()> {
        let cmd = cli::Cli::command();
        let man = Man::new(cmd);

        let mut buffer = Vec::new();
        man.render(&mut buffer)?;

        let mut file = fs::File::create(Path::new(&self.root).join("target/gfold.1"))?;
        file.write_all(&buffer)?;

        Ok(())
    }

    pub fn task_outdated(&self) -> Result<()> {
        self.exec("cargo", "outdated")?;
        Ok(())
    }

    pub fn task_prepare(&self) -> Result<()> {
        self.task_format()?;
        self.exec("cargo", "update")?;
        self.exec("cargo", "check --all-targets --all-features --workspace")?;
        self.exec(
            "cargo",
            "fix --edition-idioms --allow-dirty --allow-staged --workspace",
        )?;
        self.exec(
            "cargo",
    "clippy --all-features --all-targets --workspace --no-deps --fix --allow-dirty --allow-staged"
        )?;
        Ok(())
    }

    pub fn task_run(&self, help: bool) -> Result<()> {
        if help {
            self.exec("cargo", "run -- -h")?;
        } else {
            self.exec("cargo", "run -- -vvv ..")?;
        }
        Ok(())
    }

    pub fn task_size(&self) -> Result<()> {
        self.task_build_release()?;

        let installed = if let Some(installed) = Self::find_in_path("gfold") {
            installed
        } else {
            bail!("could not find gfold in PATH");
        };
        let local = self.root.join("target").join("release").join("gfold");

        let installed_metadata = std::fs::metadata(&installed)?;
        let installed_size_mib = installed_metadata.len() as f64 / 1_048_576.0;
        let local_metadata = std::fs::metadata(&local)?;
        let local_size_mib = local_metadata.len() as f64 / 1_048_576.0;

        println!("{}: {:.2} MiB", installed.display(), installed_size_mib);
        println!("{}: {:.2} MiB", local.display(), local_size_mib);

        Ok(())
    }
}

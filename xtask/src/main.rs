use anyhow::{anyhow, Result};
use std::{
    env,
    fs::File,
    path::{Path, PathBuf},
    process::Command,
};

const NAME: &str = "gfold";
const HELP_TEXT: &str = "\
> release
> build
> prepare
> scan
> bloat
> ci
> compare";

fn main() -> Result<()> {
    match env::args().nth(1).as_ref().map(|arg| arg.as_str()) {
        Some("release") => release(),
        Some("build") => build(),
        Some("prepare") => prepare(),
        Some("scan") => scan(),
        Some("bloat") => bloat(),
        Some("ci") => ci(),
        Some("compare") => compare(),
        Some(_) => help(true),
        None => help(false),
    }
}

fn help(invalid: bool) -> Result<()> {
    match invalid {
        true => {
            eprintln!("{}", HELP_TEXT);
            Err(anyhow!("[xtask] invalid xtask provided"))
        }
        false => {
            println!("{}", HELP_TEXT);
            Ok(())
        }
    }
}

fn release() -> Result<()> {
    prepare()?;
    scan()?;
    cargo(&["build", "--release"])?;
    println!("[xtask] binary size: {}", size()?);
    Ok(())
}

fn build() -> Result<()> {
    prepare()?;
    cargo(&["build"])
}

fn prepare() -> Result<()> {
    cargo(&["update"])?;
    cargo(&["fix", "--edition-idioms", "--allow-dirty", "--allow-staged"])?;
    cargo(&["+nightly", "fmt"])?;
    cargo(&["clippy", "--all-features", "--all-targets"])
}

fn scan() -> Result<()> {
    cargo(&["+nightly", "udeps"])?;
    cargo(&["audit"])
}

fn bloat() -> Result<()> {
    cargo(&["bloat", "--release"])?;
    cargo(&["bloat", "--release", "--crates"])
}

fn ci() -> Result<()> {
    cargo(&["+nightly", "fmt", "--all", "--", "--checks"])?;
    cargo(&["clippy", "--", "-D", "warnings"])?;
    cargo(&["test", "--", "--nocapture"])?;
    cargo(&["clippy", "--", "-D", "warnings"])
}

fn compare() -> Result<()> {
    cargo(&["build", "--release"])?;
    println!("[xtask] new size:     {}", size()?);
    println!("[xtask] current size: {}", current_size()?);
    Ok(())
}

fn cargo(args: &[&str]) -> Result<()> {
    println!("[xtask] cargo {}", &args.join(" "));
    let mut cmd = Command::new("cargo");
    match cmd.current_dir(root()?).args(args).status()?.success() {
        true => Ok(()),
        false => Err(anyhow!("[xtask] command failed")),
    }
}

fn root() -> Result<PathBuf> {
    match Path::new(&env!("CARGO_MANIFEST_DIR")).ancestors().nth(1) {
        Some(s) => Ok(s.to_path_buf()),
        None => Err(anyhow!("[xtask] could not determine repository root")),
    }
}

fn size() -> Result<u64> {
    Ok(
        File::open(root()?.join("target").join("release").join(NAME))?
            .metadata()?
            .len(),
    )
}

fn current_size() -> Result<u64> {
    Ok(File::open(
        Path::new(&env::var("HOME")?)
            .join(".cargo")
            .join("bin")
            .join(NAME),
    )?
    .metadata()?
    .len())
}

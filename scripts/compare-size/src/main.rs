use std::fs;
use std::fs::Metadata;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;
use anyhow::Result;
use anyhow::anyhow;

fn main() -> Result<()>{
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let repo = manifest_dir
        .parent()
        .ok_or_else(|| anyhow!("could not get parent"))?
        .parent()
        .ok_or_else(|| anyhow!("could not get parent"))?;

    println!("Running \"cargo build --release\"...");
    let output = Command::new("cargo")
        .arg("build")
        .arg("--release")
        .current_dir(repo)
        .output()?;
    if !output.status.success() {
        return Err(anyhow!("command failed: \"cargo build --release\""));
    }

    let binary = repo.join("target").join("release").join("gfold");
    let metadata = fs::metadata(&binary)?;
    print_binary_size(&metadata, &binary)?;

    // Check if the binary is currently installed to compare release build sizes. If it is not,
    // we can skip this check.
    let home = dirs::home_dir().ok_or_else(|| anyhow!("could not find home dir"))?;
    let installed = home.join(".cargo").join("bin").join("gfold");
    match fs::metadata(&installed) {
        Ok(metadata) => {
            print_binary_size(&metadata, &installed)?;
            Ok(())
        }
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(()),
        Err(e) => Err(e.into())
    }
}

fn print_binary_size(metadata: &Metadata, path: &PathBuf) -> Result<()> {
    // Divisor used to perform human readable size conversion.
    // 1048576.0 = 1024.0 * 1024.0
    println!(
        "{:.3} MB - {}",
        metadata.len() as f64 / 1048576.0,
        path.to_str().ok_or_else(|| anyhow!("could not get parent"))?
    );
    Ok(())
}

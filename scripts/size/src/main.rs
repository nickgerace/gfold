use std::fs;
use std::fs::Metadata;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let repo = manifest_dir
        .parent()
        .expect("could not get parent")
        .parent()
        .expect("could not get parent");

    println!("Running \"cargo build --release\"...");
    let output = Command::new("cargo")
        .arg("build")
        .arg("--release")
        .output()
        .expect("could not execute command");
    if !output.status.success() {
        panic!("command failed: \"cargo build --release\"");
    }

    let binary = repo.join("target").join("release").join("gfold");
    let metadata = fs::metadata(&binary).expect("could not get metadata");
    print_binary_size(&metadata, &binary);

    // Check if the binary is currently installed to compare release build sizes. If it is not,
    // we can skip this check.
    let home = dirs::home_dir().expect("could not find home directory");
    let installed = home.join(".cargo").join("bin").join("gfold");
    match fs::metadata(&installed) {
        Ok(metadata) => {
            print_binary_size(&metadata, &installed);
        }
        Err(e) if e.kind() == io::ErrorKind::NotFound => {}
        Err(e) => panic!(
            "encountered error when trying to get metadata for file: {}",
            e.to_string()
        ),
    }
}

fn print_binary_size(metadata: &Metadata, path: &PathBuf) {
    // Divisor used to perform human readable size conversion.
    // 1048576.0 = 1024.0 * 1024.0
    println!(
        "{:.3} MB - {}",
        metadata.len() as f64 / 1048576.0,
        path.to_str().expect("could not convert to str")
    );
}

use std::path::Path;
use std::process::Command;
use std::time::{Duration, Instant};

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
    let home = dirs::home_dir().expect("could not find home directory");
    let installed = home.join(".cargo").join("bin").join("gfold");
    let runs = 4;

    // Loosely bench using the home directory as the target.
    for run in 0..runs {
        println!(
            "group {} of {} in {}",
            run + 1,
            runs,
            home.to_str().expect("could not convert to str")
        );
        loose_bench(&binary, &installed, &home);
    }

    // Loosely bench with the parent directory of the repository as the target.
    let parent_of_repo = repo.parent().expect("could not get parent");
    for run in 0..runs {
        println!(
            "group {} of {} in {}",
            run + 1,
            runs,
            parent_of_repo.to_str().expect("could not convert to str")
        );
        loose_bench(&binary, &installed, parent_of_repo);
    }
}

fn loose_bench(new: &Path, old: &Path, target: &Path) {
    let new_duration = execute(new, target);
    let old_duration = execute(old, target);
    let (new_text, old_text) = match new_duration {
        new_duration if new_duration > old_duration => ("LOST", "WON "),
        new_duration if new_duration < old_duration => ("WON ", "LOST"),
        _ => ("TIE ", "TIE "),
    };

    println!(
        "  {} @ {:?} - {}",
        new_text,
        new_duration,
        new.to_str().expect("could not convert to str"),
    );
    println!(
        "  {} @ {:?} - {}",
        old_text,
        old_duration,
        old.to_str().expect("could not convert to str"),
    );
}

fn execute(binary: &Path, target: &Path) -> Duration {
    let start = Instant::now();
    let output = Command::new(binary)
        .arg("-i")
        .arg(target)
        .output()
        .expect("could not execute command");
    let duration = start.elapsed();

    // Check for failure _after_ the bench finishes.
    if !output.status.success() {
        panic!("bench failed");
    }

    duration
}

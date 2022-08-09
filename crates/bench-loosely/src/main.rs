//! This is the loose bench for gfold. Why use a loose bench over a precise one? Most of gfold
//! development happens with a mobile processor, which is prone to performance fluctuations over
//! background I/O operations, battery life and temperature. Moreover, the "real world" use case of
//! gfold is via CLI and not a library. Thus, this loose bench executes gfold as a CLI similarly
//! to real world use. This benchmark is not precise and is designed to give a high level overview.

// TODO: add consistency deviation. We should see how far each result strays from the average. We
// want gfold to perform consistently as well as quickly.

use std::path::Path;
use std::process::Command;
use std::time::{Duration, Instant};

fn main() {
    let global_instant = Instant::now();

    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let repo = manifest_dir
        .parent()
        .expect("could not get parent")
        .parent()
        .expect("could not get parent");

    println!("running in {:?}: cargo build --release", &repo);
    let output = Command::new("cargo")
        .arg("build")
        .arg("--release")
        .current_dir(repo)
        .output()
        .expect("could not execute command");
    if !output.status.success() {
        panic!("command failed: \"cargo build --release\"");
    }

    let binary = repo.join("target").join("release").join("gfold");
    let home_path = dirs::home_dir().expect("could not find home directory");
    let installed = home_path.join(".cargo").join("bin").join("gfold");
    let home = home_path.to_str().expect("could not convert to str");

    // Add "1" to total runs to ensure caching does not skew results. We need one
    // "warm up" run.
    let runs = 41;

    // Loosely bench using the home directory as the target.
    let mut home_new_durations = Vec::new();
    let mut home_old_durations = Vec::new();
    let mut first = true;
    for run in 0..runs {
        println!("group {} of {} in {}", run + 1, runs, home);

        // Alternate at the halfway point. Though, it is doubtful that this would actually change anything.
        // Only calculate the average after the first run to avoid caching skewing results.
        if run > runs / 2 {
            let (old_duration, new_duration) = loose_bench(&installed, &binary, &home_path);
            if first {
                first = false;
            } else {
                home_new_durations.push(new_duration);
                home_old_durations.push(old_duration);
            }
        } else {
            let (new_duration, old_duration) = loose_bench(&binary, &installed, &home_path);
            if first {
                first = false;
            } else {
                home_new_durations.push(new_duration);
                home_old_durations.push(old_duration);
            }
        }
    }

    // Loosely bench with the parent directory of the repository as the target.
    let parent_of_repo = repo.parent().expect("could not get parent");
    let parent = parent_of_repo.to_str().expect("could not convert to str");
    let mut parent_new_durations = Vec::new();
    let mut parent_old_durations = Vec::new();
    let mut first = true;
    for run in 0..runs {
        println!("group {} of {} in {}", run + 1, runs, parent);

        // Alternate at the halfway point. Though, it is doubtful that this would actually change anything.
        // Only calculate the average after the first run to avoid caching skewing results.
        if run > runs / 2 {
            let (old_duration, new_duration) = loose_bench(&installed, &binary, parent_of_repo);
            if first {
                first = false;
            } else {
                parent_new_durations.push(new_duration);
                parent_old_durations.push(old_duration);
            }
        } else {
            let (new_duration, old_duration) = loose_bench(&binary, &installed, parent_of_repo);
            if first {
                first = false;
            } else {
                parent_new_durations.push(new_duration);
                parent_old_durations.push(old_duration);
            }
        }
    }

    // Print the averages. Start with a multi-platform empty newline print.
    println!();
    println!(
        "Average: {:?} - {} - {}",
        average_duration(&home_new_durations),
        home,
        &binary.display(),
    );
    println!(
        "Average: {:?} - {} - {}",
        average_duration(&home_old_durations),
        home,
        &installed.display(),
    );
    println!(
        "Average: {:?} - {} - {}",
        average_duration(&parent_new_durations),
        parent,
        &binary.display(),
    );
    println!(
        "Average: {:?} - {} - {}",
        average_duration(&parent_old_durations),
        parent,
        &installed.display(),
    );

    // Display the global duration for the entire script. Start with a multi-platform empty newline
    // print.
    println!();
    println!("Total duration: {:?}", global_instant.elapsed());
}

fn loose_bench(first: &Path, second: &Path, target: &Path) -> (Duration, Duration) {
    let first_duration = execute(first, target);
    let second_duration = execute(second, target);
    let (first_text, second_text) = match first_duration {
        first_duration if first_duration > second_duration => ("LOST", "WON "),
        first_duration if first_duration < second_duration => ("WON ", "LOST"),
        _ => ("TIE ", "TIE "),
    };

    println!(
        "  {} @ {:?} - {}",
        first_text,
        first_duration,
        first.to_str().expect("could not convert to str"),
    );
    println!(
        "  {} @ {:?} - {}",
        second_text,
        second_duration,
        second.to_str().expect("could not convert to str"),
    );
    (first_duration, second_duration)
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

fn average_duration(durations: &[Duration]) -> Duration {
    let sum = durations.iter().sum::<Duration>();
    let count = durations.len() as f32;
    sum.div_f32(count)
}

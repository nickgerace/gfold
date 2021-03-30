//! This is the minimal version of `gfold`, a CLI tool to help keep track of your Git repositories.

use std::error::Error;
use std::path::{Path, PathBuf};

use git2::{ErrorClass, ErrorCode, Repository, StatusOptions};
use log::debug;
use walkdir::WalkDir;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Outcome {
    name: String,
    status: String,
    branch: String,
    source: String,
}

/// This function is the primary driver for `gfld`.
pub fn run(path: &Path) -> Result<(), Box<dyn Error>> {
    let mut vec = Vec::new();
    let shorthand = PathBuf::from(path.file_name().ok_or("Path terminates in '..'")?);
    let mut max_name: usize = 0;
    let mut max_status: usize = 0;
    let mut max_branch: usize = 0;
    let mut max_source: usize = 0;

    // Ensure that the directory is neither hidden nor is its parent a git repository.
    for item in WalkDir::new(path)
        .into_iter()
        .filter_entry(|e| {
            !e.file_name()
                .to_str()
                .map(|s| s.starts_with('.'))
                .unwrap_or(false)
                && match e.path().parent() {
                    Some(s) => Repository::open(s).is_err(),
                    None => false,
                }
        })
        .filter_map(|e| e.ok())
    {
        let entry = item.into_path();
        let path = path.to_path_buf();
        if let Ok(repository) = Repository::open(&entry) {
            let origin = match repository.find_remote("origin") {
                Ok(o) => o,
                Err(e) => {
                    debug!("{}", e);
                    continue;
                }
            };
            let source = origin.url().unwrap_or("none");

            let head = match repository.head() {
                Ok(o) => o,
                Err(e) => {
                    debug!("{}", e);
                    continue;
                }
            };
            let branch = head.shorthand().unwrap_or("none");

            let mut opts = StatusOptions::new();
            let status = match repository.statuses(Some(&mut opts)) {
                Ok(statuses) if statuses.is_empty() => "clean",
                Ok(_) => "unclean",
                Err(error)
                    if error.code() == ErrorCode::BareRepo
                        && error.class() == ErrorClass::Repository =>
                {
                    "bare"
                }
                Err(_) => "error",
            };

            let child = match entry.strip_prefix(&path) {
                Ok(o) => o,
                Err(e) => {
                    debug!("{}", e);
                    continue;
                }
            };
            let name = shorthand.join(&child);
            let name = name.to_str().unwrap_or("none");

            if name.len() > max_name {
                max_name = name.len();
            }
            if status.len() > max_status {
                max_status = status.len();
            }
            if branch.len() > max_branch {
                max_branch = branch.len();
            }
            if source.len() > max_source {
                max_source = source.len();
            }

            vec.push(Outcome {
                name: name.to_string(),
                status: status.to_string(),
                branch: branch.to_string(),
                source: source.to_string(),
            });
        }
    }

    // Imperceptible time savings without this sort.
    vec.sort_unstable();

    // Need to insert after the sort is finished.
    vec.insert(
        0,
        Outcome {
            name: String::from("NAME"),
            status: String::from("STATUS"),
            branch: String::from("BRANCH"),
            source: String::from("SOURCE"),
        },
    );

    // Despite using "println!" N times, this loop has minimal impact on runtime performance.
    for outcome in vec {
        println!(
            "{:<width_name$}  {:<width_status$}  {:<width_branch$}  {:<width_source$}",
            outcome.name,
            outcome.status,
            outcome.branch,
            outcome.source,
            width_name = max_name,
            width_status = max_status,
            width_branch = max_branch,
            width_source = max_source
        );
    }
    Ok(())
}

//! This is the minimal version of `gfold`, a CLI tool to help keep track of your Git repositories.

use std::error::Error;
use std::ffi::OsStr;
use std::path::Path;

use git2::{ErrorClass, ErrorCode, Repository, StatusOptions};
use log::debug;
use walkdir::WalkDir;

const UNKNOWN: &str = "unknown";

#[derive(Eq, PartialEq, Ord, PartialOrd)]
struct Outcome {
    name: String,
    status: String,
    branch: String,
    source: String,
}

/// This function is the primary driver for `gfld`.
pub fn run(path: &Path) -> Result<(), Box<dyn Error>> {
    let mut vec = Vec::new();
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

            let name = match entry.strip_prefix(&path) {
                Ok(o) => match o.to_str() {
                    Some(s) if s.is_empty() => path
                        .file_name()
                        .unwrap_or_else(|| OsStr::new(UNKNOWN))
                        .to_str()
                        .unwrap_or(UNKNOWN),
                    Some(s) => s,
                    None => "none",
                },
                Err(e) => {
                    debug!("{}", e);
                    UNKNOWN
                }
            };

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

    // Imperceptible time savings without either of these sorts. We want to sort by the first
    // field in "Outcome", and then by the status. Our first sort can be unstable, but our second
    // has to be stable to retain the original order.
    vec.sort_unstable_by_key(|k| k.name.clone());
    vec.sort_by_key(|k| k.status.clone());

    // Need to insert header after the sort is finished.
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
    for outcome in &vec {
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

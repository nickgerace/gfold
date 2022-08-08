//! This module contains target generation logic for eventually generating reports.

use log::{debug, error, warn};
use rayon::prelude::*;
use std::fs::DirEntry;
use std::path::PathBuf;
use std::{fs, io};

enum TargetOption {
    Multiple(Vec<PathBuf>),
    Single(PathBuf),
    None,
}

/// Ensure the entry is a directory and is not hidden. Then, check if a Git sub directory exists,
/// which will indicate if the entry is a repository. Finally, generate targets based on that
/// repository.
fn process_entry(entry: &DirEntry) -> io::Result<TargetOption> {
    match entry.file_type()?.is_dir()
        && !entry
            .file_name()
            .to_str()
            .map(|file_name| file_name.starts_with('.'))
            .unwrap_or(false)
    {
        true => {
            let path = entry.path();
            let git_sub_directory = path.join(".git");
            match git_sub_directory.exists() && git_sub_directory.is_dir() {
                true => {
                    debug!("found target: {:?}", &path);
                    Ok(TargetOption::Single(path))
                }
                false => Ok(TargetOption::Multiple(generate_targets(path)?)),
            }
        }
        false => Ok(TargetOption::None),
    }
}

/// Generate targets from a given [`PathBuf`] based on its children (recursively).
/// We use recursion paired with [`rayon`] since we prioritize speed over memory use.
pub fn generate_targets(path: PathBuf) -> io::Result<Vec<PathBuf>> {
    let entries: Vec<DirEntry> = match fs::read_dir(&path) {
        Ok(o) => o.filter_map(|r| r.ok()).collect(),
        Err(e) if e.kind() == io::ErrorKind::PermissionDenied => {
            warn!("{}: {}", e, &path.display());
            return Ok(vec![]);
        }
        Err(e) => {
            error!("{}: {}", e, &path.display());
            return Ok(vec![]);
        }
    };

    let processed = entries
        .par_iter()
        .map(process_entry)
        .collect::<Vec<io::Result<TargetOption>>>();

    let mut results = Vec::new();
    for entry in processed {
        let entry = entry?;
        if let TargetOption::Multiple(targets) = entry {
            results.extend(targets);
        } else if let TargetOption::Single(target) = entry {
            results.push(target);
        }
    }
    Ok(results)
}

//! This module contains target generation logic for eventually generating reports.

use log::{error, warn};
use rayon::prelude::*;
use std::fs::DirEntry;
use std::path::{Path, PathBuf};
use std::{fs, io};

/// This type represents bundled target Git directories that were generated from a given [`DirEntry`].
type Targets = io::Result<Option<Vec<PathBuf>>>;

/// Ensure the entry is a directory and is not hidden. Then, check if a Git sub directory exists,
/// which will indicate if the entry is a repository. Finally, generate targets based on that
/// repository.
fn process_entry(entry: &DirEntry) -> Targets {
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
                true => Ok(Some(vec![path])),
                false => Ok(Some(recursive_target_gen(&path)?)),
            }
        }
        false => Ok(None),
    }
}

/// Recursive function for generating targets in a child directory.
pub fn recursive_target_gen(path: &Path) -> io::Result<Vec<PathBuf>> {
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

    let targets = entries
        .par_iter()
        .map(process_entry)
        .collect::<Vec<Targets>>();

    let mut results = vec![];
    for target in targets {
        match target {
            Ok(v) => {
                if let Some(mut v) = v {
                    if !v.is_empty() {
                        results.append(&mut v);
                    }
                }
            }
            Err(e) => return Err(e),
        }
    }
    Ok(results)
}

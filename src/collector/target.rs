//! This module contains target generation logic required for generating
//! [`RepositoryViews`](crate::repository_view::RepositoryView).

use log::{debug, error, warn};
use rayon::prelude::*;
use std::fs::DirEntry;
use std::path::PathBuf;
use std::{fs, io};

/// An unprocessed target that needs to be disassembled before consumption.
type UnprocessedTarget = io::Result<MaybeTarget>;

/// A unit struct used to centralizing target collection method(s).
pub(crate) struct TargetCollector;

impl TargetCollector {
    /// Generate targets for a given [`PathBuf`] based on its children (recursively). We use
    /// recursion paired with [`rayon`] since we prioritize speed over memory use.
    pub(crate) fn run(path: PathBuf) -> io::Result<Vec<PathBuf>> {
        let entries: Vec<DirEntry> = match fs::read_dir(&path) {
            Ok(read_dir) => read_dir.filter_map(|r| r.ok()).collect(),
            Err(e) => {
                match e.kind() {
                    io::ErrorKind::PermissionDenied => warn!("{}: {}", e, &path.display()),
                    _ => error!("{}: {}", e, &path.display()),
                }
                return Ok(Vec::with_capacity(0));
            }
        };

        let unprocessed = entries
            .par_iter()
            .map(Self::determine_target)
            .collect::<Vec<UnprocessedTarget>>();

        let mut results = Vec::new();
        for entry in unprocessed {
            let entry = entry?;
            if let MaybeTarget::Multiple(targets) = entry {
                results.extend(targets);
            } else if let MaybeTarget::Single(target) = entry {
                results.push(target);
            }
        }
        Ok(results)
    }

    /// Ensure the entry is a directory and is not hidden. Then, check if a ".git" sub directory
    /// exists, which will indicate if the entry is a repository. If the directory is not a Git
    /// repository, then we will recursively call [`Self::run()`].
    fn determine_target(entry: &DirEntry) -> io::Result<MaybeTarget> {
        if entry.file_type()?.is_dir()
            && !entry
                .file_name()
                .to_str()
                .is_some_and(|file_name| file_name.starts_with('.'))
        {
            let path = entry.path();
            let git_sub_item = path.join(".git");
            if git_sub_item.exists() {
                if git_sub_item.is_dir() {
                    debug!("found target: {:?}", &path.display());
                    return Ok(MaybeTarget::Single(path));
                } else if git_sub_item.is_file() {
                    debug!("found a worktree: {:?}", &path.display());
                    return Ok(MaybeTarget::Single(path));
                }
            }
            Ok(MaybeTarget::Multiple(Self::run(path)?))
        } else {
            Ok(MaybeTarget::None)
        }
    }
}

/// An enum that contains 0 to N targets based on the variant.
#[remain::sorted]
enum MaybeTarget {
    /// Contains multiple targets from recursive call(s) of [`TargetCollector::run()`].
    Multiple(Vec<PathBuf>),
    /// Does not contain a target.
    None,
    /// Contains a single target.
    Single(PathBuf),
}

use std::fs::DirEntry;
use std::path::PathBuf;
use std::{fs, io};

use log::{debug, trace, warn};
use rayon::prelude::*;

type UnprocessedTarget = io::Result<MaybeTarget>;

pub(crate) fn collect_targets(
    path: PathBuf,
    sequential: bool,
    sort: bool,
) -> io::Result<Vec<PathBuf>> {
    collect_targets_inner(path, sequential, sort, true)
}

fn collect_targets_inner(
    path: PathBuf,
    sequential: bool,
    sort: bool,
    is_root: bool,
) -> io::Result<Vec<PathBuf>> {
    trace!("scanning directory {}", path.display());
    let entries: Vec<DirEntry> = match fs::read_dir(&path) {
        Ok(read_dir) => read_dir.filter_map(Result::ok).collect(),
        Err(err) if is_root => return Err(err),
        Err(err) => {
            warn!("failed to scan directory {}: {err}", path.display());
            return Ok(Vec::new());
        }
    };

    let unprocessed = if sequential {
        entries
            .iter()
            .map(|entry| determine_target(entry, sequential))
            .collect::<Vec<UnprocessedTarget>>()
    } else {
        entries
            .par_iter()
            .map(|entry| determine_target(entry, sequential))
            .collect::<Vec<UnprocessedTarget>>()
    };

    let mut results = Vec::new();
    for entry in unprocessed {
        let entry = entry?;
        if let MaybeTarget::Multiple(targets) = entry {
            results.extend(targets);
        } else if let MaybeTarget::Single(target) = entry {
            results.push(target);
        }
    }

    if sort {
        results.sort();
    }

    Ok(results)
}

fn determine_target(entry: &DirEntry, sequential: bool) -> io::Result<MaybeTarget> {
    if entry.file_type()?.is_dir()
        && !entry
            .file_name()
            .to_str()
            .is_some_and(|file_name| file_name.starts_with('.'))
    {
        let path = entry.path();
        let jj_sub_item = path.join(".jj");
        if jj_sub_item.exists() {
            let check = jj_sub_item.join("repo");
            if check.is_dir() || check.is_file() {
                debug!("found jj workspace at {}", path.display());
                return Ok(MaybeTarget::Single(path));
            }
        }
        Ok(MaybeTarget::Multiple(collect_targets_inner(
            path, sequential, false, false,
        )?))
    } else {
        Ok(MaybeTarget::None)
    }
}

enum MaybeTarget {
    Multiple(Vec<PathBuf>),
    None,
    Single(PathBuf),
}

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};

    use super::collect_targets;

    fn create_workspace(path: &Path) -> anyhow::Result<()> {
        let jj_dir = path.join(".jj");
        std::fs::create_dir_all(&jj_dir)?;
        std::fs::create_dir(jj_dir.join("repo"))?;
        Ok(())
    }

    fn create_colocated_workspace(path: &Path) -> anyhow::Result<()> {
        let jj_dir = path.join(".jj");
        std::fs::create_dir_all(&jj_dir)?;
        std::fs::write(jj_dir.join("repo"), "../repo")?;
        Ok(())
    }

    #[test]
    fn collects_nested_workspaces() -> anyhow::Result<()> {
        let temp_dir = tempfile::tempdir()?;
        let first = temp_dir.path().join("first");
        let nested = temp_dir.path().join("group").join("nested");
        std::fs::create_dir_all(&first)?;
        std::fs::create_dir_all(&nested)?;
        create_workspace(&first)?;
        create_workspace(&nested)?;

        assert_eq!(
            collect_targets(temp_dir.path().to_path_buf(), true, true)?,
            vec![first, nested]
        );

        Ok(())
    }

    #[test]
    fn hidden_directories_are_skipped() -> anyhow::Result<()> {
        let temp_dir = tempfile::tempdir()?;
        let hidden = temp_dir.path().join(".hidden");
        std::fs::create_dir(&hidden)?;
        create_workspace(&hidden)?;

        assert!(collect_targets(temp_dir.path().to_path_buf(), true, true)?.is_empty());

        Ok(())
    }

    #[test]
    fn sequential_and_parallel_collect_same_targets() -> anyhow::Result<()> {
        let temp_dir = tempfile::tempdir()?;
        let first = temp_dir.path().join("first");
        let second = temp_dir.path().join("second");
        std::fs::create_dir(&first)?;
        std::fs::create_dir(&second)?;
        create_workspace(&first)?;
        create_workspace(&second)?;

        assert_eq!(
            collect_targets(temp_dir.path().to_path_buf(), true, true)?,
            collect_targets(temp_dir.path().to_path_buf(), false, true)?
        );

        Ok(())
    }

    #[test]
    fn missing_root_returns_error() {
        let error = collect_targets(PathBuf::from("missing"), true, true).unwrap_err();

        assert_eq!(error.kind(), std::io::ErrorKind::NotFound);
    }

    #[test]
    fn unreadable_subtree_is_skipped() -> anyhow::Result<()> {
        let temp_dir = tempfile::tempdir()?;
        let target = temp_dir.path().join("target");
        let unreadable = temp_dir.path().join("unreadable");
        std::fs::create_dir(&target)?;
        std::fs::create_dir(&unreadable)?;
        create_workspace(&target)?;

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt as _;

            let original_permissions = std::fs::metadata(&unreadable)?.permissions();
            let mut unreadable_permissions = original_permissions.clone();
            unreadable_permissions.set_mode(0o000);
            std::fs::set_permissions(&unreadable, unreadable_permissions)?;

            let result = collect_targets(temp_dir.path().to_path_buf(), true, true);

            std::fs::set_permissions(&unreadable, original_permissions)?;

            assert_eq!(result?, vec![target]);
        }

        Ok(())
    }

    #[test]
    fn targets_are_sorted() -> anyhow::Result<()> {
        let temp_dir = tempfile::tempdir()?;
        let a = temp_dir.path().join("a");
        let b = temp_dir.path().join("b");
        std::fs::create_dir(&b)?;
        std::fs::create_dir(&a)?;
        create_workspace(&b)?;
        create_workspace(&a)?;

        assert_eq!(
            collect_targets(temp_dir.path().to_path_buf(), true, true)?,
            vec![a, b]
        );

        Ok(())
    }

    #[test]
    fn collects_workspaces_with_repo_file() -> anyhow::Result<()> {
        let temp_dir = tempfile::tempdir()?;
        let colocated = temp_dir.path().join("colocated");
        std::fs::create_dir(&colocated)?;
        create_colocated_workspace(&colocated)?;

        assert_eq!(
            collect_targets(temp_dir.path().to_path_buf(), true, true)?,
            vec![colocated]
        );

        Ok(())
    }
}

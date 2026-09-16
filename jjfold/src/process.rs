use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use jj_lib::config::StackedConfig;
use jj_lib::gitignore::GitIgnoreFile;
use jj_lib::matchers::{EverythingMatcher, NothingMatcher};
use jj_lib::repo::{Repo as _, StoreFactories};
use jj_lib::settings::UserSettings;
use jj_lib::working_copy::SnapshotOptions;
use jj_lib::workspace::{
    DefaultWorkspaceLoaderFactory, WorkspaceLoaderFactory as _, default_working_copy_factories,
};
use log::{debug, info, trace};
use pollster::FutureExt as _;

pub(crate) fn process_targets(targets: Vec<PathBuf>) -> Result<()> {
    for path in targets {
        info!("checking {}", path.display());
        let is_empty = is_working_copy_empty(&path)?;
        if !is_empty {
            debug!("working copy is not empty: {}", path.display());
            println!("{}", path.display());
        } else {
            debug!("working copy is empty: {}", path.display());
        }
    }
    Ok(())
}

pub fn is_working_copy_empty(path: impl AsRef<Path>) -> Result<bool> {
    trace!("loading jj workspace from {}", path.as_ref().display());
    let workspace_root = find_workspace_dir(path.as_ref())
        .with_context(|| format!("no jj workspace found above {}", path.as_ref().display()))?;
    let settings = UserSettings::from_config(StackedConfig::with_defaults())?;
    let loader = DefaultWorkspaceLoaderFactory.create(&workspace_root)?;
    let mut workspace = loader.load(
        &settings,
        &StoreFactories::default(),
        &default_working_copy_factories(),
    )?;

    let wc_operation = workspace
        .repo_loader()
        .load_operation(workspace.working_copy().operation_id())
        .block_on()?;
    let repo = workspace.repo_loader().load_at(&wc_operation).block_on()?;
    let workspace_name = workspace.workspace_name().to_owned();
    let wc_commit_id = repo
        .view()
        .get_wc_commit_id(&workspace_name)
        .with_context(|| format!("workspace {workspace_name:?} has no working-copy commit"))?;
    let wc_commit = repo.store().get_commit(wc_commit_id)?;

    let mut locked_ws = workspace.start_working_copy_mutation()?;
    trace!("snapshotting working copy at {}", workspace_root.display());
    let (tree, stats) = locked_ws
        .locked_wc()
        .snapshot(&SnapshotOptions {
            base_ignores: GitIgnoreFile::empty(),
            progress: None,
            start_tracking_matcher: &EverythingMatcher,
            force_tracking_matcher: &NothingMatcher,
            max_new_file_size: 1 << 20,
        })
        .block_on()?;

    let parent_tree = wc_commit.parent_tree(repo.as_ref()).block_on()?;
    Ok(tree.tree_ids() == parent_tree.tree_ids() && stats.untracked_paths.is_empty())
}

fn find_workspace_dir(path: &Path) -> Option<PathBuf> {
    let start = if path.is_dir() {
        path
    } else {
        path.parent().unwrap_or(path)
    };
    start
        .ancestors()
        .find(|path| path.join(".jj").is_dir())
        .map(Path::to_path_buf)
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use anyhow::Result;
    use jj_lib::backend::{CopyId, TreeValue};
    use jj_lib::config::StackedConfig;
    use jj_lib::merge::Merge;
    use jj_lib::merged_tree_builder::MergedTreeBuilder;
    use jj_lib::repo::Repo as _;
    use jj_lib::repo_path::RepoPathBuf;
    use jj_lib::settings::UserSettings;
    use jj_lib::workspace::Workspace;
    use pollster::FutureExt as _;
    use tempfile::TempDir;

    use super::is_working_copy_empty;

    fn init_workspace() -> Result<(TempDir, Workspace)> {
        let temp_dir = tempfile::tempdir()?;
        let settings = UserSettings::from_config(StackedConfig::with_defaults())?;
        let (workspace, _) = Workspace::init_simple(&settings, temp_dir.path()).block_on()?;
        Ok((temp_dir, workspace))
    }

    fn checkout_file(workspace: &mut Workspace, path: &str, contents: &str) -> Result<()> {
        let repo = workspace.repo_loader().load_at_head().block_on()?;
        let repo_path = RepoPathBuf::from_internal_string(path)?;
        let mut reader = Cursor::new(contents.as_bytes());
        let file_id = repo
            .store()
            .write_file(&repo_path, &mut reader)
            .block_on()?;
        let mut tree_builder = MergedTreeBuilder::new(repo.store().empty_merged_tree());
        tree_builder.set_or_remove(
            repo_path,
            Merge::normal(TreeValue::File {
                id: file_id,
                executable: false,
                copy_id: CopyId::placeholder(),
            }),
        );
        let tree = tree_builder.write_tree().block_on()?;
        let mut tx = repo.start_transaction();
        let base_commit = tx
            .repo_mut()
            .new_commit(vec![repo.store().root_commit_id().clone()], tree)
            .write()
            .block_on()?;
        let repo = tx.commit("test base commit").block_on()?;

        let mut tx = repo.start_transaction();
        let workspace_name = workspace.workspace_name().to_owned();
        let wc_commit = tx
            .repo_mut()
            .check_out(workspace_name, &base_commit)
            .block_on()?;
        tx.repo_mut().rebase_descendants().block_on()?;
        let repo = tx.commit("test checkout").block_on()?;

        workspace
            .check_out(repo.op_id().clone(), None, &wc_commit)
            .block_on()?;

        Ok(())
    }

    #[test]
    fn clean_workspace_is_empty() -> Result<()> {
        let (temp_dir, _) = init_workspace()?;

        assert!(is_working_copy_empty(temp_dir.path())?);

        Ok(())
    }

    #[test]
    fn untracked_file_is_not_empty() -> Result<()> {
        let (temp_dir, _) = init_workspace()?;

        std::fs::write(temp_dir.path().join("file"), "contents")?;

        assert!(!is_working_copy_empty(temp_dir.path())?);

        Ok(())
    }

    #[test]
    fn tracked_file_modification_is_not_empty() -> Result<()> {
        let (temp_dir, mut workspace) = init_workspace()?;
        checkout_file(&mut workspace, "file", "contents")?;

        std::fs::write(temp_dir.path().join("file"), "modified")?;

        assert!(!is_working_copy_empty(temp_dir.path())?);

        Ok(())
    }

    #[test]
    fn ignored_untracked_file_is_empty() -> Result<()> {
        let (temp_dir, mut workspace) = init_workspace()?;
        checkout_file(&mut workspace, ".gitignore", "*.ignored\n")?;

        std::fs::write(temp_dir.path().join("file.ignored"), "contents")?;

        assert!(is_working_copy_empty(temp_dir.path())?);

        Ok(())
    }

    #[test]
    fn oversized_untracked_file_is_not_empty() -> Result<()> {
        let (temp_dir, _) = init_workspace()?;
        std::fs::write(temp_dir.path().join("large"), vec![0; (1 << 20) + 1])?;

        assert!(!is_working_copy_empty(temp_dir.path())?);

        Ok(())
    }

    #[test]
    fn subdirectory_resolves_to_workspace() -> Result<()> {
        let (temp_dir, _) = init_workspace()?;
        let subdirectory = temp_dir.path().join("dir");
        std::fs::create_dir(&subdirectory)?;

        assert!(is_working_copy_empty(&subdirectory)?);

        Ok(())
    }

    #[test]
    fn non_workspace_returns_error() -> Result<()> {
        let temp_dir = tempfile::tempdir()?;

        assert!(is_working_copy_empty(temp_dir.path()).is_err());

        Ok(())
    }
}

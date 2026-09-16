use std::collections::{BTreeMap, HashSet};
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use jj_lib::backend::CommitId;
use jj_lib::config::StackedConfig;
use jj_lib::gitignore::GitIgnoreFile;
use jj_lib::matchers::{EverythingMatcher, NothingMatcher};
use jj_lib::repo::{Repo as _, StoreFactories};
use jj_lib::revset::{ResolvedRevsetExpression, RevsetExpression};
use jj_lib::settings::UserSettings;
use jj_lib::working_copy::SnapshotOptions;
use jj_lib::workspace::{
    DefaultWorkspaceLoaderFactory, WorkspaceLoaderFactory as _, default_working_copy_factories,
};
use log::{info, trace};
use pollster::FutureExt as _;

const MAX_NEW_FILE_SIZE: u64 = 1 << 20;

#[derive(Debug, PartialEq, Eq)]
struct WorkspaceReport {
    path: PathBuf,
    unsnapshotted_paths: Vec<String>,
}

#[derive(Debug, PartialEq, Eq)]
struct LocalStack {
    change_id: String,
    commit_count: usize,
    description: String,
    bookmarks: Vec<String>,
    conflicted: bool,
}

#[derive(Debug, PartialEq, Eq)]
struct RepositoryReport {
    path: PathBuf,
    workspaces: Vec<WorkspaceReport>,
    stacks: Vec<LocalStack>,
}

#[derive(Debug)]
struct InspectionFailure {
    path: PathBuf,
    error: anyhow::Error,
}

impl RepositoryReport {
    fn needs_attention(&self) -> bool {
        !self.stacks.is_empty()
            || self
                .workspaces
                .iter()
                .any(|workspace| !workspace.unsnapshotted_paths.is_empty())
    }
}

pub(crate) fn process_targets(targets: Vec<PathBuf>) -> Result<()> {
    let settings = UserSettings::from_config(StackedConfig::with_defaults())?;
    let mut repositories = BTreeMap::<PathBuf, Vec<PathBuf>>::new();
    let mut failures = Vec::new();
    for path in targets {
        match DefaultWorkspaceLoaderFactory.create(&path) {
            Ok(loader) => match repository_key(loader.repo_path()) {
                Ok(repo_path) => repositories.entry(repo_path).or_default().push(path),
                Err(error) => failures.push(InspectionFailure {
                    path,
                    error: error.into(),
                }),
            },
            Err(error) => failures.push(InspectionFailure {
                path,
                error: error.into(),
            }),
        }
    }

    let mut reports = Vec::new();
    for (repo_path, workspace_paths) in repositories {
        info!("checking {}", repo_path.display());
        match inspect_repository(repo_path.clone(), workspace_paths, &settings) {
            Ok(report) => reports.push(report),
            Err(error) => failures.push(InspectionFailure {
                path: repo_path,
                error,
            }),
        }
    }
    print_reports(&reports, &failures);
    Ok(())
}

fn repository_key(repo_path: &Path) -> std::io::Result<PathBuf> {
    std::fs::canonicalize(repo_path)
}

fn inspect_repository(
    repo_path: PathBuf,
    workspace_paths: Vec<PathBuf>,
    settings: &UserSettings,
) -> Result<RepositoryReport> {
    let first_workspace_path = workspace_paths
        .first()
        .context("repository has no workspaces")?
        .clone();
    let mut workspaces = Vec::new();
    for path in workspace_paths {
        workspaces.push(inspect_workspace(&path, settings)?);
    }
    let loader = DefaultWorkspaceLoaderFactory.create(&first_workspace_path)?;
    let workspace = loader.load(
        settings,
        &StoreFactories::default(),
        &default_working_copy_factories(),
    )?;
    let repo = workspace.repo_loader().load_at_head().block_on()?;
    let stacks = find_local_stacks(repo.as_ref())?;
    Ok(RepositoryReport {
        path: repo_path,
        workspaces,
        stacks,
    })
}

fn inspect_workspace(path: &Path, settings: &UserSettings) -> Result<WorkspaceReport> {
    trace!("loading jj workspace from {}", path.display());
    let loader = DefaultWorkspaceLoaderFactory.create(path)?;
    let mut workspace = loader.load(
        settings,
        &StoreFactories::default(),
        &default_working_copy_factories(),
    )?;
    let operation = workspace
        .repo_loader()
        .load_operation(workspace.working_copy().operation_id())
        .block_on()?;
    let repo = workspace.repo_loader().load_at(&operation).block_on()?;
    let wc_commit_id = repo
        .view()
        .get_wc_commit_id(workspace.workspace_name())
        .with_context(|| {
            format!(
                "workspace {:?} has no working-copy commit",
                workspace.workspace_name()
            )
        })?;
    let wc_commit = repo.store().get_commit(wc_commit_id)?;

    let mut locked_workspace = workspace.start_working_copy_mutation()?;
    let (tree, stats) = locked_workspace
        .locked_wc()
        .snapshot(&SnapshotOptions {
            base_ignores: GitIgnoreFile::empty(),
            progress: None,
            start_tracking_matcher: &EverythingMatcher,
            force_tracking_matcher: &NothingMatcher,
            max_new_file_size: MAX_NEW_FILE_SIZE,
        })
        .block_on()?;

    let mut unsnapshotted_paths = stats
        .untracked_paths
        .keys()
        .map(|path| path.as_internal_file_string().to_owned())
        .collect::<Vec<_>>();
    if tree.tree_ids() != wc_commit.tree_ids() {
        unsnapshotted_paths.push("working-copy changes".to_owned());
    }
    unsnapshotted_paths.sort();

    Ok(WorkspaceReport {
        path: path.to_path_buf(),
        unsnapshotted_paths,
    })
}

fn find_local_stacks(repo: &dyn jj_lib::repo::Repo) -> Result<Vec<LocalStack>> {
    let view = repo.view();
    let remote_ids = view
        .all_remote_bookmarks()
        .filter(|(symbol, remote_ref)| symbol.remote.as_str() != "git" && remote_ref.is_present())
        .flat_map(|(_, remote_ref)| remote_ref.target.added_ids().cloned())
        .chain(
            view.all_remote_tags()
                .filter(|(symbol, remote_ref)| {
                    symbol.remote.as_str() != "git" && remote_ref.is_present()
                })
                .flat_map(|(_, remote_ref)| remote_ref.target.added_ids().cloned()),
        )
        .collect::<Vec<_>>();
    let local_expression = local_only_expression(remote_ids);
    let local_ids = local_expression
        .clone()
        .evaluate(repo)?
        .iter()
        .collect::<Result<Vec<_>, _>>()?;
    let ignored_wc_ids = ignorable_working_copy_ids(repo, &local_ids)?;
    let meaningful_expression = local_expression.minus(&RevsetExpression::commits(ignored_wc_ids));
    let head_ids = meaningful_expression
        .heads()
        .evaluate(repo)?
        .iter()
        .collect::<Result<Vec<_>, _>>()?;

    head_ids
        .into_iter()
        .map(|head_id| build_stack(repo, &meaningful_expression, head_id))
        .collect()
}

fn local_only_expression(remote_ids: Vec<CommitId>) -> std::sync::Arc<ResolvedRevsetExpression> {
    RevsetExpression::commits(remote_ids)
        .range(&RevsetExpression::visible_heads())
        .minus(&RevsetExpression::root())
}

fn ignorable_working_copy_ids(
    repo: &dyn jj_lib::repo::Repo,
    local_ids: &[CommitId],
) -> Result<Vec<CommitId>> {
    let local_ids = local_ids.iter().collect::<HashSet<_>>();
    let mut ignored = Vec::new();
    for commit_id in repo.view().wc_commit_ids().values() {
        if !local_ids.contains(commit_id)
            || repo
                .view()
                .local_bookmarks_for_commit(commit_id)
                .next()
                .is_some()
        {
            continue;
        }
        let commit = repo.store().get_commit(commit_id)?;
        if commit.description().is_empty() && commit.is_empty(repo).block_on()? {
            ignored.push(commit_id.clone());
        }
    }
    Ok(ignored)
}

fn build_stack(
    repo: &dyn jj_lib::repo::Repo,
    meaningful_expression: &std::sync::Arc<ResolvedRevsetExpression>,
    head_id: CommitId,
) -> Result<LocalStack> {
    let commit = repo.store().get_commit(&head_id)?;
    let commit_count = meaningful_expression
        .intersection(&RevsetExpression::commit(head_id.clone()).ancestors())
        .evaluate(repo)?
        .iter()
        .collect::<Result<Vec<_>, _>>()?
        .len();
    let bookmarks = repo
        .view()
        .local_bookmarks_for_commit(&head_id)
        .map(|(name, _)| name.as_str().to_owned())
        .collect();
    let description = commit
        .description()
        .lines()
        .next()
        .unwrap_or_default()
        .to_owned();
    Ok(LocalStack {
        change_id: commit.change_id().reverse_hex()[..8].to_owned(),
        commit_count,
        description,
        bookmarks,
        conflicted: commit.has_conflict(),
    })
}

fn print_reports(reports: &[RepositoryReport], failures: &[InspectionFailure]) {
    let attention_count = reports
        .iter()
        .filter(|report| report.needs_attention())
        .count();
    if attention_count == 0 && failures.is_empty() {
        println!("jjfold: all {} repositories are safe", reports.len());
        return;
    }
    println!("jjfold: {attention_count} repositories need attention");
    for report in reports.iter().filter(|report| report.needs_attention()) {
        println!("\n{}", report.path.display());
        for workspace in &report.workspaces {
            if !workspace.unsnapshotted_paths.is_empty() {
                println!(
                    "  filesystem   {} unsnapshotted item(s) in {}",
                    workspace.unsnapshotted_paths.len(),
                    workspace.path.display()
                );
                println!(
                    "  files        {}",
                    workspace.unsnapshotted_paths.join(", ")
                );
                println!("  action       snapshot, copy elsewhere, or delete");
            }
        }
        for stack in &report.stacks {
            let description = if stack.description.is_empty() {
                "(no description)"
            } else {
                &stack.description
            };
            println!(
                "  local-only   {} change(s) ending at {} {}{}",
                stack.commit_count,
                stack.change_id,
                description,
                if stack.conflicted {
                    " [conflicted]"
                } else {
                    ""
                }
            );
            if stack.bookmarks.is_empty() {
                println!("  action       create a bookmark and push, or abandon the stack");
            } else {
                println!("  bookmark     {}", stack.bookmarks.join(", "));
                println!(
                    "  action       jj git push --bookmark {}",
                    stack.bookmarks[0]
                );
            }
        }
    }
    let safe_count = reports.len() - attention_count;
    if safe_count > 0 {
        println!("\n{safe_count} repositories safe");
    }
    if !failures.is_empty() {
        println!("\n{} repositories could not be inspected", failures.len());
        for failure in failures {
            println!("  {}: {}", failure.path.display(), failure.error);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use anyhow::Result;
    use jj_lib::config::StackedConfig;
    use jj_lib::ref_name::WorkspaceNameBuf;
    use jj_lib::repo::Repo as _;
    use jj_lib::settings::UserSettings;
    use jj_lib::workspace::{Workspace, default_working_copy_factory};
    use pollster::FutureExt as _;

    use super::{RepositoryReport, WorkspaceReport, inspect_repository, repository_key};

    #[test]
    fn report_without_local_data_is_safe() {
        let report = RepositoryReport {
            path: PathBuf::from("repo"),
            workspaces: vec![WorkspaceReport {
                path: PathBuf::from("workspace"),
                unsnapshotted_paths: Vec::new(),
            }],
            stacks: Vec::new(),
        };

        assert!(!report.needs_attention());
    }

    #[test]
    fn unsnapshotted_data_needs_attention() {
        let report = RepositoryReport {
            path: PathBuf::from("repo"),
            workspaces: vec![WorkspaceReport {
                path: PathBuf::from("workspace"),
                unsnapshotted_paths: vec!["file".to_owned()],
            }],
            stacks: Vec::new(),
        };

        assert!(report.needs_attention());
    }

    #[test]
    fn repository_key_resolves_symlinks() -> Result<()> {
        let temp_dir = tempfile::tempdir()?;
        let repository = temp_dir.path().join("repository");
        std::fs::create_dir(&repository)?;

        #[cfg(unix)]
        {
            let alias = temp_dir.path().join("alias");
            std::os::unix::fs::symlink(&repository, &alias)?;

            assert_eq!(repository_key(&repository)?, repository_key(&alias)?);
        }

        Ok(())
    }

    #[test]
    fn repository_inspection_uses_resolved_operation_head() -> Result<()> {
        let temp_dir = tempfile::tempdir()?;
        let first_root = temp_dir.path().join("first");
        let second_root = temp_dir.path().join("second");
        std::fs::create_dir(&first_root)?;
        std::fs::create_dir(&second_root)?;
        let settings = UserSettings::from_config(StackedConfig::with_defaults())?;
        let (first_workspace, repo) = Workspace::init_simple(&settings, &first_root).block_on()?;
        let (_, repo) = Workspace::init_workspace_with_existing_repo(
            &second_root,
            first_workspace.repo_path(),
            &repo,
            &*default_working_copy_factory(),
            WorkspaceNameBuf::from("second"),
        )
        .block_on()?;

        let mut transaction = repo.start_transaction();
        transaction
            .repo_mut()
            .new_commit(
                vec![repo.store().root_commit_id().clone()],
                repo.store().empty_merged_tree(),
            )
            .set_description("local-only")
            .write()
            .block_on()?;
        transaction.commit("create local-only commit").block_on()?;

        let report = inspect_repository(
            repository_key(first_workspace.repo_path())?,
            vec![first_root, second_root],
            &settings,
        )?;

        assert!(
            report
                .stacks
                .iter()
                .any(|stack| stack.description == "local-only")
        );

        Ok(())
    }
}

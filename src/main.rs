//! [gfold](https://github.com/nickgerace/gfold) is a CLI tool that helps you keep track of
//! multiple Git repositories.

#![warn(
    bad_style,
    clippy::missing_panics_doc,
    clippy::panic,
    clippy::panic_in_result_fn,
    clippy::unwrap_in_result,
    clippy::unwrap_used,
    dead_code,
    improper_ctypes,
    missing_debug_implementations,
    // TODO(nick): fix missing docs.
    // missing_docs,
    no_mangle_generic_items,
    non_shorthand_field_patterns,
    overflowing_literals,
    path_statements,
    patterns_in_fns_without_body,
    unconditional_recursion,
    unreachable_pub,
    unused,
    unused_allocation,
    unused_comparisons,
    unused_parens,
    while_true
)]

use std::{env, path::PathBuf};

use anyhow::Result;
use args::Cli;
use clap::Parser;
use collector::RepositoryCollector;
use log::debug;

use crate::config::{Config, DisplayMode};
use crate::display::DisplayHarness;

// TODO(nick): investigate module visibility.
pub mod args;
pub mod collector;
pub mod config;
pub mod display;
pub mod repository_view;
pub mod status;

/// Initializes the logger based on the debug flag and `RUST_LOG` environment variable, then
/// parses CLI arguments and generates a [`Config`] by merging configurations as needed,
/// and finally collects results and displays them.
fn main() -> Result<()> {
    let cli = Cli::parse();

    env_logger::Builder::new()
        .filter_level(cli.verbose.log_level_filter())
        .init();
    debug!("initialized logger");

    let mut config = if cli.ignore_config_file {
        Config::try_config_default()?
    } else {
        Config::try_config()?
    };
    debug!("loaded initial config");

    if let Some(found_display_mode_raw) = &cli.display_mode {
        config.display_mode = *found_display_mode_raw;
    }
    if let Some(found_color_mode) = &cli.color_mode {
        config.color_mode = *found_color_mode;
    }
    if let Some(found_paths) = &cli.paths {
        let current_dir = env::current_dir()?;
        config.paths = found_paths
            .iter()
            .map(|p| current_dir.join(p).canonicalize())
            .collect::<Result<Vec<PathBuf>, _>>()?;
    }
    debug!("finalized config options");

    if cli.dry_run {
        config.print()?;
    } else {
        let (include_email, include_submodules) = match config.display_mode {
            DisplayMode::Classic => (false, false),
            DisplayMode::Json => (true, true),
            DisplayMode::Standard | DisplayMode::StandardAlphabetical => (true, false),
        };
        for path in &config.paths {
            debug!("processing path: {}", path.display());

            let repository_collection =
                RepositoryCollector::run(path, include_email, include_submodules)?;
            let display_harness = DisplayHarness::new(config.display_mode, config.color_mode);
            display_harness.run(&repository_collection)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use collector::RepositoryCollection;
    use git2::ErrorCode;
    use git2::Oid;
    use git2::Signature;
    use git2::{Repository, RepositoryInitOptions};
    use repository_view::RepositoryView;
    use status::Status;
    use std::fs::File;
    use std::path::{Path, PathBuf};
    use std::{fs, io};
    use tempfile::tempdir;

    /// This scenario test for `gfold` covers an end-to-end usage scenario. It uses the
    /// [`tempfile`](tempfile) crate to create some repositories with varying states and levels
    /// of nesting.
    #[allow(clippy::panic_in_result_fn)]
    #[test]
    fn scenario() -> anyhow::Result<()> {
        // Temporary directory structure:
        // └── root
        //     ├── one (repo)
        //     │   └── file
        //     ├── two (repo)
        //     ├── three (repo)
        //     ├── eight (worktree repo)
        //     └── nested
        //         ├── four (repo)
        //         ├── five (repo)
        //         │   └── file
        //         ├── six (repo)
        //         └── seven (repo)
        let root = tempdir()?;
        let repo_one = create_directory(&root, "one")?;
        let repo_two = create_directory(&root, "two")?;
        let repo_three = create_directory(&root, "three")?;

        let nested = create_directory(&root, "nested")?;
        let repo_four = create_directory(&nested, "four")?;
        let repo_five = create_directory(&nested, "five")?;
        let repo_six = create_directory(&nested, "six")?;
        let repo_seven = create_directory(&nested, "seven")?;
        // repo_eight doesn't need a dir. It's created via 'worktree add'

        // Setup repo opts
        let mut opts = RepositoryInitOptions::new();
        let initial_head = "main";
        opts.initial_head(initial_head);

        // Repo One
        Repository::init_opts(&repo_one, &opts)?;
        create_file(&repo_one)?;

        // Repo Two
        Repository::init_opts(&repo_two, &opts)?;

        // Repo Three
        Repository::init_opts(&repo_three, &opts)?;

        // Repo Four
        let repository = Repository::init_opts(&repo_four, &opts)?;
        if let Err(e) = repository.remote("origin", "https://github.com/nickgerace/gfold")
            && e.code() != ErrorCode::Exists
        {
            return Err(e.into());
        }

        // Repo Five
        Repository::init_opts(&repo_five, &opts)?;
        create_file(&repo_five)?;

        // Repo Six
        let repository = Repository::init_opts(&repo_six, &opts)?;
        if let Err(e) = repository.remote("fork", "https://github.com/nickgerace/gfold")
            && e.code() != ErrorCode::Exists
        {
            return Err(e.into());
        }
        commit_head_and_create_branch(&repository, "feat")?;

        // Repo Seven
        let repository = Repository::init_opts(&repo_seven, &opts)?;
        if let Err(e) = repository.remote("origin", "https://github.com/nickgerace/gfold")
            && e.code() != ErrorCode::Exists
        {
            return Err(e.into());
        }
        commit_head_and_create_branch(&repository, "needtopush")?;
        repository.set_head("refs/heads/needtopush")?;

        // Repo Eight
        let worktree_path = root.path().join("eight");
        repository.worktree("working-in-a-tree", &worktree_path, None)?;

        // Generate the collection directly with a default config and ensure the resulting views
        // match what we expect.
        let mut expected_collection = RepositoryCollection::new();
        let expected_views_key = root
            .path()
            .to_str()
            .expect("could not convert PathBuf to &str")
            .to_string();
        let mut expected_views = vec![
            RepositoryView::finalize(
                &worktree_path,
                Some("working-in-a-tree".to_string()),
                Status::Unpushed,
                Some("https://github.com/nickgerace/gfold".to_string()),
                None,
                Vec::with_capacity(0),
            )?,
            RepositoryView::finalize(
                &repo_one,
                Some("HEAD".to_string()),
                Status::Unclean,
                None,
                None,
                Vec::with_capacity(0),
            )?,
            RepositoryView::finalize(
                &repo_two,
                Some("HEAD".to_string()),
                Status::Clean,
                None,
                None,
                Vec::with_capacity(0),
            )?,
            RepositoryView::finalize(
                &repo_three,
                Some("HEAD".to_string()),
                Status::Clean,
                None,
                None,
                Vec::with_capacity(0),
            )?,
        ];
        expected_views.sort_by(|a, b| a.name.cmp(&b.name));
        expected_collection.insert(Some(expected_views_key), expected_views);

        // Add nested views to the expected collection.
        let nested_expected_views_key = nested
            .to_str()
            .expect("could not convert PathBuf to &str")
            .to_string();
        let mut nested_expected_views_raw = vec![
            RepositoryView::finalize(
                &repo_four,
                Some("HEAD".to_string()),
                Status::Clean,
                Some("https://github.com/nickgerace/gfold".to_string()),
                None,
                Vec::with_capacity(0),
            )?,
            RepositoryView::finalize(
                &repo_five,
                Some("HEAD".to_string()),
                Status::Unclean,
                None,
                None,
                Vec::with_capacity(0),
            )?,
            RepositoryView::finalize(
                &repo_six,
                Some(initial_head.to_string()),
                Status::Unpushed,
                Some("https://github.com/nickgerace/gfold".to_string()),
                None,
                Vec::with_capacity(0),
            )?,
            RepositoryView::finalize(
                &repo_seven,
                Some("needtopush".to_string()),
                Status::Unpushed,
                Some("https://github.com/nickgerace/gfold".to_string()),
                None,
                Vec::with_capacity(0),
            )?,
        ];
        nested_expected_views_raw.sort_by(|a, b| a.name.cmp(&b.name));
        expected_collection.insert(Some(nested_expected_views_key), nested_expected_views_raw);

        // Generate a collection.
        let found_collection = RepositoryCollector::run(root.path(), false, false)?;

        // Ensure the found collection matches our expected one. Sort the collection for the
        // assertion.
        let mut found_collection_sorted = RepositoryCollection::new();
        for (key, mut value) in found_collection {
            value.sort_by(|a, b| a.name.cmp(&b.name));
            found_collection_sorted.insert(key, value);
        }
        assert_eq!(
            expected_collection,     // expected
            found_collection_sorted  // actual
        );
        Ok(())
    }

    fn create_directory<P: AsRef<Path>>(parent: P, name: &str) -> io::Result<PathBuf> {
        let parent = parent.as_ref();
        let new_directory = parent.join(name);

        if let Err(e) = fs::create_dir(&new_directory)
            && e.kind() != io::ErrorKind::AlreadyExists
        {
            return Err(e);
        }
        Ok(new_directory)
    }

    fn create_file<P: AsRef<Path>>(parent: P) -> io::Result<()> {
        let parent = parent.as_ref();
        File::create(parent.join("file"))?;
        Ok(())
    }

    fn commit_head_and_create_branch(repository: &Repository, name: &str) -> anyhow::Result<()> {
        // We need to commit at least once before branching.
        let commit_oid = commit(repository, "HEAD")?;
        let commit = repository.find_commit(commit_oid)?;
        repository.branch(name, &commit, true)?;
        Ok(())
    }

    // Source: https://github.com/rust-lang/git2-rs/pull/885
    fn commit(repository: &Repository, update_ref: &str) -> anyhow::Result<Oid> {
        // We will commit the contents of the index.
        let mut index = repository.index()?;
        let tree_oid = index.write_tree()?;
        let tree = repository.find_tree(tree_oid)?;

        // If this is the first commit, there is no parent. If the object returned by
        // "revparse_single" cannot be converted into a commit, then it isn't a commit and we know
        // there is no parent _commit_.
        let maybe_parent = match repository.revparse_single("HEAD") {
            Ok(object) => object.into_commit().ok(),
            Err(e) if e.code() == ErrorCode::NotFound => None,
            Err(e) => return Err(e.into()),
        };

        let mut parents = Vec::new();
        if let Some(parent) = maybe_parent.as_ref() {
            parents.push(parent);
        };

        let signature = Signature::now("Bob", "bob@bob")?;
        Ok(repository.commit(
            Some(update_ref),
            &signature,
            &signature,
            "hello",
            &tree,
            parents.as_ref(),
        )?)
    }
}

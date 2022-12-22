//! [gfold](https://github.com/nickgerace/gfold) is a CLI-driven application that helps you keep
//! track of multiple Git repositories. The source code uses private modules rather than leveraging
//! a library via `lib.rs`.

use env_logger::Builder;
use log::debug;
use log::LevelFilter;
use std::env;

use crate::cli::CliHarness;

mod cli;
mod collector;
mod config;
mod display;
mod repository_view;
mod run;
mod status;

/// Initializes the logger based on the debug flag and `RUST_LOG` environment variable and uses
/// the [`CliHarness`] to generate a [`Config`](config::Config). Then, this calls
/// [`CliHarness::run()`].
fn main() -> anyhow::Result<()> {
    match env::var("RUST_LOG").is_err() {
        true => Builder::new().filter_level(LevelFilter::Off).init(),
        false => env_logger::init(),
    }
    debug!("initialized logger");

    let cli_harness = CliHarness::new();
    cli_harness.run()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use anyhow::anyhow;
    use git2::ErrorCode;
    use git2::Oid;
    use git2::Repository;
    use git2::Signature;
    use pretty_assertions::assert_eq;
    use std::fs::File;
    use std::path::{Path, PathBuf};
    use std::{fs, io};
    use tempfile::tempdir;

    use crate::collector::{RepositoryCollection, RepositoryCollector};
    use crate::config::{ColorMode, Config, DisplayMode};
    use crate::repository_view::RepositoryView;
    use crate::run::RunHarness;
    use crate::status::Status;

    /// This integration test for `gfold` covers an end-to-end usage scenario. It uses the
    /// [`tempfile`](tempfile) crate to create some repositories with varying states and levels
    /// of nesting.
    #[test]
    fn integration() -> anyhow::Result<()> {
        env_logger::builder()
            .is_test(true)
            .filter_level(LevelFilter::Info)
            .try_init()?;

        // Temporary directory structure:
        // └── root
        //     ├── one (repo)
        //     │   └── file
        //     ├── two (repo)
        //     ├── three (repo)
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

        // Repo One
        Repository::init(&repo_one)?;
        create_file(&repo_one)?;

        // Repo Two
        Repository::init(&repo_two)?;

        // Repo Three
        Repository::init(&repo_three)?;

        // Repo Four
        let repository = Repository::init(&repo_four)?;
        if let Err(e) = repository.remote("origin", "https://github.com/nickgerace/gfold") {
            if e.code() != ErrorCode::Exists {
                return Err(e.into());
            }
        }

        // Repo Five
        Repository::init(&repo_five)?;
        create_file(&repo_five)?;

        // Repo Six
        let repository = Repository::init(&repo_six)?;
        if let Err(e) = repository.remote("fork", "https://github.com/nickgerace/gfold") {
            if e.code() != ErrorCode::Exists {
                return Err(e.into());
            }
        }
        commit_head_and_create_branch(&repository, "feat")?;

        // Repo Seven
        let repository = Repository::init(&repo_seven)?;
        if let Err(e) = repository.remote("origin", "https://github.com/nickgerace/gfold") {
            if e.code() != ErrorCode::Exists {
                return Err(e.into());
            }
        }
        commit_head_and_create_branch(&repository, "needtopush")?;
        repository.set_head("refs/heads/needtopush")?;

        // Run once with default display mode.
        let mut config = Config::try_config_default()?;
        config.path = root.path().to_path_buf();
        config.color_mode = ColorMode::Never;
        let run_harness = RunHarness::new(&config);
        run_harness.run()?;

        // Now, let's run a second time, but generate the collection directly and ensure the
        // resulting views match what we expect.
        let mut expected_collection = RepositoryCollection::new();
        let expected_views_key = root
            .path()
            .to_str()
            .ok_or_else(|| anyhow!("could not convert PathBuf to &str"))?
            .to_string();
        let mut expected_views = vec![
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
            .ok_or_else(|| anyhow!("could not convert PathBuf to &str"))?
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
                Some("master".to_string()),
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

        // Generate a collection. Use classic display mode to avoid collecting email results.
        config.display_mode = DisplayMode::Classic;
        let found_collection = RepositoryCollector::run(&config.path, config.display_mode)?;

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

    fn create_directory<P: AsRef<Path>>(parent: P, name: &str) -> anyhow::Result<PathBuf> {
        let parent = parent.as_ref();
        let new_directory = parent.join(name);

        if let Err(e) = fs::create_dir(&new_directory) {
            if e.kind() != io::ErrorKind::AlreadyExists {
                return Err(anyhow!(
                    "could not create directory ({:?}) due to error kind: {:?}",
                    &new_directory,
                    e.kind()
                ));
            }
        }
        Ok(new_directory)
    }

    fn create_file<P: AsRef<Path>>(parent: P) -> anyhow::Result<()> {
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
            Ok(object) => match object.into_commit() {
                Ok(commit) => Some(commit),
                Err(_) => None,
            },
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

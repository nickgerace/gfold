//! **libgfold** provides the ability to find a minimal set of user-relevant information for git
//! repositories on a local filesystem.
//!
//! This library powers [**gfold**](https://github.com/nickgerace/gfold).

#![warn(
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms,
    unreachable_pub,
    bad_style,
    dead_code,
    improper_ctypes,
    non_shorthand_field_patterns,
    no_mangle_generic_items,
    overflowing_literals,
    path_statements,
    patterns_in_fns_without_body,
    private_in_public,
    unconditional_recursion,
    unused,
    unused_allocation,
    unused_comparisons,
    unused_parens,
    while_true,
    clippy::missing_panics_doc
)]

pub mod collector;
pub mod repository_view;
pub mod status;

pub use collector::RepositoryCollection;
pub use collector::RepositoryCollector;
pub use repository_view::RepositoryView;
pub use status::Status;

#[cfg(test)]
mod tests {
    use super::*;

    use git2::ErrorCode;
    use git2::Oid;
    use git2::Repository;
    use git2::Signature;
    use log::LevelFilter;
    use pretty_assertions::assert_eq;
    use std::fs::File;
    use std::path::{Path, PathBuf};
    use std::{fs, io};
    use tempfile::tempdir;

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

        if let Err(e) = fs::create_dir(&new_directory) {
            if e.kind() != io::ErrorKind::AlreadyExists {
                return Err(e);
            }
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

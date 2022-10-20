//! [gfold](https://github.com/nickgerace/gfold) is a CLI-driven application that helps you keep
//! track of multiple Git repositories. The source code uses private modules rather than leveraging
//! a library via `lib.rs`.

use env_logger::Builder;
use log::debug;
use log::LevelFilter;
use std::env;

mod cli;
mod config;
mod display;
mod error;
mod report;
mod run;
mod status;

/// Initializes the logger based on the debug flag and `RUST_LOG` environment variable and calls
/// [`cli::parse_and_run()`] to generate a [`config::Config`] and eventually call [`run::run()`].
fn main() -> anyhow::Result<()> {
    match env::var("RUST_LOG").is_err() {
        true => Builder::new().filter_level(LevelFilter::Off).init(),
        false => env_logger::init(),
    }
    debug!("initialized logger");

    cli::parse_and_run()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::config::{ColorMode, Config, DisplayMode};
    use crate::report::{LabeledReports, Report};
    use crate::status::Status;

    use anyhow::{anyhow, Result};
    use git2::ErrorCode;
    use git2::Repository;
    use pretty_assertions::assert_eq;
    use std::collections::BTreeMap;
    use std::path::{Path, PathBuf};
    use std::{env, fs, io};

    /// This integration test for `gfold` covers an end-to-end usage scenario. It does not
    /// _remove_ anything in the filesystem (for saftey), so you must delete the `test`
    /// directory underneath `target` to regenerate a clean dataset.
    #[test]
    fn integration() -> Result<()> {
        // Test directory structure within "target":
        // └── test
        //     ├── bar
        //     ├── baz
        //     ├── foo
        //     │   └── newfile
        //     └── nested
        //         ├── one
        //         │   └── newfile
        //         ├── three
        //         └── two
        let test_directory = integration_init()?;
        create_directory(&test_directory)?;

        for name in ["foo", "bar", "baz"] {
            let current = test_directory.join(name);
            create_directory(&current)?;
            Repository::init(&current)?;

            if name == "foo" {
                create_file(&current.join("newfile"))?;
            }
        }

        let nested = test_directory.join("nested");
        create_directory(&nested)?;
        for name in ["one", "two", "three"] {
            let current = nested.join(name);
            create_directory(&current)?;
            let repository = Repository::init(&current)?;

            if name == "one" {
                create_file(&current.join("newfile"))?;
            }

            if name == "two" {
                if let Err(e) = repository.remote("origin", "https://github.com/nickgerace/gfold") {
                    if e.code() != ErrorCode::Exists {
                        return Err(e.into());
                    }
                }
            }

            if name == "three" {
                if let Err(e) = repository.remote("fork", "https://github.com/nickgerace/gfold") {
                    if e.code() != ErrorCode::Exists {
                        return Err(e.into());
                    }
                }
            }
        }

        let mut config = Config::new()?;
        config.path = test_directory.clone();
        config.color_mode = ColorMode::Never;
        assert!(run::run(&config).is_ok());

        // Now, let's ensure our reports are what we expect.
        let mut expected_reports: LabeledReports = BTreeMap::new();

        let key = test_directory
            .to_str()
            .ok_or_else(|| anyhow!("could not convert PathBuf to &str"))?
            .to_string();
        let mut reports = vec![
            Report::new(
                &test_directory.join("foo"),
                "HEAD",
                &Status::Unclean,
                None,
                None,
            )?,
            Report::new(
                &test_directory.join("bar"),
                "HEAD",
                &Status::Clean,
                None,
                None,
            )?,
            Report::new(
                &test_directory.join("baz"),
                "HEAD",
                &Status::Clean,
                None,
                None,
            )?,
        ];
        reports.sort_by(|a, b| a.name.cmp(&b.name));
        expected_reports.insert(Some(key), reports);

        let nested_test_dir = test_directory.join("nested");
        let key = nested_test_dir
            .to_str()
            .ok_or_else(|| anyhow!("could not convert PathBuf to &str"))?
            .to_string();
        let mut reports = vec![
            Report::new(
                &nested_test_dir.join("one"),
                "HEAD",
                &Status::Unclean,
                None,
                None,
            )?,
            Report::new(
                &nested_test_dir.join("two"),
                "HEAD",
                &Status::Clean,
                Some("https://github.com/nickgerace/gfold".to_string()),
                None,
            )?,
            Report::new(
                &nested_test_dir.join("three"),
                "HEAD",
                &Status::Clean,
                Some("https://github.com/nickgerace/gfold".to_string()),
                None,
            )?,
        ];
        reports.sort_by(|a, b| a.name.cmp(&b.name));
        expected_reports.insert(Some(key), reports);

        // Use classic display mode to avoid collecting email results.
        config.display_mode = DisplayMode::Classic;
        let found_labeled_reports = report::generate_reports(&config.path, &config.display_mode)?;
        let mut found_labeled_reports_sorted = LabeledReports::new();
        for labeled_report in found_labeled_reports {
            let mut value = labeled_report.1;
            value.sort_by(|a, b| a.name.cmp(&b.name));
            found_labeled_reports_sorted.insert(labeled_report.0.clone(), value.clone());
        }

        assert_eq!(found_labeled_reports_sorted, expected_reports);
        Ok(())
    }

    /// Ensure we are underneath the repository root. Safely create the test directory.
    fn integration_init() -> Result<PathBuf> {
        let manifest_directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let repo_root = manifest_directory
            .parent()
            .ok_or_else(|| anyhow!("could not get parent"))?
            .parent()
            .ok_or_else(|| anyhow!("could not get parent"))?;
        assert!(Repository::open(repo_root).is_ok());

        let target = repo_root.join("target");
        create_directory(&target)?;
        let test = target.join("test");
        Ok(test)
    }

    fn create_directory(path: &Path) -> Result<()> {
        if let Err(e) = fs::create_dir(path) {
            if e.kind() != io::ErrorKind::AlreadyExists {
                return Err(anyhow!(
                    "could not create directory ({:?}) due to error kind: {:?}",
                    path,
                    e.kind()
                ));
            }
        }
        Ok(())
    }

    fn create_file(path: &Path) -> Result<()> {
        if let Err(e) = fs::File::create(path) {
            if e.kind() != io::ErrorKind::AlreadyExists {
                return Err(anyhow!(
                    "could not create file ({:?}) due to error kind: {:?}",
                    path,
                    e.kind()
                ));
            }
        }
        Ok(())
    }
}

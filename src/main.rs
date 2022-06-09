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
    use anyhow::anyhow;
    use anyhow::Result;
    use git2::ErrorCode;
    use git2::Repository;
    use std::collections::BTreeMap;
    use std::path::{Path, PathBuf};
    use std::{env, fs, io};

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

        let manifest_directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let target = manifest_directory.join("target");
        create_directory(&target)?;

        // Warning: setting up test directory by removing it and its contents recursively.
        let test = target.join("test");
        if let Err(error) = fs::remove_dir_all(&test) {
            if error.kind() != io::ErrorKind::NotFound {
                return Err(error.into());
            }
        }
        create_directory(&test)?;

        for name in ["foo", "bar", "baz"] {
            let current = test.join(name);
            create_directory(&current)?;
            Repository::init(&current)?;

            if name == "foo" {
                create_file(&current.join("newfile"))?;
            }
        }

        let nested = test.join("nested");
        create_directory(&nested)?;
        for name in ["one", "two", "three"] {
            let current = nested.join(name);
            create_directory(&current)?;
            let repository = Repository::init(&current).expect("could not initialize repository");

            if name == "one" {
                create_file(&current.join("newfile"))?;
            }

            if name == "two" {
                if let Err(error) =
                    repository.remote("origin", "https://github.com/nickgerace/gfold")
                {
                    if error.code() != ErrorCode::Exists {
                        return Err(error.into());
                    }
                }
            }

            if name == "three" {
                if let Err(error) = repository.remote("fork", "https://github.com/nickgerace/gfold")
                {
                    if error.code() != ErrorCode::Exists {
                        return Err(error.into());
                    }
                }
            }
        }

        let mut config = Config::new()?;
        config.path = test;
        config.color_mode = ColorMode::Never;
        assert!(run::run(&config).is_ok());

        // Now, let's ensure our reports are what we expect.
        let test_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("target")
            .join("test");
        let mut expected_reports: LabeledReports = BTreeMap::new();

        let key = test_dir
            .to_str()
            .ok_or_else(|| anyhow!("could not convert to &str"))?
            .to_string();
        let mut reports = vec![
            Report::new(&test_dir.join("foo"), "HEAD", &Status::Unclean, None, None)?,
            Report::new(&test_dir.join("bar"), "HEAD", &Status::Clean, None, None)?,
            Report::new(&test_dir.join("baz"), "HEAD", &Status::Clean, None, None)?,
        ];
        reports.sort_by(|a, b| a.name.cmp(&b.name));
        expected_reports.insert(Some(key), reports);

        let nested_test_dir = test_dir.join("nested");
        let key = nested_test_dir
            .to_str()
            .ok_or_else(|| anyhow!("could not convert to &str"))?
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
        let found_labeled_reports = report::generate_reports(&config.path, &config.display_mode)
            .expect("could not generate reports");
        let mut found_labeled_reports_sorted = LabeledReports::new();
        for labeled_report in found_labeled_reports {
            let mut value = labeled_report.1;
            value.sort_by(|a, b| a.name.cmp(&b.name));
            found_labeled_reports_sorted.insert(labeled_report.0.clone(), value.clone());
        }

        assert_eq!(found_labeled_reports_sorted, expected_reports);

        Ok(())
    }

    fn create_directory(path: &Path) -> Result<()> {
        if let Err(error) = fs::create_dir(path) {
            if error.kind() != io::ErrorKind::AlreadyExists {
                return Err(error.into());
            }
        }
        Ok(())
    }

    fn create_file(path: &Path) -> Result<()> {
        if let Err(error) = fs::File::create(path) {
            if error.kind() != io::ErrorKind::AlreadyExists {
                return Err(error.into());
            }
        }
        Ok(())
    }
}

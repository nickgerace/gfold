//! [gfold](https://github.com/nickgerace/gfold) is a CLI-driven application that helps you keep
//! track of multiple Git repositories. The source code uses private modules rather than leveraging
//! a library via `lib.rs`.

use crate::result::Result;

mod cli;
mod config;
mod display;
mod error;
mod report;
mod result;
mod run;
mod status;

/// Calls [`cli::parse_and_run()`] to generate a [`config::Config`] and eventually call [`run::run()`];
fn main() -> Result<()> {
    cli::parse_and_run()
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::config::{Config, DisplayMode};
    use crate::report::{LabeledReports, Report};
    use crate::status::Status;
    use git2::Repository;
    use pretty_assertions::assert_eq;
    use std::collections::BTreeMap;
    use std::path::Path;
    use std::{env, fs, io};

    #[test]
    fn integration() {
        fn create_dir_or_die(path: &Path) {
            if let Err(e) = fs::create_dir(path) {
                if e.kind() != io::ErrorKind::AlreadyExists {
                    panic!(
                        "could not create directory ({:?}) due to error kind: {:?}",
                        path,
                        e.kind()
                    );
                }
            }
        }

        fn create_file_or_die(path: &Path) {
            if let Err(e) = fs::File::create(path) {
                if e.kind() != io::ErrorKind::AlreadyExists {
                    panic!(
                        "could not create file ({:?}) due to error kind: {:?}",
                        path,
                        e.kind()
                    );
                }
            }
        }

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

        let cwd = env::current_dir().expect("failed to get current working directory");
        let target = cwd.join("target");
        create_dir_or_die(&target);
        let test = target.join("test");
        create_dir_or_die(&test);
        for name in ["foo", "bar", "baz"] {
            let current = test.join(name);
            create_dir_or_die(&current);
            Repository::init(&current).expect("could not initialize repository");

            if name == "foo" {
                create_file_or_die(&current.join("newfile"));
            }
        }

        let nested = test.join("nested");
        create_dir_or_die(&nested);
        for name in ["one", "two", "three"] {
            let current = nested.join(name);
            create_dir_or_die(&current);
            Repository::init(&current).expect("could not initialize repository");

            if name == "one" {
                create_file_or_die(&current.join("newfile"));
            }
        }

        let mut config = Config::new().expect("could not create new config");
        config.path = test;
        assert!(run::run(&config).is_ok());

        // Now, let's ensure our reports are what we expect.
        let test_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("target")
            .join("test");
        let mut expected_reports: LabeledReports = BTreeMap::new();

        let key = test_dir
            .to_str()
            .expect("could not convert to str")
            .to_string();
        let mut reports = vec![
            Report::new(&test_dir.join("foo"), "HEAD", &Status::Unclean, None, None)
                .expect("could not create report"),
            Report::new(&test_dir.join("bar"), "HEAD", &Status::Clean, None, None)
                .expect("could not create report"),
            Report::new(&test_dir.join("baz"), "HEAD", &Status::Clean, None, None)
                .expect("could not create report"),
        ];
        reports.sort_by(|a, b| a.name.cmp(&b.name));
        expected_reports.insert(Some(key), reports);

        let nested_test_dir = test_dir.join("nested");
        let key = nested_test_dir
            .to_str()
            .expect("could not convert to str")
            .to_string();
        let mut reports = vec![
            Report::new(
                &nested_test_dir.join("one"),
                "HEAD",
                &Status::Unclean,
                None,
                None,
            )
            .expect("could not create report"),
            Report::new(
                &nested_test_dir.join("two"),
                "HEAD",
                &Status::Clean,
                None,
                None,
            )
            .expect("could not create report"),
            Report::new(
                &nested_test_dir.join("three"),
                "HEAD",
                &Status::Clean,
                None,
                None,
            )
            .expect("could not create report"),
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
    }
}

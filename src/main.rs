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

    use crate::config::Config;
    use git2::Repository;
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
        //     │   └── newfile
        //     └── nested
        //         ├── one
        //         │   ├── newfile
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
    }
}

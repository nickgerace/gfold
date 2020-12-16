/*
 * gfold
 * https://github.com/nickgerace/gfold
 * Author: Nick Gerace
 * License: Apache 2.0
 */

//! This is a CLI tool to help keep track of your Git repositories.

mod driver;
mod util;

#[macro_use]
extern crate prettytable;

use std::path::Path;

use eyre::Result;

/// This function is the primary, backend driver for `gfold`.
///
/// - `path`: the target path to find and parse Git repositories
/// - `no_color`: disables color, bolding, etc.
/// - `recursive`: recursively searches directories for Git repositories
/// - `skip_sort`: skips sorting the repositories for output
///
/// When executed, results will be printed to STDOUT.
pub fn run(
    path: &Path,
    disable_unpushed_check: bool,
    no_color: bool,
    recursive: bool,
    skip_sort: bool,
) -> Result<()> {
    let config = driver::Config {
        disable_unpushed_check,
        no_color,
        recursive,
        skip_sort,
    };
    let results = driver::Results::new(path, &config)?;
    results.print_results();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn current_directory() {
        let current_dir = env::current_dir().expect("failed to get CWD");
        assert_ne!(run(&current_dir, false, false, false, false).is_err(), true);
    }

    #[test]
    fn parent_directory() {
        let mut current_dir = env::current_dir().expect("failed to get CWD");
        current_dir.pop();
        assert_ne!(run(&current_dir, false, false, false, false).is_err(), true);
    }

    #[test]
    fn parent_directory_all_options() {
        let mut current_dir = env::current_dir().expect("failed to get CWD");
        current_dir.pop();

        assert_ne!(run(&current_dir, false, true, false, false).is_err(), true);
        assert_ne!(run(&current_dir, false, true, false, true).is_err(), true);
        assert_ne!(run(&current_dir, false, true, true, false).is_err(), true);

        assert_ne!(run(&current_dir, false, false, true, false).is_err(), true);
        assert_ne!(run(&current_dir, false, false, true, true).is_err(), true);
        assert_ne!(run(&current_dir, false, false, false, true).is_err(), true);

        assert_ne!(run(&current_dir, false, true, true, true).is_err(), true);
    }
}

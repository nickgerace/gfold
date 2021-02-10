//! This is a CLI tool to help keep track of your Git repositories.

#[macro_use]
extern crate prettytable;

use std::path::Path;

use eyre::Result;

mod driver;
mod util;

/// This function is the primary, backend driver for `gfold`.
///
/// - `enable_unpushed_check`: enable checking for unpushed commits (experimental)
/// - `include_non_repos`: include standard directories in the result
/// - `no_color`: disables color, bolding, etc.
/// - `path`: the target path to find and parse Git repositories
/// - `recursive`: recursively searches directories for Git repositories
/// - `skip_sort`: skips sorting the repositories for output
///
/// When executed, results will be printed to STDOUT.
pub fn run(
    path: &Path,
    enable_unpushed_check: bool,
    include_non_repos: bool,
    no_color: bool,
    recursive: bool,
    skip_sort: bool,
) -> Result<()> {
    let config = driver::Config {
        enable_unpushed_check,
        include_non_repos,
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
        assert_ne!(
            run(&current_dir, false, false, false, false, false).is_err(),
            true
        );
    }

    #[test]
    fn parent_directory() {
        let mut current_dir = env::current_dir().expect("failed to get CWD");
        current_dir.pop();
        assert_ne!(
            run(&current_dir, false, false, false, false, false).is_err(),
            true
        );
    }

    #[test]
    fn parent_directory_all_options() {
        let mut current_dir = env::current_dir().expect("failed to get CWD");
        current_dir.pop();
        for include_non_repos in vec![true, false] {
            for no_color in vec![true, false] {
                for recursive in vec![true, false] {
                    for skip_sort in vec![true, false] {
                        assert_ne!(
                            run(
                                &current_dir,
                                false,
                                include_non_repos,
                                no_color,
                                recursive,
                                skip_sort
                            )
                            .is_err(),
                            true
                        );
                    }
                }
            }
        }
    }
}

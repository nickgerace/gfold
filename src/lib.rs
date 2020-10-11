/*
 * gfold
 * https://github.com/nickgerace/gfold
 * Author: Nick Gerace
 * License: Apache 2.0
 */

#[macro_use]
extern crate prettytable;

use std::cmp::Ordering;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

/// Creating a ```Results``` object requires using this ```struct``` as a pre-requisite.
struct Config {
    recursive: bool,
    skip_sort: bool,
}

/// This ```struct``` is a wrapper around the ```prettytable::Table``` object.
/// It exists to provide a label for the table.
struct TableWrapper {
    path_string: String,
    table: prettytable::Table,
}

/// Contains all tables with results for each directory.
struct Results(Vec<TableWrapper>);

impl Results {
    /// Create a new ```Results``` object with a given path and config.
    fn new(path: &Path, config: &Config) -> Result<Results, Box<dyn Error>> {
        let mut results = Results(Vec::new());
        results.execute_in_directory(&config, path)?;
        if !&config.skip_sort {
            results.sort_results();
        }
        Ok(results)
    }

    /// Load results into the calling ```Results``` object via a given path and config.
    /// This function may be called recursively.
    fn execute_in_directory(&mut self, config: &Config, dir: &Path) -> Result<(), Box<dyn Error>> {
        // FIXME: find ways to add concurrent programming (tokio, async, etc.) to this section.
        let path_entries = fs::read_dir(dir)?;
        let mut repos = Vec::new();

        for entry in path_entries {
            let subpath = &entry?.path();
            if subpath.is_dir() {
                if git2::Repository::open(subpath).is_ok() {
                    repos.push(subpath.to_owned());
                } else if config.recursive {
                    self.execute_in_directory(&config, &subpath)?;
                }
            }
        }
        if !repos.is_empty() {
            if !&config.skip_sort {
                repos.sort();
            }
            // If a table was successfully created with the given repositories, add the table.
            if let Some(table_wrapper) = create_table_from_paths(repos, &dir) {
                self.0.push(table_wrapper);
            }
        }
        Ok(())
    }

    /// Sort the results alphabetically using ```sort_by_key```.
    /// This function will only perform the sort if there are at least two ```TableWrapper``` objects.
    fn sort_results(&mut self) {
        if self.0.len() >= 2 {
            // FIXME: find a way to do this without "clone()".
            self.0.sort_by_key(|table| table.path_string.clone());
        }
    }

    /// Iterate through every table and print each to STDOUT.
    /// If there is only one table, this function avoids using a loop.
    fn print_results(self) {
        match self.0.len().cmp(&1) {
            Ordering::Greater => {
                for table_wrapper in self.0 {
                    println!("\n{}", table_wrapper.path_string);
                    table_wrapper.table.printstd();
                }
            }
            Ordering::Equal => {
                self.0[0].table.printstd();
            }
            Ordering::Less => {
                println!("There are no results to display.");
            }
        };
    }
}

/// Create a ```TableWrapper``` object from a given vector of paths (```Vec<PathBuf>```).
fn create_table_from_paths(repos: Vec<PathBuf>, path: &Path) -> Option<TableWrapper> {
    // For every path, we will create a mutable Table containing its results.
    let mut table = prettytable::Table::new();
    table.set_format(
        prettytable::format::FormatBuilder::new()
            .column_separator(' ')
            .padding(0, 1)
            .build(),
    );

    // FIXME: handle all panic and fatal scenarios in this loop.
    for repo in repos {
        let repo_obj = git2::Repository::open(&repo).expect("failed to open");

        // This match cascade combats the error: remote 'origin' does not exist. If we
        // encounter this specific error, then we "continue" to the next iteration.
        let origin = match repo_obj.find_remote("origin") {
            Ok(origin) => origin,
            Err(error) if error.class() == git2::ErrorClass::Config => continue,
            Err(error) => panic!("{}", error),
        };
        let url = match origin.url() {
            Some(url) => url,
            None => "none",
        };
        let head = repo_obj.head().expect("failed get head");
        let branch = match head.shorthand() {
            Some(head) => head,
            None => "none",
        };

        // Special thanks to @yaahc_ for the original recommendation to use a "match guard" here.
        // The code has evolved since the original implementation, but the core idea still stands!
        let mut opts = git2::StatusOptions::new();
        let statuses = match repo_obj.statuses(Some(&mut opts)) {
            Ok(statuses) => Ok(Some(statuses)),
            Err(error)
                if error.code() == git2::ErrorCode::BareRepo
                    && error.class() == git2::ErrorClass::Repository =>
            {
                Ok(None)
            }
            Err(error) => Err(error),
        };

        let formatted_name = Path::new(&repo)
            .strip_prefix(path)
            .expect("failed to format name from Path object");
        let str_name = match formatted_name.to_str() {
            Some(x) => x,
            None => "none",
        };

        match statuses {
            Ok(statuses_contents) => match statuses_contents {
                Some(statuses_contents) if statuses_contents.is_empty() => {
                    table.add_row(row![Flb->str_name, Fgl->"clean", Fl->branch, Fl->url])
                }
                Some(_) => table.add_row(row![Flb->str_name, Fyl->"unclean", Fl->branch, Fl->url]),
                None => table.add_row(row![Flb->str_name, Frl->"bare", Fl->branch, Fl->url]),
            },
            Err(_) => table.add_row(row![Flb->str_name, Frl->"error", Fl->branch, Fl->url]),
        };
    }

    // After looping over all the paths, check if the table contains any rows. We perform this
    // check because we only want results for directories that contain Git repositories. Push
    // the resulting TableWrapper object after creating a heap-allocated string for the path name.
    if table.is_empty() {
        return None;
    }
    let path_string = path
        .to_str()
        .expect("could not convert &Path object to &str object");
    Some(TableWrapper {
        path_string: path_string.to_string(),
        table,
    })
}

/// This function is the primary driver for this file, ```lib.rs```.
pub fn run(path: &Path, recursive: bool, skip_sort: bool) -> Result<(), Box<dyn Error>> {
    let config = Config {
        recursive,
        skip_sort,
    };
    let results = Results::new(path, &config)?;
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
        assert_ne!(run(&current_dir, false, false).is_err(), true);
    }

    #[test]
    fn parent_directory() {
        let mut current_dir = env::current_dir().expect("failed to get CWD");
        current_dir.pop();
        assert_ne!(run(&current_dir, false, false).is_err(), true);
    }

    #[test]
    fn parent_directory_recursive() {
        let mut current_dir = env::current_dir().expect("failed to get CWD");
        current_dir.pop();
        assert_ne!(run(&current_dir, true, false).is_err(), true);
    }

    #[test]
    fn parent_directory_recursive_skip_sort() {
        let mut current_dir = env::current_dir().expect("failed to get CWD");
        current_dir.pop();
        assert_ne!(run(&current_dir, true, true).is_err(), true);
    }
}

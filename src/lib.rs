/*
 * gfold
 * https://github.com/nickgerace/gfold
 * Author: Nick Gerace
 * License: Apache 2.0
 */

//! This is a CLI tool to help keep track of your Git repositories.

#[macro_use]
extern crate prettytable;

use std::cmp::Ordering;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

use eyre::Result;
use log::debug;

#[derive(Debug)]
struct Config {
    no_color: bool,
    recursive: bool,
    skip_sort: bool,
}

struct TableWrapper {
    path_string: String,
    table: prettytable::Table,
}

struct Results(Vec<TableWrapper>);

impl Results {
    fn new(path: &Path, config: &Config) -> Result<Results> {
        let mut results = Results(Vec::new());
        results.execute_in_directory(&config, path)?;
        if !&config.skip_sort {
            results.sort_results();
        }
        Ok(results)
    }

    fn execute_in_directory(&mut self, config: &Config, dir: &Path) -> Result<()> {
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
            if let Some(table_wrapper) = create_table_from_paths(repos, &dir, &config.no_color) {
                self.0.push(table_wrapper);
            }
        }
        Ok(())
    }

    fn sort_results(&mut self) {
        if self.0.len() >= 2 {
            // FIXME: find a way to do this without "clone()".
            self.0.sort_by_key(|table| table.path_string.clone());
        }
    }

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

fn create_table_from_paths(
    repos: Vec<PathBuf>,
    path: &Path,
    no_color: &bool,
) -> Option<TableWrapper> {
    let mut table = prettytable::Table::new();
    table.set_format(
        prettytable::format::FormatBuilder::new()
            .column_separator(' ')
            .padding(0, 1)
            .build(),
    );

    // FIXME: maximize error recovery in this loop.
    for repo in repos {
        debug!("Creating row from path: {:#?}", repo);
        let repo_obj = git2::Repository::open(&repo).ok()?;

        // FIXME: in case deeper recoverable errors are desired, use the match arm...
        // Err(error) if error.class() == git2::ErrorClass::Config => continue,
        let origin = match repo_obj.find_remote("origin") {
            Ok(origin) => origin,
            Err(_) => continue,
        };
        let url = match origin.url() {
            Some(url) => url,
            None => "none",
        };

        let head = repo_obj.head().ok()?;
        let branch = match head.shorthand() {
            Some(branch) => branch,
            None => "none",
        };

        let str_name = match Path::new(&repo).strip_prefix(path).ok()?.to_str() {
            Some(x) => x,
            None => "none",
        };

        if repo_obj.is_bare() {
            if *no_color {
                table.add_row(row![Fl->str_name, Fl->"bare", Fl->branch, Fl->url]);
            } else {
                table.add_row(row![Flb->str_name, Frl->"bare", Fl->branch, Fl->url]);
            }
        } else {
            let mut opts = git2::StatusOptions::new();
            match repo_obj.statuses(Some(&mut opts)) {
                Ok(statuses) if statuses.is_empty() => {
                    if is_unpushed(&repo_obj, &head).ok()? {
                        if *no_color {
                            table.add_row(row![Fl->str_name, Fl->"unpushed", Fl->branch, Fl->url])
                        } else {
                            table.add_row(row![Flb->str_name, Fcl->"unpushed", Fl->branch, Fl->url])
                        }
                    } else {
                        if *no_color {
                            table.add_row(row![Fl->str_name, Fl->"clean", Fl->branch, Fl->url])
                        } else {
                            table.add_row(row![Flb->str_name, Fgl->"clean", Fl->branch, Fl->url])
                        }
                    }
                }
                Ok(_) => {
                    if *no_color {
                        table.add_row(row![Fl->str_name, Fl->"unclean", Fl->branch, Fl->url])
                    } else {
                        table.add_row(row![Flb->str_name, Fyl->"unclean", Fl->branch, Fl->url])
                    }
                }
                Err(_) => {
                    if *no_color {
                        table.add_row(row![Fl->str_name, Fl->"error", Fl->branch, Fl->url])
                    } else {
                        table.add_row(row![Flb->str_name, Frl->"error", Fl->branch, Fl->url])
                    }
                }
            };
        }
    }

    match table.is_empty() {
        true => None,
        false => Some(TableWrapper {
            path_string: path.to_str()?.to_string(),
            table,
        }),
    }
}

// FIXME: add an -o/--offline flag if this function is changed to connect to the remote.
fn is_unpushed(repo: &git2::Repository, head: &git2::Reference) -> Result<bool> {
    let local = head.peel_to_commit()?;
    debug!("Local commit: {:#?}", local.id());

    let upstream = repo
        .resolve_reference_from_short_name("origin")?
        .peel_to_commit()?;
    debug!("Origin commit: {:#?}", upstream.id());

    match repo.graph_ahead_behind(local.id(), upstream.id())? {
        ahead if ahead.0 > 0 => Ok(true),
        _ => Ok(false),
    }
}

/// This function is the primary, backend driver for `gfold`.
///
/// - `path`: the target path to find and parse Git repositories
/// - `no_color`: disables color, bolding, etc.
/// - `recursive`: recursively searches directories for Git repositories
/// - `skip_sort`: skips sorting the repositories for output
///
/// When executed, results will be printed to STDOUT.
pub fn run(path: &Path, no_color: bool, recursive: bool, skip_sort: bool) -> Result<()> {
    let config = Config {
        no_color,
        recursive,
        skip_sort,
    };
    debug!("Running with path: {:#?}", path);
    debug!("Running with config: {:#?}", &config);

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
        assert_ne!(run(&current_dir, false, false, false).is_err(), true);
    }

    #[test]
    fn parent_directory() {
        let mut current_dir = env::current_dir().expect("failed to get CWD");
        current_dir.pop();
        assert_ne!(run(&current_dir, false, false, false).is_err(), true);
    }

    #[test]
    fn parent_directory_all_options() {
        let mut current_dir = env::current_dir().expect("failed to get CWD");
        current_dir.pop();

        assert_ne!(run(&current_dir, true, false, false).is_err(), true);
        assert_ne!(run(&current_dir, true, false, true).is_err(), true);
        assert_ne!(run(&current_dir, true, true, false).is_err(), true);

        assert_ne!(run(&current_dir, false, true, false).is_err(), true);
        assert_ne!(run(&current_dir, false, true, true).is_err(), true);
        assert_ne!(run(&current_dir, false, false, true).is_err(), true);

        assert_ne!(run(&current_dir, true, true, true).is_err(), true);
    }
}

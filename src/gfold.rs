/*
 * gfold
 * https://github.com/nickgerace/gfold
 * Author: Nick Gerace
 * License: Apache 2.0
 */

use git2::{ErrorClass, ErrorCode, Repository, StatusOptions};
use prettytable::{format, Table};

use std::fs;
use std::path::Path;

use crate::util::is_git_repo;

struct TableWrapper {
    path_string: String,
    table: Table,
}

struct Results {
    recursive: bool,
    tables: Vec<TableWrapper>,
}

impl Results {
    fn print_results(self) {
        if self.recursive && self.tables.len() > 1 {
            for table_wrapper in self.tables {
                println!("\n{}", table_wrapper.path_string);
                table_wrapper.table.printstd();
            }
        } else if self.tables.len() == 1 {
            self.tables[0].table.printstd();
        } else {
            println!("There are no results to display.");
        }
    }

    fn sort_results(&mut self) {
        // FIXME: find a way to do this without "clone()".
        self.tables.sort_by_key(|table| table.path_string.clone());
    }

    fn execute_on_path(&mut self, path: &Path) {
        // FIXME: find ways to add concurrent programming (tokio, async, etc.) to this section.
        // In such implementation, sort the results at the end after concurrent operations conclude.
        let path_entries = fs::read_dir(path).expect("failed to get sub directories");
        let mut repos = Vec::new();

        for entry in path_entries {
            let subpath = &entry.expect("failed to get DirEntry").path();
            if subpath.is_dir() {
                if is_git_repo(subpath) {
                    repos.push(subpath.to_owned());
                } else if self.recursive {
                    self.execute_on_path(&subpath);
                }
            }
        }
        if repos.is_empty() {
            return;
        }

        // Alphabetically sort the repository paths, and create a mutable Table object. For
        // every path, we will create a Table containing its results.
        repos.sort();
        let mut table = Table::new();
        table.set_format(
            format::FormatBuilder::new()
                .column_separator(' ')
                .padding(0, 1)
                .build(),
        );

        for repo in repos {
            let repo_obj = Repository::open(&repo).expect("failed to open");

            // This match cascade combats the error: remote 'origin' does not exist. If we
            // encounter this specific error, then we "continue" to the next iteration.
            let origin = match repo_obj.find_remote("origin") {
                Ok(origin) => origin,
                Err(error) if error.class() == ErrorClass::Config => continue,
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

            // If the repository is bare, then we return "None". This addresses GitHub issue #11
            // (https://github.com/nickgerace/gfold/issues/11), and special thanks to @yaahc_ for
            // the recommendation to use a "match guard" here. We also use the Option type instead
            // to handle the "None" case later.
            let mut opts = StatusOptions::new();
            let statuses = match repo_obj.statuses(Some(&mut opts)) {
                Ok(statuses) => Some(statuses),
                Err(error)
                    if error.code() == ErrorCode::BareRepo
                        && error.class() == ErrorClass::Repository =>
                {
                    None
                }
                Err(error) => panic!("failed to get statuses: {}", error),
            };

            let formatted_name = Path::new(&repo)
                .strip_prefix(path)
                .expect("failed to format name from Path object");
            let str_name = match formatted_name.to_str() {
                Some(x) => x,
                None => "none",
            };

            match statuses {
                Some(statuses) if statuses.is_empty() => {
                    table.add_row(row![Flb->str_name, Fgl->"clean", Fl->branch, Fl->url])
                }
                Some(_) => table.add_row(row![Flb->str_name, Fyl->"unclean", Fl->branch, Fl->url]),
                None => table.add_row(row![Flb->str_name, Frl->"bare", Fl->branch, Fl->url]),
            };
        }

        // Only perform the following actions if the Table object is not empty. We only want
        // results for directories that contain repositories. Push the resulting TableWrapper
        // object aftering creating a heap-allocated string for the path name.
        if !table.is_empty() {
            let path_string = path
                .to_str()
                .expect("could not convert &Path object to &str object");
            let table_wrapper = TableWrapper {
                path_string: path_string.to_string(),
                table: table,
            };
            self.tables.push(table_wrapper);
        }
    }
}

pub fn harness(path: &Path, recursive: bool, skip_sort: bool) {
    let mut results = Results {
        recursive: recursive,
        tables: Vec::new(),
    };
    results.execute_on_path(&path);
    if !skip_sort {
        results.sort_results();
    }
    results.print_results();
}

#[cfg(test)]
mod tests {
    use super::harness;
    use std::env::current_dir;

    #[test]
    fn current_directory() {
        let current_dir = current_dir().expect("failed to get CWD");
        harness(&current_dir, false, false);
    }

    #[test]
    fn parent_directory() {
        let mut current_dir = current_dir().expect("failed to get CWD");
        current_dir.pop();
        harness(&current_dir, false, false);
    }

    #[test]
    fn parent_directory_recursive() {
        let mut current_dir = current_dir().expect("failed to get CWD");
        current_dir.pop();
        harness(&current_dir, true, false);
    }

    #[test]
    fn parent_directory_recursive_skip_sort() {
        let mut current_dir = current_dir().expect("failed to get CWD");
        current_dir.pop();
        harness(&current_dir, true, true);
    }
}

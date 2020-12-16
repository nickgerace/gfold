/*
 * gfold
 * https://github.com/nickgerace/gfold
 * Author: Nick Gerace
 * License: Apache 2.0
 */

use crate::util;

use std::cmp::Ordering;
use std::fs;
use std::path::Path;

use eyre::Result;
use log::debug;

#[derive(Debug)]
pub struct Config {
    pub disable_unpushed_check: bool,
    pub no_color: bool,
    pub recursive: bool,
    pub skip_sort: bool,
}

pub struct TableWrapper {
    pub path_string: String,
    pub table: prettytable::Table,
}

pub struct Results(Vec<TableWrapper>);

impl Results {
    pub fn new(path: &Path, config: &Config) -> Result<Results> {
        debug!("Running with config: {:#?}", &config);
        debug!("Running in path: {:#?}", &path);
        let mut results = Results(Vec::new());
        results.execute_in_directory(&config, path)?;
        if !&config.skip_sort {
            results.sort_results();
        }
        Ok(results)
    }

    pub fn print_results(self) {
        debug!("Printing results with {} tables...", self.0.len());
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
            _ => {}
        };
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
                    debug!("Recursive execution into directory: {:#?}", &subpath);
                    self.execute_in_directory(&config, &subpath)?;
                }
            }
        }

        debug!("Git repositories found: {:#?}", repos);
        if !repos.is_empty() {
            if !&config.skip_sort {
                repos.sort();
            }
            if let Some(table_wrapper) = util::create_table_from_paths(
                repos,
                &dir,
                &config.disable_unpushed_check,
                &config.no_color,
            ) {
                self.0.push(table_wrapper);
            }
        }
        Ok(())
    }

    fn sort_results(&mut self) {
        debug!("Sorting {:#?} tables...", self.0.len());
        if self.0.len() >= 2 {
            // FIXME: find a way to do this without "clone()".
            self.0.sort_by_key(|table| table.path_string.clone());
        }
    }
}

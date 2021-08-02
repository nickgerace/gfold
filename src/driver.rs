//! This module contains the types required for generating results for `gfold`.
use crate::{driver_internal::TableWrapper, util};
use ansi_term::Style;
use anyhow::{anyhow, Result};
use std::{
    cmp::Ordering,
    fs, io,
    path::{Path, PathBuf},
};

/// A bedrock type that is a required parameter when creating a new `Driver`.
#[derive(Debug)]
pub struct Config {
    /// Enable checking for unpushed commits (experimental).
    pub enable_unpushed_check: bool,
    /// Include standard directories in the result.
    pub include_non_repos: bool,
    /// Disables color, bolding, etc.
    pub no_color: bool,
    /// The target path to find and parse Git repositories.
    pub shallow: bool,
    /// Displays the email address corresponding to the local Git config (same as `git config user.email`).
    pub show_email: bool,
    /// Skips sorting the repositories for output
    pub skip_sort: bool,
}

/// Creating this object with a given `Config` will generate results that can be printed to `STDOUT`.
pub struct Driver {
    tables: Vec<TableWrapper>,
    config: Config,
}

impl Driver {
    /// Constructing a `Driver` will generate results with a given `&Path` and `&Config`.
    pub fn new(path: &Path, config: Config) -> Result<Driver> {
        let mut driver = Driver {
            tables: Vec::new(),
            config,
        };
        driver.execute_in_directory(path)?;
        if !driver.config.skip_sort {
            driver.sort_results();
        }
        Ok(driver)
    }

    /// Print results to `STDOUT` after generation.
    pub fn print_results(self) -> Result<()> {
        #[cfg(windows)]
        if !self.config.no_color {
            ansi_term::enable_ansi_support();
        }

        match self.tables.len().cmp(&1) {
            Ordering::Greater => {
                let last = match self.tables.last() {
                    Some(s) => s.path_string.clone(),
                    None => return Err(anyhow!("Last object not found for table vector")),
                };
                for table_wrapper in self.tables {
                    match self.config.no_color {
                        false => {
                            println!("{}", Style::new().bold().paint(&table_wrapper.path_string))
                        }
                        true => println!("{}", &table_wrapper.path_string),
                    }
                    table_wrapper.table.printstd();
                    if table_wrapper.path_string != last {
                        println!();
                    }
                }
            }
            Ordering::Equal => {
                self.tables[0].table.printstd();
            }
            _ => {}
        };
        Ok(())
    }

    // Sequential exeuction has benchmarked faster than concurrent implementations.
    fn execute_in_directory(&mut self, dir: &Path) -> Result<()> {
        let mut repos: Vec<PathBuf> = Vec::new();
        let mut non_repos: Vec<PathBuf> = Vec::new();

        match fs::read_dir(dir) {
            Ok(o) => {
                for entry in o.flatten() {
                    let file_name_buf = entry.file_name();
                    let file_name = match file_name_buf.to_str() {
                        Some(o) => o,
                        None => continue,
                    };
                    if !file_name.starts_with('.') && entry.file_type()?.is_dir() {
                        let entry_path = entry.path();
                        match git2::Repository::open(&entry_path) {
                            Ok(_) => repos.push(entry_path),
                            Err(_) => {
                                if self.config.include_non_repos {
                                    non_repos.push(entry_path.clone());
                                }
                                if !self.config.shallow {
                                    self.execute_in_directory(&entry_path)?;
                                }
                            }
                        }
                    }
                }
            }
            Err(e) if e.kind() == io::ErrorKind::PermissionDenied => {
                println!("Permission denied: {}", dir.display())
            }
            Err(e) => return Err(e.into()),
        }

        if !repos.is_empty() {
            if !self.config.skip_sort {
                repos.sort();
            }
            if let Some(table_wrapper) = util::create_table_from_paths(
                repos,
                non_repos,
                dir,
                &self.config.enable_unpushed_check,
                &self.config.no_color,
                &self.config.show_email,
            ) {
                self.tables.push(table_wrapper);
            }
        }
        Ok(())
    }

    fn sort_results(&mut self) {
        if self.tables.len() >= 2 {
            // FIXME: find a way to do this without cloning.
            self.tables.sort_by_key(|table| table.path_string.clone());
        }
    }
}

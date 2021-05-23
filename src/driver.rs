//! This module contains the types required for generating results for `gfold`.
use crate::{driver_internal::TableWrapper, util};
use ansi_term::Style;
use anyhow::Result;
use log::{debug, error, warn};
use std::{
    cmp::Ordering,
    fs,
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
pub struct Driver(Vec<TableWrapper>);

impl Driver {
    /// Constructing a `Driver` will generate results with a given `&Path` and `&Config`.
    pub fn new(path: &Path, config: &Config) -> Result<Driver> {
        debug!("Running with config: {:#?}", &config);
        debug!("Running in path: {:#?}", &path);
        let mut driver = Driver(Vec::new());
        driver.execute_in_directory(&config, path)?;
        if !&config.skip_sort {
            driver.sort_results();
        }
        Ok(driver)
    }

    /// Print results to `STDOUT` after generation.
    pub fn print_results(self) {
        #[cfg(windows)]
        ansi_term::enable_ansi_support();

        debug!("Printing results with {} tables...", self.0.len());
        match self.0.len().cmp(&1) {
            Ordering::Greater => {
                let last = match self.0.last() {
                    Some(s) => s.path_string.clone(),
                    None => {
                        error!(
                            "Last object not found for table vector. Continuing with empty string."
                        );
                        String::from("")
                    }
                };
                for table_wrapper in self.0 {
                    println!("{}", Style::new().bold().paint(&table_wrapper.path_string));
                    table_wrapper.table.printstd();
                    if table_wrapper.path_string != last {
                        println!();
                    }
                }
            }
            Ordering::Equal => {
                self.0[0].table.printstd();
            }
            _ => {}
        };
    }

    // Sequential exeuction has benchmarked faster than concurrent implementations.
    fn execute_in_directory(&mut self, config: &Config, dir: &Path) -> Result<()> {
        let mut repos: Vec<PathBuf> = Vec::new();
        let mut non_repos: Vec<PathBuf> = Vec::new();

        for entry in (fs::read_dir(dir)?).flatten() {
            let file_name_buf = entry.file_name();
            let file_name = match file_name_buf.to_str() {
                Some(o) => o,
                None => continue,
            };
            if !file_name.starts_with('.') && entry.file_type()?.is_dir() {
                let entry_path = entry.path();
                match git2::Repository::open(&entry_path) {
                    Ok(_) => repos.push(entry_path),
                    Err(e) => {
                        debug!(
                            "Tried to open {:#?} as git repository: {:#?}",
                            entry_path,
                            e.message()
                        );
                        if config.include_non_repos {
                            non_repos.push(entry_path.clone());
                        }
                        if !config.shallow {
                            if let Err(e) = self.execute_in_directory(&config, &entry_path) {
                                warn!(
                                    "Encountered error during recursive walk into {:#?}: {:#?}",
                                    &entry_path, e
                                );
                            }
                        }
                    }
                }
            }
        }

        debug!("Git repositories found: {:#?}", repos);
        if config.include_non_repos {
            debug!("Standard directories found: {:#?}", non_repos);
        }
        if !repos.is_empty() {
            if !&config.skip_sort {
                repos.sort();
            }
            if let Some(table_wrapper) = util::create_table_from_paths(
                repos,
                non_repos,
                &dir,
                &config.enable_unpushed_check,
                &config.no_color,
                &config.show_email,
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

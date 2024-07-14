//! This module contains the functionality for displaying reports to `stdout`.

use libgfold::RepositoryCollection;
use log::warn;
use std::path::{Path, PathBuf};
use thiserror::Error;
use writer::DisplayWriter;

use crate::config::ColorMode;
use crate::config::JsonOptions;

mod writer;

const PAD: usize = 2;
const NONE: &str = "none";

#[remain::sorted]
#[derive(Error, Debug)]
pub enum DisplayError {
    #[error("could not convert path (Path) to &str: {0}")]
    PathToStrConversionFailure(PathBuf),
}

/// This struct is used for displaying the contents of a [`RepositoryCollection`] to `stdout`.
pub struct DisplayHarness;

impl DisplayHarness {
    pub fn run(
        color_mode: ColorMode,
        reports: &RepositoryCollection,
        json: JsonOptions,
        alphabetical: bool,
        sort_status: bool,
        group_by_parent_directory: bool,
    ) -> anyhow::Result<()> {
        let display_writer = DisplayWriter::new(color_mode);

        let (json, json_raw) = match json {
            JsonOptions::False => (false, false),
            JsonOptions::Pretty => (true, false),
            JsonOptions::Raw => (true, true),
        };

        if json {
            let mut all_reports = Vec::new();
            for grouped_report in reports {
                all_reports.append(&mut grouped_report.1.clone());
            }
            if alphabetical {
                all_reports.sort_by(|a, b| a.name.cmp(&b.name));
            }
            if sort_status {
                all_reports.sort_by(|a, b| a.status.as_str().cmp(b.status.as_str()));
            }
            let json = if json_raw {
                serde_json::to_string(&all_reports)?
            } else {
                serde_json::to_string_pretty(&all_reports)?
            };
            display_writer.write(json, true, None);
            return Ok(());
        }

        if group_by_parent_directory {
            let length = reports.keys().len();
            let mut first = true;
            for (title, group) in reports {
                // FIXME(nick): make group title matching less cumbersome.
                if length > 1 {
                    match first {
                        true => {
                            first = false;
                        }
                        false => println!(),
                    }
                    display_writer.write_bold(
                        match &title {
                            Some(s) => s,
                            None => NONE,
                        },
                        true,
                    )?;
                }

                let mut name_max = 0;
                let mut branch_max = 0;
                let mut status_max = 0;
                for report in group {
                    if report.name.len() > name_max {
                        name_max = report.name.len();
                    }
                    let status_length = report.status.as_str().len();
                    if status_length > status_max {
                        status_max = status_length;
                    }
                    if report.branch.len() > branch_max {
                        branch_max = report.branch.len();
                    }
                }

                let mut reports = group.clone();
                if alphabetical {
                    reports.sort_by(|a, b| a.name.cmp(&b.name));
                }
                if sort_status {
                    reports.sort_by(|a, b| a.status.as_str().cmp(b.status.as_str()));
                }

                for report in reports {
                    display_writer.write(report.name, false, Some(name_max + PAD));
                    display_writer.write_status(report.status, status_max + PAD)?;
                    display_writer.write(report.branch, false, Some(branch_max + PAD));
                    display_writer.write(report.url.unwrap_or(NONE.to_string()), true, None);
                }
            }
        } else {
            let mut all_reports = Vec::new();
            for grouped_report in reports {
                all_reports.append(&mut grouped_report.1.clone());
            }

            if alphabetical {
                all_reports.sort_by(|a, b| a.name.cmp(&b.name));
            }
            if sort_status {
                all_reports.sort_by(|a, b| a.status.as_str().cmp(b.status.as_str()));
            }

            for report in all_reports {
                display_writer.write_bold(&report.name, false)?;

                let parent = match report.parent {
                    Some(s) => s,
                    None => {
                        warn!("parent is empty for collector: {}", report.name);
                        continue;
                    }
                };
                let full_path = Path::new(&parent).join(&report.name);
                let full_path_formatted = format!(
                    " ~ {}",
                    full_path.to_str().ok_or_else(|| {
                        DisplayError::PathToStrConversionFailure(full_path.clone())
                    })?
                );
                display_writer.write_gray(&full_path_formatted, true)?;

                print!("  ");
                display_writer.write_status(report.status, PAD)?;
                println!(" ({})", report.branch);
                if let Some(url) = &report.url {
                    println!("  {url}");
                }
                if let Some(email) = &report.email {
                    println!("  {email}");
                }
            }
        }

        Ok(())
    }
}

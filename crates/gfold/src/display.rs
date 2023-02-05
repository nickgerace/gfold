//! This module contains the functionality for displaying reports to `stdout`.

use color::ColorHarness;
use log::debug;
use log::warn;
use std::io;
use std::path::{Path, PathBuf};
use thiserror::Error;

use crate::collector::RepositoryCollection;
use crate::config::{ColorMode, DisplayMode};

mod color;

const PAD: usize = 2;
const NONE: &str = "none";

#[derive(Error, Debug)]
pub enum DisplayError {
    #[error("could not convert path (Path) to &str: {0}")]
    PathToStrConversionFailure(PathBuf),
}

/// This struct is used for displaying the contents of a [`RepositoryCollection`] to `stdout`.
pub struct DisplayHarness {
    display_mode: DisplayMode,
    color_mode: ColorMode,
}

impl DisplayHarness {
    pub fn new(display_mode: DisplayMode, color_mode: ColorMode) -> Self {
        Self {
            display_mode,
            color_mode,
        }
    }

    /// This function chooses the display execution function based on the [`DisplayMode`] provided.
    pub fn run(&self, reports: &RepositoryCollection) -> anyhow::Result<()> {
        match self.display_mode {
            DisplayMode::Standard => Self::standard(reports, self.color_mode)?,
            DisplayMode::Json => Self::json(reports)?,
            DisplayMode::Classic => Self::classic(reports, self.color_mode)?,
        }
        Ok(())
    }

    /// Display [`RepositoryCollection`] to `stdout` in the standard (default) format.
    fn standard(reports: &RepositoryCollection, color_mode: ColorMode) -> anyhow::Result<()> {
        debug!("detected standard display mode");
        let mut all_reports = Vec::new();
        for grouped_report in reports {
            all_reports.append(&mut grouped_report.1.clone());
        }
        all_reports.sort_by(|a, b| a.name.cmp(&b.name));
        all_reports.sort_by(|a, b| a.status.as_str().cmp(b.status.as_str()));

        let color_harness = ColorHarness::new(&color_mode);

        for report in all_reports {
            color_harness.write_bold(&report.name, false)?;

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
                full_path
                    .to_str()
                    .ok_or_else(|| DisplayError::PathToStrConversionFailure(full_path.clone()))?
            );
            color_harness.write_gray(&full_path_formatted, true)?;

            print!("  ");
            color_harness.write_status(&report.status, PAD)?;
            println!(" ({})", report.branch);
            if let Some(url) = &report.url {
                println!("  {url}");
            }
            if let Some(email) = &report.email {
                println!("  {email}");
            }
        }
        Ok(())
    }

    /// Display [`RepositoryCollection`] to `stdout` in JSON format.
    fn json(reports: &RepositoryCollection) -> serde_json::Result<()> {
        debug!("detected json display mode");
        let mut all_reports = Vec::new();
        for grouped_report in reports {
            all_reports.append(&mut grouped_report.1.clone());
        }
        all_reports.sort_by(|a, b| a.name.cmp(&b.name));
        all_reports.sort_by(|a, b| a.status.as_str().cmp(b.status.as_str()));
        println!("{}", serde_json::to_string_pretty(&all_reports)?);
        Ok(())
    }

    /// Display [`RepositoryCollection`] to `stdout` in the classic format.
    fn classic(reports: &RepositoryCollection, color_mode: ColorMode) -> io::Result<()> {
        debug!("detected classic display mode");
        let color_harness = ColorHarness::new(&color_mode);

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
                color_harness.write_bold(
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
            reports.sort_by(|a, b| a.name.cmp(&b.name));
            reports.sort_by(|a, b| a.status.as_str().cmp(b.status.as_str()));

            for report in reports {
                print!("{:<path_width$}", report.name, path_width = name_max + PAD);
                color_harness.write_status(&report.status, status_max + PAD)?;
                println!(
                    "{:<branch_width$}{}",
                    report.branch,
                    match &report.url {
                        Some(s) => s,
                        None => NONE,
                    },
                    branch_width = branch_max + PAD
                );
            }
        }
        Ok(())
    }
}

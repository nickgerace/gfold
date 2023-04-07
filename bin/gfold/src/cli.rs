//! This module contains the CLI entrypoint, CLI options and config generation based on the user's
//! settings and environment.

use clap::Parser;
use libgfold::RepositoryCollector;
use log::debug;
use std::env;
use thiserror::Error;

use crate::config::{ColorMode, Config, DisplayMode};
use crate::display::DisplayHarness;

const HELP: &str = "\
More information: https://github.com/nickgerace/gfold

Description:
  This application helps you keep track of multiple Git repositories via CLI.
  By default, it displays relevant information for all repos in the current
  working directory.

Config File Usage:
  While CLI options are prioritized, default options will fallback to the
  config file if it exists. Here is the config file lookup locations for some
  common platforms:

    macOS, Linux, etc.    $HOME/.config/gfold.toml
    Windows               {{FOLDERID_Profile}}\\.config\\gfold.toml

Troubleshooting:
  Investigate unexpected behavior by prepending execution with
  \"RUST_BACKTRACE=1\"and \"RUST_LOG=debug\". You can adjust those variable's
  values to aid investigation.";

#[derive(Error, Debug)]
pub enum CliError {
    #[error("invalid color mode provided (exec \"--help\" for options): {0}")]
    InvalidColorMode(String),
    #[error("invalid display mode provided (exec \"--help\" for options): {0}")]
    InvalidDisplayMode(String),
}

#[derive(Parser)]
#[command(version, about = HELP, long_about = None)]
struct Cli {
    #[arg(help = "specify path to target directory (defaults to current working directory)")]
    path: Option<String>,

    #[arg(
        short,
        long,
        help = "specify color mode (options: [\"always\", \"compatibility\", \"never\"])"
    )]
    color_mode: Option<String>,
    #[arg(
        short,
        long,
        help = "specify display format (options: [\"standard\", \"default\", \"json\", \"classic\"])"
    )]
    display_mode: Option<String>,
    #[arg(
        long,
        help = "display finalized config options and exit (merged options from an optional config file and command line arguments)"
    )]
    dry_run: bool,
    #[arg(short, long, help = "ignore config file settings")]
    ignore_config_file: bool,
}

pub struct CliHarness {
    cli: Cli,
}

impl CliHarness {
    /// Parse CLI arguments and store the result on the [`self`](Self).
    pub fn new() -> Self {
        let cli = Cli::parse();
        debug!("collected args");
        Self { cli }
    }

    /// Merge configurations as needed, collect results and display them.
    pub fn run(&self) -> anyhow::Result<()> {
        let mut config = match self.cli.ignore_config_file {
            true => Config::try_config_default()?,
            false => Config::try_config()?,
        };
        debug!("loaded initial config");

        if let Some(found_display_mode) = &self.cli.display_mode {
            config.display_mode = match found_display_mode.to_lowercase().as_str() {
                "classic" => DisplayMode::Classic,
                "json" => DisplayMode::Json,
                "standard" | "default" => DisplayMode::Standard,
                _ => {
                    return Err(CliError::InvalidDisplayMode(found_display_mode.to_string()).into())
                }
            }
        }

        if let Some(found_color_mode) = &self.cli.color_mode {
            config.color_mode = match found_color_mode.to_lowercase().as_str() {
                "always" => ColorMode::Always,
                "compatibility" => ColorMode::Compatibility,
                "never" => ColorMode::Never,
                _ => return Err(CliError::InvalidColorMode(found_color_mode.to_string()).into()),
            }
        }

        if let Some(found_path) = &self.cli.path {
            config.path = env::current_dir()?.join(found_path).canonicalize()?;
        }

        debug!("finalized config options");
        match &self.cli.dry_run {
            true => config.print()?,
            false => {
                let (include_email, include_submodules) = match config.display_mode {
                    DisplayMode::Classic => (false, false),
                    DisplayMode::Json => (true, true),
                    DisplayMode::Standard => (true, false),
                };
                let repository_collection =
                    RepositoryCollector::run(&config.path, include_email, include_submodules)?;
                let display_harness = DisplayHarness::new(config.display_mode, config.color_mode);
                display_harness.run(&repository_collection)?;
            }
        }
        Ok(())
    }
}

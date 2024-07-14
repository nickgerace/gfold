//! This module contains the CLI entrypoint, CLI options and config generation based on the user's
//! settings and environment.

use clap::Parser;
use libgfold::RepositoryCollector;
use log::debug;
use std::env;
use thiserror::Error;

use crate::config::{ColorMode, Config, JsonOptions};
use crate::display::DisplayHarness;

const HELP: &str = "\
More information: https://github.com/nickgerace/gfold

Description:
  This application helps you keep track of multiple Git repositories via CLI.
  By default, it displays relevant information for all repos in the current
  working directory.

Config File Usage:
  While CLI options are prioritized, default options will fallback to a config
  file if one exists. Here is the config file lookup locations for common
  platforms:

    macOS and Linux    $HOME/.config/gfold.toml
    Windows            {{FOLDERID_Profile}}\\.config\\gfold.toml

Troubleshooting:
  Investigate unexpected behavior by prepending execution with
  \"RUST_BACKTRACE=1\"and \"RUST_LOG=debug\". You can adjust the values for
  both environment variables, as needed.";

#[remain::sorted]
#[derive(Error, Debug)]
pub enum CliError {
    #[error("invalid color mode provided (use \"--help\" for options): {0}")]
    InvalidColorMode(String),
    #[error("invalid json option provided (use \"--help\" for options): {0}")]
    InvalidJsonOption(String),
}

#[derive(Parser)]
#[command(version, about = HELP, long_about = None)]
struct Cli {
    #[arg(help = "specify path to target directory (defaults to current working directory)")]
    path: Option<String>,

    // --------------------------------
    #[arg(
        short,
        long,
        help = "specify color mode (options: [\"always\", \"compatibility\", \"never\"])"
    )]
    color_mode: Option<String>,
    #[arg(short, long)]
    json: Option<String>,

    // --------------------------------
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

        if let Some(found_color_mode) = &self.cli.color_mode {
            config.color_mode = match found_color_mode.to_lowercase().as_str() {
                "always" => ColorMode::Always,
                "compatibility" => ColorMode::Compatibility,
                "never" => ColorMode::Never,
                _ => return Err(CliError::InvalidColorMode(found_color_mode.to_string()).into()),
            }
        }
        if let Some(found_json) = &self.cli.json {
            config.json = match found_json.to_lowercase().as_str() {
                "false" => JsonOptions::False,
                "pretty" => JsonOptions::Pretty,
                "raw" => JsonOptions::Raw,
                _ => return Err(CliError::InvalidJsonOption(found_json.to_string()).into()),
            }
        }

        if let Some(found_path) = &self.cli.path {
            config.path = env::current_dir()?.join(found_path).canonicalize()?;
        }

        debug!("finalized config options");
        match &self.cli.dry_run {
            true => config.print()?,
            false => {
                let repository_collection = RepositoryCollector::run(
                    &config.path,
                    config.include_email,
                    config.include_submodules,
                    config.parallel,
                )?;
                DisplayHarness::run(
                    config.color_mode,
                    &repository_collection,
                    config.json,
                    config.alphabetical,
                    config.sort_status,
                    config.group_by_parent_directory,
                )?;
            }
        }
        Ok(())
    }
}

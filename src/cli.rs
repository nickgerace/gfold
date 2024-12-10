//! This module contains the CLI entrypoint, CLI options and config generation based on the user's
//! settings and environment.

use clap::Parser;
use gfold::RepositoryCollector;
use log::debug;
use std::env;

use crate::config::{ColorMode, Config, DisplayMode};
use crate::display::DisplayHarness;

const HELP: &str = "\
More information: https://github.com/nickgerace/gfold

Description: this application helps you keep track of multiple Git repositories via CLI. By default, it displays relevant information for all repos in the current working directory.

Config File Usage: while CLI options are prioritized, default options will fallback to the config file if it exists. Here are the config file lookup locations:

    $XDG_CONFIG_HOME/gfold.toml
    $XDG_CONFIG_HOME/gfold/config.toml
    $HOME/.config/gfold.toml (or {{FOLDERID_Profile}}\\.config\\gfold.toml on Windows)

Troubleshooting: investigate unexpected behavior by prepending execution with \"RUST_BACKTRACE=1\"and \"RUST_LOG=debug\". You can adjust those variable's values to aid investigation.";

#[derive(Parser)]
#[command(version, about = HELP, long_about = None)]
struct Cli {
    /// specify path to target directory (defaults to current working directory)
    path: Option<String>,

    #[arg(short, long)]
    color_mode: Option<ColorMode>,
    #[arg(short, long)]
    display_mode: Option<DisplayMode>,

    /// display finalized config options and exit (merged options from an optional config file and command line arguments)
    #[arg(long)]
    dry_run: bool,
    /// ignore config file settings
    #[arg(short, long)]
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
        let mut config = if self.cli.ignore_config_file {
            Config::try_config_default()?
        } else {
            Config::try_config()?
        };
        debug!("loaded initial config");

        if let Some(found_display_mode_raw) = &self.cli.display_mode {
            config.display_mode = *found_display_mode_raw;
        }

        if let Some(found_color_mode) = &self.cli.color_mode {
            config.color_mode = *found_color_mode;
        }

        if let Some(found_path) = &self.cli.path {
            config.path = env::current_dir()?.join(found_path).canonicalize()?;
        }

        debug!("finalized config options");
        if self.cli.dry_run {
            config.print()?;
        } else {
            let (include_email, include_submodules) = match config.display_mode {
                DisplayMode::Classic => (false, false),
                DisplayMode::Json => (true, true),
                DisplayMode::Standard | DisplayMode::StandardAlphabetical => (true, false),
            };
            let repository_collection =
                RepositoryCollector::run(&config.path, include_email, include_submodules)?;
            let display_harness = DisplayHarness::new(config.display_mode, config.color_mode);
            display_harness.run(&repository_collection)?;
        }
        Ok(())
    }
}

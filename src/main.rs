//! [gfold](https://github.com/nickgerace/gfold) is a CLI tool that helps you keep track of
//! multiple Git repositories. The source code uses private modules rather than leveraging
//! a library via `lib.rs`.

#![warn(missing_docs, clippy::missing_errors_doc, clippy::missing_panics_doc)]

use crate::config::{ColorMode, Config, DisplayMode};
use crate::display::DisplayHarness;
use gfold::RepositoryCollector;

use clap::Parser;
use env_logger::Builder;
use log::{debug, LevelFilter};
use std::env;

mod config;
mod display;

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
    pub path: Option<String>,

    #[arg(short, long)]
    pub color_mode: Option<ColorMode>,
    #[arg(short, long)]
    pub display_mode: Option<DisplayMode>,

    /// display finalized config options and exit (merged options from an optional config file and command line arguments)
    #[arg(long)]
    pub dry_run: bool,
    /// ignore config file settings
    #[arg(short, long)]
    pub ignore_config_file: bool,
}

/// Initializes the logger based on the debug flag and `RUST_LOG` environment variable, then
/// parses CLI arguments and generates a [`Config`](config::Config) by merging configurations as needed,
/// and finally collects results and displays them.
fn main() -> anyhow::Result<()> {
    if env::var("RUST_LOG").is_err() {
        Builder::new().filter_level(LevelFilter::Off).init();
    } else {
        env_logger::init();
    }
    debug!("initialized logger");

    let cli = Cli::parse();
    let mut config = if cli.ignore_config_file {
        Config::try_config_default()?
    } else {
        Config::try_config()?
    };
    debug!("loaded initial config");

    if let Some(found_display_mode_raw) = &cli.display_mode {
        config.display_mode = *found_display_mode_raw;
    }
    if let Some(found_color_mode) = &cli.color_mode {
        config.color_mode = *found_color_mode;
    }
    if let Some(found_path) = &cli.path {
        config.path = env::current_dir()?.join(found_path).canonicalize()?;
    }
    debug!("finalized config options");

    if cli.dry_run {
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

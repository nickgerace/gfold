use std::path::PathBuf;

use clap::Parser;
use clap_verbosity_flag::{InfoLevel, Verbosity};
use config::{ColorMode, DisplayMode};

const HELP: &str = "\
More information: https://github.com/nickgerace/gfold

Description: this application helps you keep track of multiple Git repositories via CLI. By default, it displays relevant information for all repos in the current working directory.

Config file usage: while CLI options are prioritized, default options will fallback to the config file if it exists. Here are the config file lookup locations:

    $XDG_CONFIG_HOME/gfold.toml
    $XDG_CONFIG_HOME/gfold/config.toml
    $HOME/.config/gfold.toml (or {{FOLDERID_Profile}}\\.config\\gfold.toml on Windows)";

#[derive(Debug, Parser)]
#[command(version, about = HELP, long_about = None)]
pub struct Cli {
    /// Specify path(s) to target directories (defaults to current working directory)
    pub paths: Option<Vec<PathBuf>>,
    /// Configure the color settings
    #[arg(short, long)]
    pub color_mode: Option<ColorMode>,
    /// Configure how collected information is displayed
    #[arg(short, long)]
    pub display_mode: Option<DisplayMode>,
    /// Display finalized config options and exit (merged options from an optional config file and command line arguments)
    #[arg(long)]
    pub dry_run: bool,
    /// Ignore config file settings
    #[arg(short, long)]
    pub ignore_config_file: bool,
    #[command(flatten)]
    pub verbose: Verbosity<InfoLevel>,
}

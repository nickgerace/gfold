//! This module contains the CLI entrypoint, CLI options and config generation based on the user's
//! settings and environment.

use crate::config::{ColorMode, Config, DisplayMode};
use crate::error::Error;
use anyhow::Result;
use crate::run;
use argh::FromArgs;
use std::env;

mod logging;

#[derive(FromArgs)]
#[argh(description = "More information: https://github.com/nickgerace/gfold

This application helps you keep track of multiple Git repositories via CLI.
By default, it displays relevant information for all repos in the current
working directory.

While CLI options are prioritized, default options will fallback to the config
file if it exists. Here is the config file lookup locations for some common
operating systems:

  macOS/Linux       $HOME/.config/gfold.toml
  Windows           {{FOLDERID_Profile}}\\.config\\gfold.toml")]
struct Args {
    #[argh(
        positional,
        description = "specify path to target directory (defaults to current working directory)"
    )]
    path: Option<String>,

    #[argh(
        option,
        short = 'c',
        description = "specify color mode [options: \"always\", \"compatibility\", \"off\"]"
    )]
    color_mode: Option<String>,
    #[argh(
        switch,
        description = "enable debug logging (sets \"RUST_LOG\" to \"debug\")"
    )]
    debug: bool,
    #[argh(
        option,
        short = 'd',
        description = "specify display format [options: \"standard\"/\"default\", \"json\", \"classic\"]"
    )]
    display_mode: Option<String>,
    #[argh(
        switch,
        description = "display config options chosen, including those from the config file if they exist"
    )]
    dry_run: bool,
    #[argh(switch, short = 'i', description = "ignore config file settings")]
    ignore_config_file: bool,
    #[argh(
        switch,
        short = 'V',
        description = "display version information (tip: set display mode (-d/--display-mode) to \"json\" to display version information as valid JSON"
    )]
    version: bool,
}

/// Parse CLI arguments, initialize the logger, merge configurations as needed, and call
/// [`run::run()`] with the resulting [`Config`].
pub fn parse_and_run() -> Result<()> {
    // First and foremost, get logging up and running. We want logs as quickly as possible for
    // debugging by setting "RUST_LOG".
    let args: Args = argh::from_env();
    logging::init(args.debug);

    let mut config = match args.ignore_config_file {
        true => Config::new()?,
        false => Config::try_config()?,
    };

    if let Some(found_display_mode) = &args.display_mode {
        config.display_mode = match found_display_mode.to_lowercase().as_str() {
            "classic" => DisplayMode::Classic,
            "json" => DisplayMode::Json,
            "standard" | "default" => DisplayMode::Standard,
            _ => return Err(Error::InvalidDisplayMode(found_display_mode.to_string())),
        }
    }

    // If the version flag is enabled, we need to use display mode to determine its output shape
    // and then return once version information is displayed.
    if args.version {
        match &config.display_mode {
            DisplayMode::Json => println!("{}", serde_json::to_string(env!("CARGO_PKG_VERSION"))?),
            _ => println!("gfold {}", env!("CARGO_PKG_VERSION")),
        }
        return Ok(());
    }

    if let Some(found_color_mode) = &args.color_mode {
        config.color_mode = match found_color_mode.to_lowercase().as_str() {
            "always" => ColorMode::Always,
            "compatibility" => ColorMode::Compatibility,
            "never" => ColorMode::Never,
            _ => return Err(Error::InvalidColorMode(found_color_mode.to_string())),
        }
    }

    if let Some(found_path) = &args.path {
        config.path = env::current_dir()?.join(found_path).canonicalize()?;
    }

    match &args.dry_run {
        true => config.print(),
        false => run::run(&config),
    }
}

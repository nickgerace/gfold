use crate::config::{Config, DisplayMode};
use crate::{logging, run};
use anyhow::Result;
use argh::FromArgs;
use std::env;

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
        switch,
        short = 'c',
        description = "display results with classic formatting"
    )]
    classic: bool,
    #[argh(
        switch,
        description = "enable debug logging (sets \"RUST_LOG\" to \"debug\")"
    )]
    debug: bool,
    #[argh(switch, short = 'i', description = "ignore config file settings")]
    ignore_config_file: bool,
    #[argh(
        switch,
        description = "display config options chosen, including those from the config file if they exist"
    )]
    print: bool,
    #[argh(switch, short = 'V', description = "display version information")]
    version: bool,
}

pub fn parse() -> Result<()> {
    // First and foremost, get logging up and running. We want logs as quickly as possible for
    // debugging by setting "RUST_LOG".
    let args: Args = argh::from_env();
    logging::init(args.debug);

    match args.version {
        true => println!("gfold {}", env!("CARGO_PKG_VERSION")),
        false => merge_config_and_run(&args)?,
    }
    Ok(())
}

fn merge_config_and_run(args: &Args) -> Result<()> {
    let mut config = match args.ignore_config_file {
        true => Config::new()?,
        false => Config::try_config()?,
    };

    if let Some(s) = &args.path {
        config.path = env::current_dir()?.join(s).canonicalize()?;
    }
    if args.classic {
        config.display_mode = DisplayMode::Classic;
    }

    match args.print {
        true => config.print(),
        false => run::run(&config),
    }
}

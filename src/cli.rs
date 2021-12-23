use crate::config::{Config, DisplayMode};
use crate::{logging, run};
use anyhow::Result;
use argh::FromArgs;
use std::env;

#[derive(FromArgs)]
#[argh(description = "https://github.com/nickgerace/gfold

This application helps you keep track of multiple Git repositories via CLI.
By default, it displays relevant information for all repos in the current
working directory.

While CLI options are prioritized, default options will fallback to the config
file if it exists. Here is the config file lookup locations for some common
operating systems:

  macOS/linux       $HOME/.config/gfold/gfold.json
  windows           {{FOLDERID_Profile}}\\.config\\gfold\\gfold.json")]
struct Args {
    #[argh(
        positional,
        description = "path to target directory (defaults to current working directory)"
    )]
    path: Option<String>,
    #[argh(
        switch,
        description = "enable debug logging (sets \"RUST_LOG\" to \"debug\")"
    )]
    debug: bool,
    #[argh(
        switch,
        description = "(TODO) display results with the new output mode"
    )]
    new: bool,
    #[argh(switch, description = "display merged config options")]
    print: bool,
    #[argh(switch, short = 'V', description = "display version information")]
    version: bool,
}

pub fn parse() -> Result<()> {
    // First and foremost, get logging up and running.
    let args: Args = argh::from_env();
    logging::init(args.debug);
    match args.version {
        true => {
            println!("gfold {}", env!("CARGO_PKG_VERSION"));
            Ok(())
        }
        false => merge_config_and_run(&args),
    }
}

fn merge_config_and_run(args: &Args) -> Result<()> {
    let mut config = Config::try_config()?;

    if let Some(s) = &args.path {
        config.default_path = Some(env::current_dir()?.join(s).canonicalize()?);
    }
    if args.new {
        config.display_mode = Some(DisplayMode::Modern);
    }

    // Set remaining "None" options to their defaults, if needed.
    config.set_defaults_if_empty()?;

    match args.print {
        true => config.print(),
        false => run::run(&config),
    }
}

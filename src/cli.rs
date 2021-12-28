use crate::config::{Config, DisplayMode};
use crate::{logging, run};
use anyhow::Result;
use argh::FromArgs;
use std::env;
use std::path::PathBuf;

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
        description = "specify path to target directory (defaults to current working directory)"
    )]
    path: Option<String>,

    #[argh(switch, short = 'i', description = "ignore config file settings")]
    ignore_config_file: bool,

    #[argh(switch, description = "display results with classic formatting")]
    classic: bool,
    #[argh(
        switch,
        description = "enable debug logging (sets \"RUST_LOG\" to \"debug\")"
    )]
    debug: bool,
    #[argh(
        option,
        description = "specify path to git binary rather than using git in PATH"
    )]
    git_path: Option<PathBuf>,
    #[argh(switch, description = "display merged config options")]
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
        true => {
            println!("gfold {}", env!("CARGO_PKG_VERSION"));
            Ok(())
        }
        false => merge_config_and_run(&args),
    }
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
    if let Some(s) = &args.git_path {
        config.git_path = Some(s.clone());
    }

    match args.print {
        true => config.print(),
        false => run::run(&config),
    }
}

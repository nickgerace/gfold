use crate::config::{Config, Mode};
use crate::run;
use anyhow::Result;
use clap::Parser;
use std::env;

#[derive(Parser)]
#[clap(
    name = "gfold",
    version = env!("CARGO_PKG_VERSION"),
    about = "https://github.com/nickgerace/gfold

This application helps you keep track of multiple Git repositories via CLI.
By default, it displays relevant information for all repos in the current
working directory.",
)]
struct Opt {
    #[clap(long, short, help = "(TODO) Display results with the new output mode")]
    new: bool,
    #[clap(help = "Path to target directory (defaults to current working directory")]
    path: Option<String>,
    #[clap(long, short, help = "Print config options and exit")]
    print: bool,
}

pub fn parse() -> Result<()> {
    let mut config = Config::try_config()?;

    let opt = Opt::parse();
    if let Some(s) = opt.path {
        config.default_path = Some(env::current_dir()?.join(s).canonicalize()?);
    }
    if opt.new {
        config.mode = Some(Mode::Modern);
    }

    // Set remaining "None" options to their defaults, if needed.
    config.set_defaults_if_empty()?;

    match opt.print {
        true => config.print(),
        false => run::run(&config),
    }
}

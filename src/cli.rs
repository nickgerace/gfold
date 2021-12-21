use crate::config::{Config, Mode};
use crate::display;
use crate::report::Reports;
use crate::target_gen::Targets;
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
}

pub fn parse() -> Result<()> {
    let mut config = Config::try_config()?;

    let opt = Opt::parse();
    if let Some(s) = opt.path {
        config.path = env::current_dir()?.join(s).canonicalize()?;
    }
    if opt.new {
        config.mode = Mode::Modern;
    }

    run(&config)
}

fn run(config: &Config) -> Result<()> {
    let reports = Reports::new(Targets::new(&config.path)?)?;
    match config.mode {
        Mode::Modern => display::modern(&reports)?,
        Mode::Classic => display::classic(&reports)?,
    }
    Ok(())
}

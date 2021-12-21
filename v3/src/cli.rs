use crate::config::Config;
use crate::display;
use crate::report::Reports;
use crate::target_gen::Targets;
use anyhow::Result;
use std::env;
use crate::config::Mode;
use clap::Parser;

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
    #[clap(
        long,
        short,
        help = "Display results with the new output mode"
    )]
    new: bool,
    #[clap(help = "Foo")]
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
        Mode::Classic =>  display::classic(&reports)?,
    }
    Ok(())
}

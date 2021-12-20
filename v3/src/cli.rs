use crate::config::Config;
use crate::display;
use crate::report::Reports;
use crate::target_gen::Targets;
use anyhow::Result;
use std::env;
use crate::config::Mode;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const HELP: &str = "
https://github.com/nickgerace/gfold

This application helps you keep track of multiple Git repositories via CLI.
By default, it displays relevant information for all repos in the current
working directory.

USAGE:
    gfold [FLAGS] [path]

FLAGS:
    -h, --help                     Prints help information
    -V, --version                  Prints version information

ARGS:
    <path>    Target a different directory";

pub fn parse() -> Result<()> {
    let mut config = Config::try_config()?;
    match env::args().nth(1).as_deref() {
        Some(s) if s == "-h" || s == "--help" => {
            println!("gfold {}{}", VERSION, HELP);
            return Ok(());
        }
        Some(s) if s == "-V" || s == "--version" => {
            println!("gfold {}", VERSION);
            return Ok(());
        }
        Some(s) if s == "--new" => {
            config.mode = Mode::Modern;

        }
        Some(s) => {
            config.path = env::current_dir()?.join(s).canonicalize()?;
        }
        None => {}
    }
    run(&config)
}

fn run(config: &Config) -> Result<()> {
    let reports = Reports::new(Targets::new(&config.path)?)?;
    match config.mode {
        Mode::Modern => display::standard(&reports)?,
        Mode::Classic =>  display::classic(&reports)?,
    }
    Ok(())
}

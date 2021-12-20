use crate::config::Config;
use crate::display;
use crate::report::Reports;
use crate::target_gen::Targets;
use anyhow::Result;
use std::env;

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
    match env::args().nth(1).as_deref() {
        Some(s) if s == "-h" || s == "--help" => {
            println!("gfold {}{}", VERSION, HELP);
            Ok(())
        }
        Some(s) if s == "-V" || s == "--version" => {
            println!("gfold {}", VERSION);
            Ok(())
        }
        Some(s) => run(Some(s)),
        None => run(None),
    }
}

fn run(path: Option<&str>) -> Result<()> {
    let mut config = Config::try_config()?;
    if let Some(s) = path {
        config.path = env::current_dir()?.join(s).canonicalize()?;
    }

    let reports = Reports::new(Targets::new(config.path)?)?;
    display::classic(&reports)?;
    Ok(())
}

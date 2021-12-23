use crate::config::{Config, Mode};
use crate::display;
use crate::error::Error;
use crate::report::Reports;
use crate::target_gen::Targets;
use anyhow::Result;

pub fn run(config: &Config) -> Result<()> {
    let reports = Reports::new(Targets::new(match &config.default_path {
        Some(s) => s,
        None => return Err(Error::EmptyConfigOption(config.to_owned()).into()),
    })?)?;
    match config.mode {
        Some(Mode::Modern) => display::modern(&reports),
        Some(Mode::Classic) => display::classic(&reports),
        None => Err(Error::EmptyConfigOption(config.to_owned()).into()),
    }
}

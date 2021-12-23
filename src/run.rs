use crate::config::{Config, DisplayMode};
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
    match config.display_mode {
        Some(DisplayMode::Modern) => display::modern(&reports),
        Some(DisplayMode::Classic) => display::classic(&reports),
        None => Err(Error::EmptyConfigOption(config.to_owned()).into()),
    }
}

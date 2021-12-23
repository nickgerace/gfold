use crate::config::{Config, DisplayMode};
use crate::display;
use crate::error::Error;
use crate::report::Reports;
use crate::target_gen::Targets;
use anyhow::Result;

pub fn run(config: &Config) -> Result<()> {
    let display_mode = config.determine_display_mode()?;

    let reports = Reports::new(
        Targets::new(match &config.default_path {
            Some(s) => s,
            None => return Err(Error::EmptyConfigOption(config.to_owned()).into()),
        })?,
        &display_mode,
    )?;

    match display_mode {
        DisplayMode::Standard => display::standard(&reports),
        DisplayMode::Classic => display::classic(&reports),
    }
}

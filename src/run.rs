use crate::config::{Config, DisplayMode};
use crate::display;

use crate::report::Reports;
use crate::target_gen::Targets;
use anyhow::Result;

pub fn run(config: &Config) -> Result<()> {
    let reports = Reports::new(Targets::new(&config.path)?, &config.display_mode)?;

    match config.display_mode {
        DisplayMode::Standard => display::standard(&reports),
        DisplayMode::Classic => display::classic(&reports),
    }
}

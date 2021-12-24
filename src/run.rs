use crate::config::{Config, DisplayMode};
use crate::display;

use crate::report::Reports;
use crate::target_gen::Targets;
use anyhow::Result;

// This function is the primary entrypoint for the crate. It takes a given config and performs
// the end-to-end workflow using it. At this point, all CLI and config file options should be
// set, merged, ignored, etc.
pub fn run(config: &Config) -> Result<()> {
    let reports = Reports::new(Targets::new(&config.path)?, &config.display_mode)?;

    match config.display_mode {
        DisplayMode::Standard => display::standard(&reports),
        DisplayMode::Classic => display::classic(&reports),
    }
}

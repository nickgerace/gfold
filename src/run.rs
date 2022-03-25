//! This module contains the execution logic for generating reports and displaying them to `stdout`.

use crate::config::Config;
use crate::result::Result;
use crate::{display, report};

/// This function is the primary entrypoint for the crate. It takes a given config and performs
/// the end-to-end workflow using it. At this point, all CLI and config file options should be
/// set, merged, ignored, etc.
pub fn run(config: &Config) -> Result<()> {
    let reports = report::generate_reports(&config.path, &config.display_mode)?;
    display::display(&config.display_mode, &reports, &config.color_mode)
}

//! This module contains the execution logic for generating reports and displaying them to `stdout`.

use crate::collector::RepositoryCollector;
use crate::config::Config;
use crate::display::DisplayHarness;

/// This struct provides the primary entrypoint for this crate. It takes a given config and performs
/// the end-to-end workflow using it. At this point, all CLI and config file options should be
/// set, merged, ignored, etc.
pub struct RunHarness<'config> {
    config: &'config Config,
}

impl<'config> RunHarness<'config> {
    pub fn new(config: &'config Config) -> Self {
        Self { config }
    }

    pub fn run(&self) -> anyhow::Result<()> {
        let repository_collection =
            RepositoryCollector::run(&self.config.path, self.config.display_mode)?;
        let display_harness = DisplayHarness::new(self.config.display_mode, self.config.color_mode);
        display_harness.run(&repository_collection)
    }
}

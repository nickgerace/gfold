//! This module contains the logger initialization logic.

use env_logger::Builder;
use log::LevelFilter;
use std::env;

/// Initialize the logger based on the debug flag and `RUST_LOG` environment variable. The flag
/// takes precedence over the environment variable.
pub fn init(debug: bool) {
    match debug {
        true => Builder::new().filter_level(LevelFilter::Debug).init(),
        false => match env::var("RUST_LOG").is_err() {
            true => Builder::new().filter_level(LevelFilter::Off).init(),
            false => env_logger::init(),
        },
    }
}

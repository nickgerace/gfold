use env_logger::Builder;
use log::LevelFilter;
use std::env;

pub fn init(debug: bool) {
    match debug {
        true => Builder::new().filter_level(LevelFilter::Debug).init(),
        false => match env::var("RUST_LOG").is_err() {
            true => Builder::new().filter_level(LevelFilter::Off).init(),
            false => env_logger::init(),
        },
    }
}

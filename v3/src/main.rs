use anyhow::Result;

mod cli;
mod color;
mod config;
mod display;
mod error;
mod logging;
mod report;
mod status;
mod target_gen;

fn main() -> Result<()> {
    logging::init();
    cli::parse()
}

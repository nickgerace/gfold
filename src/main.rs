use anyhow::Result;

mod cli;
mod color;
mod config;
mod consts;
mod display;
mod error;
mod logging;
mod report;
mod run;
mod status;
mod target_gen;

fn main() -> Result<()> {
    cli::parse()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    use crate::error::Error;
    use std::env;

    #[test]
    fn current_directory() {
        let config = Config::new().unwrap();

        assert!(run::run(&config).is_ok());
    }

    #[test]
    fn parent_directory() {
        let mut config = Config::new().unwrap();

        let mut parent = env::current_dir().expect("failed to get current working directory");
        parent.pop();
        config.path = parent;

        assert!(run::run(&config).is_ok());
    }

    #[test]
    fn home_directory() {
        let mut config = Config::new().unwrap();

        config.path = dirs::home_dir().ok_or(Error::HomeDirNotFound).unwrap();

        assert!(run::run(&config).is_ok());
    }
}

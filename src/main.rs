use anyhow::Result;

mod cli;
mod color;
mod config;
mod display;
mod error;
mod logging;
mod report;
mod run;
mod status;

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
        let config = Config::new().expect("could not create new Config");
        assert!(run::run(&config).is_ok());
    }

    #[test]
    fn parent_directory() {
        let mut config = Config::new().expect("could not create new Config");
        let mut parent = env::current_dir().expect("failed to get current working directory");
        parent.pop();
        config.path = parent;
        assert!(run::run(&config).is_ok());
    }

    #[test]
    fn home_directory() {
        let mut config = Config::new().expect("could not create new Config");
        config.path = dirs::home_dir()
            .ok_or(Error::HomeDirNotFound)
            .expect("could not find home directory");
        assert!(run::run(&config).is_ok());
    }
}

use crate::error::Error;
use anyhow::Result;
use log::warn;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Config {
    pub default_path: Option<PathBuf>,
    pub mode: Option<Mode>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Mode {
    Classic,
    Modern,
}

impl Config {
    pub fn try_config() -> Result<Config> {
        let home = dirs::home_dir().ok_or(Error::HomeDirNotFound)?;
        match File::open(home.join(".config").join("gfold").join("gfold.json")) {
            Ok(o) => {
                let reader = BufReader::new(o);
                let config: Config = serde_json::from_reader(reader)?;
                Ok(config)
            }
            Err(e) => {
                warn!("{}", e);
                Ok(Config {
                    default_path: None,
                    mode: None,
                })
            }
        }
    }

    pub fn set_defaults_if_empty(&mut self) -> Result<()> {
        if self.default_path.is_none() {
            self.default_path = Some(env::current_dir()?.canonicalize()?);
        }
        if self.mode.is_none() {
            self.mode = Some(Mode::Classic)
        }
        Ok(())
    }

    pub fn print(self) -> Result<()> {
        println!("{}", serde_json::to_string_pretty(&self)?);
        Ok(())
    }
}

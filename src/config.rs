use crate::error::Error;
use anyhow::Result;
use log::warn;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub path: PathBuf,
    pub mode: Mode,
}

#[derive(Serialize, Deserialize)]
pub enum Mode {
    Classic,
    Modern,
}

impl Config {
    pub fn try_config() -> Result<Config> {
        let home = dirs::home_dir().ok_or(Error::HomeDirNotFound)?;
        let filepath = home.join(".config").join("gfold").join("gfold.json");
        match File::open(filepath) {
            Ok(o) => {
                let reader = BufReader::new(o);
                let config: Config = serde_json::from_reader(reader)?;
                Ok(config)
            }
            Err(e) => {
                warn!("{}", e);
                Ok(Config {
                    path: env::current_dir()?.canonicalize()?,
                    mode: Mode::Classic,
                })
            }
        }
    }
}

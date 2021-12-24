use crate::error::Error;
use anyhow::Result;
use log::warn;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

#[derive(Deserialize, Default)]
pub struct EntryConfig {
    pub path: Option<PathBuf>,
    pub display_mode: Option<DisplayMode>,
}

#[derive(Serialize)]
pub struct Config {
    pub path: PathBuf,
    pub display_mode: DisplayMode,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum DisplayMode {
    Standard,
    Classic,
}

impl Config {
    pub fn try_config() -> Result<Config> {
        let home = dirs::home_dir().ok_or(Error::HomeDirNotFound)?;
        let entry_config = match File::open(home.join(".config").join("gfold").join("gfold.json")) {
            Ok(o) => {
                let reader = BufReader::new(o);
                serde_json::from_reader(reader)?
            }
            Err(e) => {
                warn!("{}", e);
                EntryConfig::default()
            }
        };
        entry_config_to_config(&entry_config)
    }

    pub fn print(self) -> Result<()> {
        println!("{}", serde_json::to_string_pretty(&self)?);
        Ok(())
    }
}

pub fn entry_config_to_config(entry_config: &EntryConfig) -> Result<Config> {
    Ok(Config {
        path: match &entry_config.path {
            Some(s) => s.clone(),
            None => env::current_dir()?.canonicalize()?,
        },
        display_mode: match &entry_config.display_mode {
            Some(s) => s.clone(),
            None => DisplayMode::Standard,
        },
    })
}

use crate::error::Error;
use anyhow::{Context, Result};
use log::warn;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

// "Config" is the actual type consumed through the codebase. It is boostrapped via its public
// methods and uses "EntryConfig", a private struct, under the hood in order to deserialize empty,
// non-existent, partial, and complete config files.
#[derive(Serialize)]
pub struct Config {
    pub path: PathBuf,
    pub display_mode: DisplayMode,
}

// "EntryConfig" is a reflection of "Config" with its fields wrapped as "Option" types. This is to
// ensure that we can deserialize from partial config file contents and populate empty fields with
// defaults. Moreover, enumerations cannot set defaults values currently, so we need to set
// desired defaults for the user. In this case, the public methods for "Config" use "EntryConfig"
// privately.
#[derive(Deserialize, Default)]
struct EntryConfig {
    pub path: Option<PathBuf>,
    pub display_mode: Option<DisplayMode>,
}

// "DisplayMode" dictates which way the results gathered should be displayed to the user via
// STDOUT. Setting this enumeration is _mostly_ cosmetic, but it is possible that collected data
// may differ in order to reduce compute load. For example: if one display mode dislays more
// information than another display mode, more subcommands and functions might get executed.
// Conversely, if another display mode requires less information to be displayed, then some
// commands and functions migth get skipped.
//
// TLDR: while this setting is primarily for cosmetics, it may also affect runtime performance
// based on what needs to be displayed.
#[derive(Serialize, Deserialize, Clone)]
pub enum DisplayMode {
    Standard,
    Classic,
}

impl Config {
    // This method tries to deserialize the config file (empty, non-existent, partial or complete)
    // and uses "EntryConfig" as an intermediary struct. This is the primary method used when
    // creating a config.
    //
    // Within this method, we check if the config file is empty before deserializing it. Users
    // should be able to proceed with empty config files. If empty, then we fall back to the
    // "EntryConfig" default before conversion.
    pub fn try_config() -> Result<Self> {
        let home = dirs::home_dir().ok_or(Error::HomeDirNotFound)?;
        let entry_config = match File::open(home.join(".config").join("gfold").join("gfold.json")) {
            Ok(o) => match o.metadata()?.len() {
                len if len > 0 => {
                    let reader = BufReader::new(o);
                    serde_json::from_reader(reader)
                        .context("config file's contents are likely invalid JSON")?
                }
                _ => EntryConfig::default(),
            },
            Err(e) => {
                warn!("{}", e);
                EntryConfig::default()
            }
        };
        Self::from_entry_config(&entry_config)
    }

    // This method does not look for the config file and uses "EntryConfig"'s defaults instead.
    // It is best for testing use and when the user wishes to skip config file lookup.
    pub fn new() -> Result<Self> {
        Self::from_entry_config(&EntryConfig::default())
    }

    // This method prints the full config (merged with config file, as needed) as valid JSON.
    pub fn print(self) -> Result<()> {
        println!("{}", serde_json::to_string_pretty(&self)?);
        Ok(())
    }

    fn from_entry_config(entry_config: &EntryConfig) -> Result<Self> {
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
}

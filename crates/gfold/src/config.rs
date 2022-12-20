//! This module contains the config specification and functionality for creating a config.

use crate::error::Error;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::{env, fs, io};

/// This struct is the actual config type consumed through the codebase. It is boostrapped via its
/// public methods and uses [`EntryConfig`], a private struct, under the hood in order to
/// deserialize empty, non-existent, partial, and complete config files.
#[derive(Serialize)]
pub struct Config {
    /// The path that `gfold` will begin traversal in and collect results from.
    pub path: PathBuf,
    /// The display format for results printed to `stdout`.
    pub display_mode: DisplayMode,
    /// The color mode for results printed to `stdout`.
    pub color_mode: ColorMode,
}

/// This struct is a reflection of [`Config`] with its fields wrapped with [`Option`], which
/// ensures that we can deserialize from partial config file contents and populate empty fields
/// with defaults. Moreover, enum fields cannot set defaults values currently, so we need to
/// manually set defaults for the user. For those reasons, the public methods for [`Config`] use
/// this struct privately.
#[derive(Deserialize, Default)]
struct EntryConfig {
    /// Reflection of the `path` field on [`Config`].
    pub path: Option<PathBuf>,
    /// Reflection of the `display_mode` field on [`Config`].
    pub display_mode: Option<DisplayMode>,
    /// Reflection of the `color_mode` field on [`Config`].
    pub color_mode: Option<ColorMode>,
}

/// Dictates how the results gathered should be displayed to the user via `stdout`. Setting this
/// enum is _mostly_ cosmetic, but it is possible that collected data may differ in order to
/// reduce compute load. For example: if one display mode dislays more information than another
/// display mode, more data may need to be collected. Conversely, if another display mode requires
/// less information to be displayed, then some commands and functions migth get skipped.
/// In summary, while this setting is primarily for cosmetics, it may also affect runtime
/// performance based on what needs to be displayed.
#[derive(Serialize, Deserialize, Clone)]
pub enum DisplayMode {
    /// Informs the caller to display results in the standard (default) format.
    Standard,
    /// Informs the caller to display results in the classic format.
    Classic,
    /// Informs the caller to display results in JSON format.
    Json,
}

/// Set the color mode of results printed to `stdout`.
#[derive(Serialize, Deserialize, Clone)]
pub enum ColorMode {
    /// Attempt to display colors as intended (default behavior).
    Always,
    /// Display colors using widely-compatible methods at the potential expense of colors being
    /// displayed as intended.
    Compatibility,
    /// Never display colors.
    Never,
}

impl Config {
    /// This method tries to deserialize the config file (empty, non-existent, partial or complete)
    /// and uses [`EntryConfig`] as an intermediary struct. This is the primary method used when
    /// creating a config.
    pub fn try_config() -> anyhow::Result<Self> {
        // Within this method, we check if the config file is empty before deserializing it. Users
        // should be able to proceed with empty config files. If empty or not found, then we fall
        // back to the "EntryConfig" default before conversion.
        let home = dirs::home_dir().ok_or(Error::HomeDirNotFound)?;
        let path = home.join(".config").join("gfold.toml");
        let entry_config = match fs::read_to_string(path) {
            Ok(contents) => match contents.is_empty() {
                true => EntryConfig::default(),
                false => toml::from_str(&contents)?,
            },
            Err(e) => match e.kind() {
                io::ErrorKind::NotFound => EntryConfig::default(),
                _ => return Err(e.into()),
            },
        };
        Ok(Self::from_entry_config(&entry_config)?)
    }

    /// This method does not look for the config file and uses [`EntryConfig`]'s defaults instead.
    /// Use this method when the user wishes to skip config file lookup.
    pub fn try_config_default() -> io::Result<Self> {
        Self::from_entry_config(&EntryConfig::default())
    }

    /// This method prints the full config (merged with config file, as needed) as valid, pretty TOML.
    pub fn print(self) -> Result<(), toml::ser::Error> {
        print!("{}", toml::to_string_pretty(&self)?);
        Ok(())
    }

    fn from_entry_config(entry_config: &EntryConfig) -> io::Result<Self> {
        Ok(Config {
            path: match &entry_config.path {
                Some(s) => s.clone(),
                None => env::current_dir()?.canonicalize()?,
            },
            display_mode: match &entry_config.display_mode {
                Some(s) => s.clone(),
                None => DisplayMode::Standard,
            },
            color_mode: match &entry_config.color_mode {
                Some(s) => s.clone(),
                None => ColorMode::Always,
            },
        })
    }
}

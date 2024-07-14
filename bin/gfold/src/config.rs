//! This module contains the config specification and functionality for creating a config.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::{env, fs, io};
use thiserror::Error;

#[remain::sorted]
#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("could not find home directory")]
    HomeDirNotFound,
}

/// This struct is the actual config type consumed through the codebase. It is boostrapped via its
/// public methods and uses [`EntryConfig`], a private struct, under the hood in order to
/// deserialize empty, non-existent, partial, and complete config files.
#[derive(Deserialize, Serialize)]
pub struct Config {
    /// The path that `gfold` will begin traversal in and collect results from.
    pub path: PathBuf,
    /// The color mode for results printed to `stdout`.
    pub color_mode: ColorMode,
    /// Toggles parallel collection optimizations (default is "true").
    pub parallel: bool,
    pub json: JsonOptions,
    pub group_by_parent_directory: bool,
    pub sort_status: bool,
    pub alphabetical: bool,
    pub include_email: bool,
    pub include_submodules: bool,
}

impl Config {
    /// This method tries to deserialize the config file (empty, non-existent, partial or complete)
    /// and uses [`EntryConfig`] as an intermediary struct. This is the primary method used when
    /// creating a config.
    pub fn try_config() -> anyhow::Result<Self> {
        // Within this method, we check if the config file is empty before deserializing it. Users
        // should be able to proceed with empty config files. If empty or not found, then we fall
        // back to the "EntryConfig" default before conversion.
        let home = dirs::home_dir().ok_or(ConfigError::HomeDirNotFound)?;
        let path = home.join(".config").join("gfold").join("config.toml");
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
    pub fn print(self) -> std::result::Result<(), toml::ser::Error> {
        print!("{}", toml::to_string_pretty(&self)?);
        Ok(())
    }

    fn from_entry_config(entry_config: &EntryConfig) -> io::Result<Self> {
        Ok(Config {
            path: match &entry_config.path {
                Some(path) => path.clone(),
                None => env::current_dir()?.canonicalize()?,
            },
            color_mode: match &entry_config.color_mode {
                Some(color_mode) => *color_mode,
                None => ColorMode::Always,
            },
            parallel: match &entry_config.parallel {
                Some(parallel) => *parallel,
                None => true,
            },
            json: match &entry_config.json {
                Some(json) => *json,
                None => JsonOptions::False,
            },
            group_by_parent_directory: match &entry_config.group_by_parent_directory {
                Some(group_by_parent_directory) => *group_by_parent_directory,
                None => true,
            },
            sort_status: match &entry_config.sort_status {
                Some(sort_status) => *sort_status,
                None => true,
            },
            alphabetical: match &entry_config.alphabetical {
                Some(alphabetical) => *alphabetical,
                None => true,
            },
            include_submodules: match &entry_config.include_submodules {
                Some(include_submodules) => *include_submodules,
                None => false,
            },
            include_email: match &entry_config.include_email {
                Some(include_email) => *include_email,
                None => true,
            },
        })
    }
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
    /// Reflection of the `color_mode` field on [`Config`].
    pub color_mode: Option<ColorMode>,
    /// Reflection of the `parallel` if on [`Config`].
    pub parallel: Option<bool>,
    pub json: Option<JsonOptions>,
    pub group_by_parent_directory: Option<bool>,
    pub sort_status: Option<bool>,
    pub alphabetical: Option<bool>,
    pub include_email: Option<bool>,
    pub include_submodules: Option<bool>,
}

/// Set the color mode of results printed to `stdout`.
#[remain::sorted]
#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum ColorMode {
    /// Attempt to display colors as intended (default behavior).
    Always,
    /// Display colors using widely-compatible methods at the potential expense of colors being
    /// displayed as intended.
    Compatibility,
    /// Never display colors.
    Never,
}

#[remain::sorted]
#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum JsonOptions {
    False,
    Pretty,
    Raw,
}

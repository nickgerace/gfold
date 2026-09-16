use std::env;
use std::ffi::OsString;
use std::fs;
use std::num::NonZeroUsize;
use std::path::PathBuf;

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Config {
    pub path: PathBuf,
    pub sequential: bool,
    pub parallel_collect_threads: Option<NonZeroUsize>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct FileConfig {
    pub path: Option<PathBuf>,
    pub parallel_collect_threads: Option<NonZeroUsize>,
}

impl Config {
    pub fn resolve(
        cli_path: Option<PathBuf>,
        sequential: bool,
        cli_parallel_collect_threads: Option<NonZeroUsize>,
        ignore_config_file: bool,
    ) -> Result<Self> {
        if sequential && cli_parallel_collect_threads.is_some() {
            bail!("--sequential cannot be used with --parallel-collect-threads");
        }

        let file_config = if ignore_config_file {
            None
        } else {
            FileConfig::load()?
        };

        Ok(Self::from_sources(
            cli_path,
            sequential,
            cli_parallel_collect_threads,
            file_config,
            default_path()?,
        ))
    }

    fn from_sources(
        cli_path: Option<PathBuf>,
        sequential: bool,
        cli_parallel_collect_threads: Option<NonZeroUsize>,
        file_config: Option<FileConfig>,
        default_path: PathBuf,
    ) -> Self {
        let file_path = file_config.as_ref().and_then(|config| config.path.clone());
        let file_parallel_collect_threads =
            file_config.and_then(|config| config.parallel_collect_threads);

        Self {
            path: cli_path.or(file_path).unwrap_or(default_path),
            sequential,
            parallel_collect_threads: cli_parallel_collect_threads
                .or(file_parallel_collect_threads),
        }
    }
}

impl FileConfig {
    pub fn load() -> Result<Option<Self>> {
        let Some(path) = config_path() else {
            return Ok(None);
        };

        if !path.exists() {
            return Ok(None);
        }

        let contents = fs::read_to_string(&path)
            .with_context(|| format!("failed to read config file {}", path.display()))?;
        let config = toml::from_str(&contents)
            .with_context(|| format!("failed to parse config file {}", path.display()))?;
        Ok(Some(config))
    }
}

pub fn config_path() -> Option<PathBuf> {
    config_path_from_env(env::var_os("XDG_CONFIG_HOME"), env::var_os("HOME"))
}

fn config_path_from_env(
    xdg_config_home: Option<OsString>,
    home: Option<OsString>,
) -> Option<PathBuf> {
    let config_home = xdg_config_home
        .filter(|value| !value.is_empty())
        .map(PathBuf::from)
        .or_else(|| home.map(|home| PathBuf::from(home).join(".config")))?;
    Some(config_home.join("jjfold").join("config.toml"))
}

fn default_path() -> Result<PathBuf> {
    let cwd = env::current_dir().context("failed to read current directory")?;
    Ok(cwd.parent().unwrap_or(&cwd).to_path_buf())
}

#[cfg(test)]
mod tests {
    use std::num::NonZeroUsize;
    use std::path::PathBuf;

    use super::{Config, FileConfig, config_path_from_env};

    #[test]
    fn cli_path_takes_precedence() {
        let config = Config::from_sources(
            Some(PathBuf::from("/cli")),
            false,
            None,
            Some(FileConfig {
                path: Some(PathBuf::from("/config")),
                parallel_collect_threads: None,
            }),
            PathBuf::from("/default"),
        );

        assert_eq!(config.path, PathBuf::from("/cli"));
    }

    #[test]
    fn config_path_takes_precedence_over_default() {
        let config = Config::from_sources(
            None,
            false,
            None,
            Some(FileConfig {
                path: Some(PathBuf::from("/config")),
                parallel_collect_threads: None,
            }),
            PathBuf::from("/default"),
        );

        assert_eq!(config.path, PathBuf::from("/config"));
    }

    #[test]
    fn default_path_is_used_without_cli_or_config() {
        let config = Config::from_sources(None, false, None, None, PathBuf::from("/default"));

        assert_eq!(config.path, PathBuf::from("/default"));
    }

    #[test]
    fn sequential_flag_is_preserved() {
        let config = Config::from_sources(None, true, None, None, PathBuf::from("/default"));

        assert!(config.sequential);
    }

    #[test]
    fn cli_parallel_collect_threads_takes_precedence() {
        let config = Config::from_sources(
            None,
            false,
            NonZeroUsize::new(8),
            Some(FileConfig {
                path: None,
                parallel_collect_threads: NonZeroUsize::new(2),
            }),
            PathBuf::from("/default"),
        );

        assert_eq!(
            config.parallel_collect_threads,
            Some(NonZeroUsize::new(8).unwrap())
        );
    }

    #[test]
    fn sequential_conflicts_with_cli_parallel_collect_threads() {
        let error = Config::resolve(None, true, NonZeroUsize::new(8), true).unwrap_err();

        assert!(
            error
                .to_string()
                .contains("--sequential cannot be used with --parallel-collect-threads")
        );
    }

    #[test]
    fn config_parallel_collect_threads_takes_precedence_over_default() {
        let config = Config::from_sources(
            None,
            false,
            None,
            Some(FileConfig {
                path: None,
                parallel_collect_threads: NonZeroUsize::new(2),
            }),
            PathBuf::from("/default"),
        );

        assert_eq!(
            config.parallel_collect_threads,
            Some(NonZeroUsize::new(2).unwrap())
        );
    }

    #[test]
    fn parallel_collect_threads_is_unset_without_cli_or_config() {
        let config = Config::from_sources(None, false, None, None, PathBuf::from("/default"));

        assert_eq!(config.parallel_collect_threads, None);
    }

    #[test]
    fn ignored_config_file_uses_default_values() -> anyhow::Result<()> {
        let config = Config::resolve(None, false, None, true)?;

        assert_eq!(config.parallel_collect_threads, None);
        Ok(())
    }

    #[test]
    fn config_path_uses_xdg_config_home() {
        let path = config_path_from_env(Some(PathBuf::from("/xdg").into_os_string()), None);

        assert_eq!(path, Some(PathBuf::from("/xdg/jjfold/config.toml")));
    }

    #[test]
    fn config_path_falls_back_to_home() {
        let path = config_path_from_env(None, Some(PathBuf::from("/home/user").into_os_string()));

        assert_eq!(
            path,
            Some(PathBuf::from("/home/user/.config/jjfold/config.toml"))
        );
    }

    #[test]
    fn config_serializes_to_toml() -> anyhow::Result<()> {
        let config = Config {
            path: PathBuf::from("/target"),
            sequential: true,
            parallel_collect_threads: NonZeroUsize::new(8),
        };

        let serialized = toml::to_string(&config)?;
        let deserialized = toml::from_str::<Config>(&serialized)?;

        assert_eq!(deserialized, config);
        Ok(())
    }
}

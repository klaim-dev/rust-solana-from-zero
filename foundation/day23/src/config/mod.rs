use crate::env::Env;
use std::{
    fmt,
    io::ErrorKind,
    path::{Path, PathBuf},
};

mod env;
mod file;

use env::{load_env, EnvConfig};
use file::{load_from_file, FileConfig};

#[cfg(test)]
use env::{env_usize, require_env};

#[derive(Debug, PartialEq)]
pub enum ConfigError {
    MissingRequired {
        key: &'static str,
    },
    EmptyEnv {
        key: &'static str,
    },
    InvalidEnv {
        key: &'static str,
        reason: &'static str,
    },
    Io {
        path: PathBuf,
        kind: ErrorKind,
    },
    BadToml {
        path: PathBuf,
    },
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::MissingRequired { key } => {
                write!(f, "missing required setting: {key}")
            }
            ConfigError::EmptyEnv { key } => write!(f, "empty env var: {key}"),
            ConfigError::InvalidEnv { key, reason } => {
                write!(f, "invalid env var {key}: {reason}")
            }
            ConfigError::Io { path, kind } => {
                write!(f, "io error for {}: {:?}", path.display(), kind)
            }
            ConfigError::BadToml { path } => write!(f, "bad toml: {}", path.display()),
        }
    }
}

impl std::error::Error for ConfigError {}

#[derive(Debug, PartialEq)]
pub enum AppEnv {
    Dev,
    Test,
    Prod,
}

impl AppEnv {
    fn parse(raw: &str, key: &'static str) -> Result<AppEnv, ConfigError> {
        let v = raw.trim();
        if v.is_empty() {
            return Err(ConfigError::EmptyEnv { key });
        }

        match v.to_lowercase().as_str() {
            "dev" => Ok(AppEnv::Dev),
            "test" => Ok(AppEnv::Test),
            "prod" => Ok(AppEnv::Prod),
            _ => Err(ConfigError::InvalidEnv {
                key,
                reason: "expected one of: dev, test, prod",
            }),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Config {
    app_env: AppEnv,
    data_file: PathBuf,
    page_size: usize,
}

impl Config {
    pub fn load<E: Env>(env: &E, path: &Path) -> Result<Self, ConfigError> {
        let file_cfg = match load_from_file(path) {
            Ok(cfg) => Some(cfg),
            Err(ConfigError::Io {
                kind: ErrorKind::NotFound,
                ..
            }) => None,
            Err(e) => return Err(e),
        };
        let env_cfg = load_env(env)?;

        merge(file_cfg, env_cfg)
    }

    pub fn data_file(&self) -> &Path {
        self.data_file.as_path()
    }

    pub fn page_size(&self) -> usize {
        self.page_size
    }

    pub fn app_env(&self) -> &AppEnv {
        &self.app_env
    }
}

pub(crate) fn merge(
    file_cfg: Option<FileConfig>,
    env_cfg: EnvConfig,
) -> Result<Config, ConfigError> {
    let file_cfg = file_cfg.unwrap_or_default();
    let app_env_raw = env_cfg.app_env.or(file_cfg.app_env);

    let app_env = match app_env_raw {
        Some(raw) => AppEnv::parse(&raw, "APP_ENV")?,
        None => AppEnv::Dev,
    };

    let page_size = env_cfg.page_size.or(file_cfg.page_size).unwrap_or(50);

    let data_file = env_cfg
        .data_file
        .or(file_cfg.data_file)
        .ok_or(ConfigError::MissingRequired { key: "DATA_FILE" })?;

    Ok(Config {
        app_env,
        data_file,
        page_size,
    })
}

#[cfg(test)]
mod tests;

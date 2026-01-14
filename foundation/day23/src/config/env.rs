use crate::env::Env;
use std::path::PathBuf;

use super::ConfigError;

pub(crate) struct EnvConfig {
    pub(crate) app_env: Option<String>,
    pub(crate) data_file: Option<PathBuf>,
    pub(crate) page_size: Option<usize>,
}

#[allow(dead_code)]
pub(crate) fn require_env<E: Env>(env: &E, key: &'static str) -> Result<String, ConfigError> {
    let raw = env
        .get(key)
        .ok_or_else(|| ConfigError::MissingRequired { key })?;

    if raw.is_empty() {
        return Err(ConfigError::EmptyEnv { key });
    }

    Ok(raw)
}

pub(crate) fn env_usize<E: Env>(
    env: &E,
    key: &'static str,
) -> Result<Option<usize>, ConfigError> {
    let raw = env.get(key);
    match raw {
        None => Ok(None),
        Some(value) => {
            if value.is_empty() {
                return Err(ConfigError::EmptyEnv { key });
            }
            let parsed = value
                .parse::<usize>()
                .map_err(|_| ConfigError::InvalidEnv {
                    key,
                    reason: "expected usize",
                })?;
            Ok(Some(parsed))
        }
    }
}

pub(crate) fn env_opt_string<E: Env>(
    env: &E,
    key: &'static str,
) -> Result<Option<String>, ConfigError> {
    match env.get(key) {
        None => Ok(None),
        Some(v) if v.is_empty() => Err(ConfigError::EmptyEnv { key }),
        Some(v) => Ok(Some(v)),
    }
}

pub(crate) fn env_opt_path<E: Env>(
    env: &E,
    key: &'static str,
) -> Result<Option<PathBuf>, ConfigError> {
    Ok(env_opt_string(env, key)?.map(PathBuf::from))
}

pub(crate) fn load_env<E: Env>(env: &E) -> Result<EnvConfig, ConfigError> {
    Ok(EnvConfig {
        app_env: env_opt_string(env, "APP_ENV")?,
        data_file: env_opt_path(env, "DATA_FILE")?,
        page_size: env_usize(env, "PAGE_SIZE")?,
    })
}

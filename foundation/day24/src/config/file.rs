use crate::config::error::ConfigError;
use crate::config::models::FileConfig;
use std::{path::Path, path::PathBuf};

#[derive(serde::Deserialize)]
struct RawFileConfig {
    pipeline_raw: Option<String>,
    data_file: Option<PathBuf>,
}

pub fn load_file(path: &Path) -> Result<FileConfig, ConfigError> {
    let text = std::fs::read_to_string(path).map_err(|e| ConfigError::Io {
        path: path.to_path_buf(),
        source: e,
    })?;

    let raw: RawFileConfig = toml::from_str(&text).map_err(|e| ConfigError::Toml {
        path: path.to_path_buf(),
        source: e,
    })?;

    Ok(FileConfig {
        pipeline_raw: raw.pipeline_raw,
        data_file: raw.data_file,
    })
}

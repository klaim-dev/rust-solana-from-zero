use std::{fs, path::Path, path::PathBuf};

use super::ConfigError;

#[derive(Debug, serde::Deserialize, PartialEq, Default)]
pub(crate) struct FileConfig {
    pub(crate) app_env: Option<String>,
    pub(crate) data_file: Option<PathBuf>,
    pub(crate) page_size: Option<usize>,
}

pub(crate) fn load_from_file(path: &Path) -> Result<FileConfig, ConfigError> {
    let text = fs::read_to_string(path).map_err(|e| ConfigError::Io {
        path: path.to_path_buf(),
        kind: e.kind(),
    })?;
    let cfg = toml::from_str::<FileConfig>(&text).map_err(|_| ConfigError::BadToml {
        path: path.to_path_buf(),
    })?;
    Ok(cfg)
}

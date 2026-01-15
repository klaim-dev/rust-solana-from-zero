use crate::config::{env::load_env, error::ConfigError, file::load_file};
use std::{path::Path, path::PathBuf};
pub struct Config {
    pub pipeline_raw: String,       // REQUIRED
    pub data_file: Option<PathBuf>, // optional
    pub strict: bool,               // optional
}
impl Config {
    pub fn load(path: &Path) -> Result<Self, ConfigError> {
        let env = load_env();
        let file = match load_file(path) {
            Ok(f) => Some(f),
            Err(ConfigError::Io { path: _, source })
                if source.kind() == std::io::ErrorKind::NotFound =>
            {
                None
            }
            Err(e) => return Err(e),
        };
        Self::merge(env, file)
    }
    pub fn merge(env: EnvConfig, file: Option<FileConfig>) -> Result<Self, ConfigError> {
        let file_pipline = file.as_ref().and_then(|f| f.pipeline_raw.clone());
        let file_data = file.as_ref().and_then(|f| f.data_file.clone());
        let pipeline_raw = match env.pipeline_raw.or_else(|| file_pipline.map(|f| f)) {
            None => return Err(ConfigError::MissingRequired { key: "PIPELINE" }),
            Some(s) if s.trim().is_empty() => {
                return Err(ConfigError::EmptyValue { key: "PIPELINE" });
            }
            Some(s) => s,
        };

        let data_file = env.data_file.or_else(|| file_data.map(|f| f));

        Ok(Config {
            pipeline_raw,
            data_file,
            strict: false,
        })
    }
}

pub struct EnvConfig {
    pub pipeline_raw: Option<String>,
    pub data_file: Option<PathBuf>,
}

pub struct FileConfig {
    pub pipeline_raw: Option<String>,
    pub data_file: Option<PathBuf>,
}

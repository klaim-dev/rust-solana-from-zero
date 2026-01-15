use crate::config::models::EnvConfig;
use std::path::PathBuf;
pub fn load_env() -> EnvConfig {
    EnvConfig {
        pipeline_raw: std::env::var("PIPELINE").ok(),
        data_file: std::env::var("DATA_FILE").ok().map(PathBuf::from),
    }
}

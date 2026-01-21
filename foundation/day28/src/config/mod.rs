use std::path::PathBuf;

const DEFAULT_FILE: &str = "orders.txt";
const ENV_ORDERS_FILE: &str = "ORDERS_FILE";

#[derive(Debug, Clone)]
pub struct Config {
    pub storage_path: PathBuf,
}

impl Config {
    pub fn load(file_override: Option<PathBuf>) -> Self {
        let storage_path = file_override
            .or_else(env_override)
            .unwrap_or_else(|| PathBuf::from(DEFAULT_FILE));
        Self { storage_path }
    }
}

fn env_override() -> Option<PathBuf> {
    std::env::var(ENV_ORDERS_FILE)
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .map(PathBuf::from)
}

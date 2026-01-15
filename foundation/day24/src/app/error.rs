use crate::config::error::ConfigError;
use crate::pipeline::error::BuildError;
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error(transparent)]
    Config(#[from] ConfigError),
    #[error(transparent)]
    Build(#[from] BuildError),
}

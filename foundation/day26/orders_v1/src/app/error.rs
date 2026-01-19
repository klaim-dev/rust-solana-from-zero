use crate::domain::error::DomainError;
use crate::persist::error::PersistError;
use thiserror::Error;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum AppError {
    #[error("domain: {0}")]
    Domain(#[from] DomainError),
    #[error("persist: {0}")]
    Persist(#[from] PersistError),
}

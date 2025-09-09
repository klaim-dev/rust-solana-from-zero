
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum UserError {
    #[error("name is empty after normalization")]
    EmptyName,
    #[error("name is too long (max {max})")]
    NameTooLong { max: usize },
    #[error("age out of range (0..={max})")]
    AgeOutOfRange { max: u32 },
    #[error("email format is invalid")]
    InvalidEmail,
    #[error("age overflow at max")]
    AgeOverflow,
}

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum CsvError {
    #[error("invalid arity: expected {expected}, got {got}")]
    WrongArity { expected: usize, got: usize },

    #[error("invalid user id: `{0}`")]
    InvalidUserId(String),

    #[error("user id cannot be zero")]
    UserIdZero,

    #[error("invalid action: `{0}`")]
    InvalidAction(String),

    #[error("invalid value: `{0}`")]
    InvalidValue(String),
}

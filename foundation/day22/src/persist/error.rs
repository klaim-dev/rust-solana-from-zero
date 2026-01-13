use crate::domain::index::IndexError;
use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseLineError {
    /// Token format is not key=value
    /// Examples: "id1", "=10", "id="
    #[error("invalid token format: `{0}` (expected key=value)")]
    InvalidTokenFormat(String),

    /// Key is not in the allowed set
    #[error("unknown key `{key}`")]
    UnknownKey { key: String },

    /// Key exists but value is empty
    #[error("empty value for key `{key}`")]
    EmptyValue { key: String },

    /// Duplicate key encountered
    #[error("duplicate key `{key}`")]
    DuplicateKey { key: String },

    /// Missing required key after parsing
    #[error("missing required key `{key}`")]
    MissingKey { key: &'static str },

    /// Value exists but cannot be parsed into correct type
    #[error("bad value for key `{key}`: {reason}")]
    BadValue { key: String, reason: String },
}

#[derive(Debug, Error)]
pub enum PersistError {
    #[error("io error: {0}")]
    Io(#[from] io::Error),

    #[error("invalid line {line_no}: {line}")]
    InvalidLine {
        line_no: usize,
        line: String,
        #[source]
        source: ParseLineError,
    },

    #[error("insert error at line {line_no}: {line}")]
    Insert {
        line_no: usize,
        line: String,
        #[source]
        source: IndexError,
    },

    #[error("invalid path: {0}")]
    InvalidPath(String),
}

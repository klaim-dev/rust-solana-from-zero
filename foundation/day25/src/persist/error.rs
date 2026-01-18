use std::{io, path::PathBuf};

use crate::domain::error::DomainError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PersistError {
    #[error("unknown record kind at line {line_no} (expected ORDER or ITEM)")]
    UnknownKind { line_no: usize },
    #[error("missing '=' in token at line {line_no}: `{token}` (expected key=value)")]
    MissingEquals { line_no: usize, token: String },
    #[error("empty field key at line {line_no} (expected key=value)")]
    EmptyKey { line_no: usize },
    #[error("duplicate field `{field}` at line {line_no}")]
    DuplicateField { line_no: usize, field: String },
    #[error("unclosed quote at line {line_no}")]
    UnclosedQuote { line_no: usize },
    #[error("missing required field `{field}` at line {line_no}")]
    MissingField { line_no: usize, field: &'static str },
    #[error("invalid integer for `{field}` at line {line_no}: `{input}`")]
    InvalidInt {
        line_no: usize,
        field: &'static str,
        input: String,
    },
    #[error(
        "invalid status at line {line_no}: `{input}` (expected draft, confirmed, or cancelled)"
    )]
    InvalidStatus { line_no: usize, input: String },
    #[error("invalid value for `{field}` at line {line_no}: `{input}`")]
    InvalidFieldValue {
        line_no: usize,
        field: &'static str,
        input: String,
    },
    #[error("io error at {path:?}: {source}")]
    Io {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("domain error at line {line_no}: {source}")]
    Domain {
        line_no: usize,
        #[source]
        source: DomainError,
    },
    #[error("io error at '{path}': {source}")]
    IO {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
}

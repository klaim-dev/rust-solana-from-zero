use thiserror::Error;

use catalog::domain::errors::CatalogError;

pub type CliResult<T> = Result<T, CliError>;

#[derive(Debug, Error, PartialEq)]
pub enum CliError {
    // ─────────────────────────────────────────────────────────────
    // Input / parsing
    // ─────────────────────────────────────────────────────────────
    #[error("empty input")]
    EmptyLine,

    #[error("bad token (expected key=value): `{token}`")]
    BadToken { token: String },

    #[error("empty key in token: `{token}`")]
    EmptyKey { token: String },

    #[error("empty value in token: `{token}`")]
    EmptyValue { token: String },

    #[error("invalid quoted string in token: `{value}`")]
    InvalidQuotedString { key: String, value: String },

    #[error("invalid input : {key} , {raw} ")]
    InvalidInt { key: String, raw: String },

    #[error("invalid input, max can be bigger than min: {min} > {max}")]
    InvalidRange { min: u64, max: u64 },

    // ─────────────────────────────────────────────────────────────
    // CLI semantics (command/fields contract)
    // ─────────────────────────────────────────────────────────────
    #[error("unknown command: `{cmd}`")]
    UnknownCommand { cmd: String },

    #[error("invalid category: `{raw}`")]
    InvalidCategory { raw: String },

    // User provided a field that doesn't exist in the command
    #[error("unknown field: `{field}`")]
    UnknownField { field: String },

    // Required field not provided (fixed field name)
    #[error("missing field: `{field}`")]
    MissingField { field: &'static str },

    // Field provided twice (for fixed fields like email/password)
    #[error("duplicate field: `{field}`")]
    DuplicateField { field: &'static str },

    // Duplicate key for arbitrary k=v tokens (dynamic keys)
    #[error("duplicate key: `{key}`")]
    DuplicateKey { key: String },

    // Value failed validation according to field rules
    #[error("bad value for `{field}`: `{value}`")]
    BadValue { field: &'static str, value: String },

    #[error("not found: `{what}`: `{value}`")]
    NotFound { what: &'static str, value: String },

    #[error("mutual exclusive fields: `{a}`, `{b}`")]
    MutuallyExclusiveFields { a: String, b: String },

    // ─────────────────────────────────────────────────────────────
    // Domain bridging
    // ─────────────────────────────────────────────────────────────
    #[error(transparent)]
    Catalog(#[from] CatalogError),
}

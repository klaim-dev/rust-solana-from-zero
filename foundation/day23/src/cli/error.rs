use crate::config::ConfigError;
use crate::domain::error::ItemErr;
use crate::domain::index::IndexError;
use crate::persist::error::{ParseLineError, PersistError};

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum UsageError {
    #[error("missing command: {input}")]
    MissingCommand { input: String },
    #[error("unknown command: {input}")]
    UnknownCommand { input: String },
    #[error("unknown flag: {flag}")]
    UnknownFlag { flag: String },
    #[error("missing value: {flag}")]
    MissingValue { flag: String },
    #[error("unexpected flag value: {flag}, got : {got}")]
    UnexpectedFlagValue { flag: String, got: String },
    #[error("missing required flag: {flag}")]
    MissingRequiredFlag { flag: String },
    #[error("empty file path; {input}")]
    EmptyFilePath { input: String },
    #[error("unexpected argument: {input}")]
    UnexpectedArg { input: String },
    #[error("invalid sort")]
    InvalidSort { input: String },
    #[error("invalid page size: {input}")]
    InvalidPageSize { input: String },
}

#[derive(Debug, thiserror::Error)]
pub enum RunError {
    #[error("missing --file for add")]
    MissingFileForAdd,

    #[error("invalid id '{input}': {reason}")]
    InvalidId { input: String, reason: String },

    #[error("invalid price '{input}': {reason}")]
    InvalidPrice { input: String, reason: String },

    #[error("invalid sku: {reason}")]
    InvalidSku { reason: String },

    #[error(transparent)]
    Item(#[from] ItemErr),

    #[error(transparent)]
    Index(#[from] IndexError),

    #[error(transparent)]
    Persist(#[from] PersistError),

    #[error("config error: {0}")]
    Config(#[from] ConfigError),

    #[error("--file is <none>")]
    MissingFileForPrint,

    #[error("{0}")]
    ParseLineError(#[from] ParseLineError),
}


#[derive(Debug)]
pub enum CliError {
    Usage(UsageError),
    Run(RunError),
}

impl CliError {
    pub fn exit_code(&self) -> i32 {
        match self {
            Self::Usage(..) => 2,
            Self::Run(..) => 1,
        }
    }

    pub fn render(&self, program: &str) -> String {
        match self {
            Self::Usage(ue) => {
                format!("{}: error: {}\nTry '{} --help'", program, ue, program)
            }
            Self::Run(pe) => {
                format!("{}: error: {pe}\n", program)
            }
        }
    }
}

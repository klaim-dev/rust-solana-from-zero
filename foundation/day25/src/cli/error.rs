use thiserror::Error;

#[derive(Debug, Error)]
pub enum UsageError {
    #[error("missing command")]
    MissingCommand,
    #[error("unknown command: {0} (try --help)")]
    UnknownCommand(String),
    #[error("unknown flag: {0} (try --help)")]
    UnknownFlag(String),
    #[error("unexpected flag for {cmd}: {flag} (try --help)")]
    UnexpectedFlagForCommand { cmd: String, flag: &'static str },
    #[error("missing value for flag: {0}")]
    MissingFlagValue(String),
    #[error("missing required flag: {0}")]
    MissingRequiredFlag(&'static str),
    #[error("invalid integer for {flag}: {input}")]
    InvalidInt { flag: &'static str, input: String },
    #[error("invalid value for {flag}: {input} ({reason})")]
    InvalidValue {
        flag: &'static str,
        input: String,
        reason: String,
    },
    #[error("showing help")]
    HelpRequested,
}

use thiserror::Error;

#[derive(Debug, Error)]
pub enum UsageError {
    #[error("missing command (try --help)")]
    MissingCommand,
    #[error("unknown command: {0} (try --help)")]
    UnknownCommand(String),
    #[error("unknown flag: {0} (try --help)")]
    UnknownFlag(String),
    #[error("unexpected flag for {cmd}: {flag} (try --help)")]
    UnexpectedFlagForCommand { cmd: String, flag: &'static str },
    #[error("missing value for flag: {0} (try --help)")]
    MissingFlagValue(String),
    #[error("missing required flag: {0} (try --help)")]
    MissingRequiredFlag(&'static str),
    #[error("invalid integer for {flag}: {input} (try --help)")]
    InvalidInt { flag: &'static str, input: String },
    #[error("invalid value for {flag}: {input} ({reason}) (try --help)")]
    InvalidValue {
        flag: &'static str,
        input: String,
        reason: String,
    },
    #[error("showing help")]
    HelpRequested,
}

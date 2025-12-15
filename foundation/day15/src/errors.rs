#[derive(Debug, thiserror::Error, Clone, PartialEq, Eq)]
pub enum ConfigError {
    #[error("missing required config key: {key}")]
    MissingKey { key: String },

    #[error("config key '{key}' is present but empty")]
    EmptyValue { key: String },

    #[error("invalid u16 for key '{key}': '{raw}'")]
    InvalidU16 { key: String, raw: String },

    #[error("invalid u32 for key '{key}': '{raw}'")]
    InvalidU32 { key: String, raw: String },

    #[error("invalid bool for key '{key}': '{raw}' (expected true/false/1/0)")]
    InvalidBool { key: String, raw: String },

    #[error("invalid mode for key '{key}': '{raw}'")]
    InvalidMode { key: String, raw: String },
}

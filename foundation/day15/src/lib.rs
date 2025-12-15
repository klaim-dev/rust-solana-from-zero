pub mod domain;
pub mod errors;
pub mod kv;
pub mod resolver;

pub use domain::{Config, Mode};
pub use errors::ConfigError;
pub use resolver::build_config;

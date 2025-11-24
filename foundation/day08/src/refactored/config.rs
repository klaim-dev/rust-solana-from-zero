#[derive(Debug, PartialEq)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub use_tls: bool,
    pub timeout_ms: u64,
}

#[derive(Default, Debug)]
struct PartialConfig {
    host: Option<String>,
    port: Option<u16>,
    use_tls: Option<bool>,
    timeout_ms: Option<u64>,
}

#[derive(thiserror::Error, Debug)]
pub enum ConfigError {
    #[error("empty line")]
    EmptyLine,
    #[error("missing '=' separator in line: {0}")]
    MissingSeparator(String),
    #[error("unknown key: {0}")]
    UnknownKey(String),
    #[error("duplicate key: {0}")]
    DuplicateKey(String),
    #[error("invalid port: {0}")]
    InvalidPort(String),
    #[error("invalid timeout: {0}")]
    InvalidTimeout(String),
    #[error("invalid boolean: {0}")]
    InvalidBool(String),
    #[error("missing required key: {0}")]
    MissingRequiredKey(&'static str),
}

pub fn parse_config_line(line: &str) -> Result<(String, String), ConfigError> {
    if line.trim().is_empty() {
        return Err(ConfigError::EmptyLine);
    }

    let parts = line.split('=').collect::<Vec<&str>>();
    if parts.len() != 2 {
        return Err(ConfigError::MissingSeparator(line.to_string()));
    }

    let key = parts[0].trim().to_string();
    let value = parts[1].trim().to_string();
    Ok((key, value))
}

fn apply_config_kv(cfg: &mut PartialConfig, key: &str, value: &str) -> Result<(), ConfigError> {
    match key {
        "host" => {
            if cfg.host.is_some() {
                return Err(ConfigError::DuplicateKey(key.to_string()));
            }
            cfg.host = Some(value.to_string());
        }
        "port" => {
            if cfg.port.is_some() {
                return Err(ConfigError::DuplicateKey(key.to_string()));
            }
            let port_u16 = parse_port(value)?;
            cfg.port = Some(port_u16);
        }
        "use_tls" => {
            if cfg.use_tls.is_some() {
                return Err(ConfigError::DuplicateKey(key.to_string()));
            }
            let tls_bool = parse_bool(value)?;
            cfg.use_tls = Some(tls_bool);
        }

        "timeout_ms" => {
            if cfg.timeout_ms.is_some() {
                return Err(ConfigError::DuplicateKey(key.to_string()));
            }
            let timeout_ms_u64 = parse_timeout(value)?;
            cfg.timeout_ms = Some(timeout_ms_u64);
        }
        _ => {
            return Err(ConfigError::UnknownKey(key.to_string()));
        }
    }
    Ok(())
}

pub fn parse_config(input: &str) -> Result<Config, ConfigError> {
    if input.lines().all(|l| l.trim().is_empty()) {
        return Err(ConfigError::EmptyLine);
    }
    let mut domain = PartialConfig::default();
    for line in input.lines() {
        let (key, value) = parse_config_line(line)?;
        apply_config_kv(&mut domain, &key, &value)?;
    }

    let host = domain.host.ok_or(ConfigError::MissingRequiredKey("host"))?;
    let port = domain.port.ok_or(ConfigError::MissingRequiredKey("port"))?;
    let use_tls = domain
        .use_tls
        .ok_or(ConfigError::MissingRequiredKey("use_tls"))?;
    let timeout_ms = domain
        .timeout_ms
        .ok_or(ConfigError::MissingRequiredKey("timeout_ms"))?;

    let config = Config {
        host,
        port,
        use_tls,
        timeout_ms,
    };
    Ok(config)
}

fn parse_port(value: &str) -> Result<u16, ConfigError> {
    value
        .parse::<u16>()
        .map_err(|_| ConfigError::InvalidPort(value.to_string()))
}

fn parse_bool(value: &str) -> Result<bool, ConfigError> {
    match value {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(ConfigError::InvalidBool(value.to_string())),
    }
}

fn parse_timeout(value: &str) -> Result<u64, ConfigError> {
    value
        .parse::<u64>()
        .map_err(|_| ConfigError::InvalidTimeout(value.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_config() {
        let input = "host=localhost\nport=8080\nuse_tls=true\ntimeout_ms=5000";
        let config = parse_config(input).unwrap();
        assert_eq!(config.host, "localhost");
        assert_eq!(config.port, 8080);
        assert_eq!(config.use_tls, true);
        assert_eq!(config.timeout_ms, 5000);
    }

    #[test]
    fn test_missing_required_key() {
        let input = "host=localhost\nport=8080\nuse_tls=true";
        let err = parse_config(input).unwrap_err();
        match err {
            ConfigError::MissingRequiredKey(key) => assert_eq!(key, "timeout_ms"),
            _ => panic!("Unexpected error: {:?}", err),
        }
    }

    #[test]
    fn test_invalid_port() {
        let input = "host=localhost\nport=99999\nuse_tls=true\ntimeout_ms=5000";
        let err = parse_config(input).unwrap_err();
        match err {
            ConfigError::InvalidPort(val) => assert_eq!(val, "99999"),
            _ => panic!("Unexpected error: {:?}", err),
        }
    }

    #[test]
    fn test_invalid_bool() {
        let input = "host=localhost\nport=8080\nuse_tls=yes\ntimeout_ms=5000";
        let err = parse_config(input).unwrap_err();
        match err {
            ConfigError::InvalidBool(val) => assert_eq!(val, "yes"),
            _ => panic!("Unexpected error: {:?}", err),
        }
    }

    #[test]
    fn test_invalid_timeout() {
        let input = "host=localhost\nport=8080\nuse_tls=true\ntimeout_ms=-5";
        let err = parse_config(input).unwrap_err();
        match err {
            ConfigError::InvalidTimeout(val) => assert_eq!(val, "-5"),
            _ => panic!("Unexpected error: {:?}", err),
        }
    }

    #[test]
    fn test_duplicate_key() {
        let input = "host=localhost\nport=8080\nuse_tls=true\ntimeout_ms=5000\nhost=example.com";
        let err = parse_config(input).unwrap_err();
        match err {
            ConfigError::DuplicateKey(key) => assert_eq!(key, "host"),
            _ => panic!("Unexpected error: {:?}", err),
        }
    }

    #[test]
    fn test_unknown_key() {
        let input = "host=localhost\nport=8080\nuse_tls=true\ntimeout_ms=5000\nunknown=value";
        let err = parse_config(input).unwrap_err();
        match err {
            ConfigError::UnknownKey(key) => assert_eq!(key, "unknown"),
            _ => panic!("Unexpected error: {:?}", err),
        }
    }

    #[test]
    fn test_empty_line_in_middle() {
        let input = "host=localhost\n\nport=8080\nuse_tls=true\ntimeout_ms=5000";
        let err = parse_config(input).unwrap_err();
        match err {
            ConfigError::EmptyLine => (),
            _ => panic!("Unexpected error: {:?}", err),
        }
    }

    #[test]
    fn test_missing_separator() {
        let input = "hostlocalhost";
        let err = parse_config(input).unwrap_err();
        match err {
            ConfigError::MissingSeparator(line) => assert_eq!(line, "hostlocalhost"),
            _ => panic!("Unexpected error: {:?}", err),
        }
    }

    #[test]
    fn test_whitespace_handling() {
        let input = "host =  localhost \n port = 8080 \n use_tls = true \n timeout_ms = 5000 ";
        let config = parse_config(input).unwrap();
        assert_eq!(config.host, "localhost");
        assert_eq!(config.port, 8080);
        assert_eq!(config.use_tls, true);
        assert_eq!(config.timeout_ms, 5000);
    }
}

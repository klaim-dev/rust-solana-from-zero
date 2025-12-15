use crate::domain::{Config, Mode};
use crate::errors::ConfigError;
use crate::kv::Kv;
use std::collections::HashMap;

pub fn build_config(map: &HashMap<String, String>) -> Result<Config, ConfigError> {
    let kv = Kv::new(map);
    let db_url = kv.required("DB_URL")?.to_string();
    let port = kv.optional_u16("PORT")?.unwrap_or(8080);

    let debug = kv.optional_bool("DEBUG")?.unwrap_or(false);

    let max_connections = kv.optional_u32("MAX_CONNECTIONS")?.unwrap_or(10);

    let mode = kv
        .optional("MODE")
        .map(|raw| parse_mode("MODE", raw))
        .transpose()?
        .unwrap_or(Mode::Dev);

    Ok(Config::new(port, debug, db_url, max_connections, mode))
}

fn parse_mode(key: &str, raw: &str) -> Result<Mode, ConfigError> {
    let s = raw.trim();
    let norm = s.to_ascii_lowercase();

    match norm.as_str() {
        "dev" => Ok(Mode::Dev),
        "prod" => Ok(Mode::Prod),
        _ => Err(ConfigError::InvalidMode {
            key: key.to_string(),
            raw: s.to_string(),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{Config, Mode};
    use crate::errors::ConfigError;
    use std::collections::HashMap;

    fn hm(pairs: &[(&str, &str)]) -> HashMap<String, String> {
        pairs
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect()
    }

    // ---- happy path ----

    #[test]
    fn build_config_happy_path_all_fields() {
        let map = hm(&[
            ("DB_URL", "postgres://x"),
            ("PORT", "9000"),
            ("DEBUG", "true"),
            ("MAX_CONNECTIONS", "33"),
            ("MODE", "dev"),
        ]);

        let cfg = build_config(&map).unwrap();

        // Prefer direct field checks if Config exposes getters/fields.
        // If it derives PartialEq, comparing against Config::new(...) is perfect.
        assert_eq!(
            cfg,
            Config::new(9000, true, "postgres://x".to_string(), 33, Mode::Dev)
        );
    }

    // ---- required ----

    #[test]
    fn build_config_missing_db_url_is_error() {
        let map = hm(&[("PORT", "8081")]);

        let err = build_config(&map).unwrap_err();
        assert_eq!(
            err,
            ConfigError::MissingKey {
                key: "DB_URL".to_string()
            }
        );
    }

    #[test]
    fn build_config_empty_db_url_is_error() {
        let map = hm(&[("DB_URL", "")]);

        let err = build_config(&map).unwrap_err();
        assert_eq!(
            err,
            ConfigError::EmptyValue {
                key: "DB_URL".to_string()
            }
        );
    }

    // ---- defaults ----

    #[test]
    fn build_config_defaults_when_optionals_missing() {
        let map = hm(&[("DB_URL", "postgres://x")]);

        let cfg = build_config(&map).unwrap();

        assert_eq!(
            cfg,
            Config::new(8080, false, "postgres://x".to_string(), 10, Mode::Dev)
        );
    }

    #[test]
    fn build_config_defaults_when_mode_missing() {
        let map = hm(&[
            ("DB_URL", "postgres://x"),
            ("PORT", "9000"),
            ("DEBUG", "1"),
            ("MAX_CONNECTIONS", "20"),
        ]);

        let cfg = build_config(&map).unwrap();

        assert_eq!(
            cfg,
            Config::new(9000, true, "postgres://x".to_string(), 20, Mode::Dev)
        );
    }

    // ---- PORT ----

    #[test]
    fn build_config_invalid_port_is_error() {
        let map = hm(&[("DB_URL", "postgres://x"), ("PORT", "nope")]);

        let err = build_config(&map).unwrap_err();
        assert_eq!(
            err,
            ConfigError::InvalidU16 {
                key: "PORT".to_string(),
                raw: "nope".to_string()
            }
        );
    }

    #[test]
    fn build_config_port_empty_after_trim_is_error() {
        let map = hm(&[("DB_URL", "postgres://x"), ("PORT", "   ")]);

        let err = build_config(&map).unwrap_err();
        assert_eq!(
            err,
            ConfigError::EmptyValue {
                key: "PORT".to_string()
            }
        );
    }

    // ---- DEBUG ----

    #[test]
    fn build_config_invalid_debug_is_error() {
        let map = hm(&[("DB_URL", "postgres://x"), ("DEBUG", "yes")]);

        let err = build_config(&map).unwrap_err();
        assert_eq!(
            err,
            ConfigError::InvalidBool {
                key: "DEBUG".to_string(),
                raw: "yes".to_string()
            }
        );
    }

    #[test]
    fn build_config_debug_empty_after_trim_is_error() {
        let map = hm(&[("DB_URL", "postgres://x"), ("DEBUG", "   ")]);

        let err = build_config(&map).unwrap_err();
        assert_eq!(
            err,
            ConfigError::EmptyValue {
                key: "DEBUG".to_string()
            }
        );
    }

    // ---- MAX_CONNECTIONS ----

    #[test]
    fn build_config_invalid_max_connections_is_error() {
        let map = hm(&[("DB_URL", "postgres://x"), ("MAX_CONNECTIONS", "abc")]);

        let err = build_config(&map).unwrap_err();
        assert_eq!(
            err,
            ConfigError::InvalidU32 {
                key: "MAX_CONNECTIONS".to_string(),
                raw: "abc".to_string()
            }
        );
    }

    #[test]
    fn build_config_max_connections_empty_after_trim_is_error() {
        let map = hm(&[("DB_URL", "postgres://x"), ("MAX_CONNECTIONS", "   ")]);

        let err = build_config(&map).unwrap_err();
        assert_eq!(
            err,
            ConfigError::EmptyValue {
                key: "MAX_CONNECTIONS".to_string()
            }
        );
    }
}

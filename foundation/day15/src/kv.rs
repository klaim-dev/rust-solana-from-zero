use crate::errors::ConfigError;
pub struct Kv<'a> {
    map: &'a std::collections::HashMap<String, String>,
}

impl<'a> Kv<'a> {
    pub fn new(map: &'a std::collections::HashMap<String, String>) -> Self {
        Self { map }
    }

    pub fn required(&self, key: &str) -> Result<&'a str, ConfigError> {
        let value = self
            .map
            .get(key)
            .ok_or_else(|| ConfigError::MissingKey {
                key: key.to_string(),
            })?
            .as_str();

        if value.is_empty() {
            return Err(ConfigError::EmptyValue {
                key: key.to_string(),
            });
        }

        Ok(value)
    }

    pub fn optional(&self, key: &str) -> Option<&'a str> {
        self.map.get(key).map(|s| s.as_str())
    }

    pub fn required_u16(&self, key: &str) -> Result<u16, ConfigError> {
        let value = self.required(key)?;
        value.parse::<u16>().map_err(|_| ConfigError::InvalidU16 {
            key: key.to_string(),
            raw: value.to_string(),
        })
    }

    pub fn optional_u32(&self, key: &str) -> Result<Option<u32>, ConfigError> {
        let raw = self.optional(key);

        let parsed = raw
            .map(|s| {
                let s = s.trim();
                if s.is_empty() {
                    return Err(ConfigError::EmptyValue {
                        key: key.to_string(),
                    });
                }

                s.parse::<u32>().map_err(|_| ConfigError::InvalidU32 {
                    key: key.to_string(),
                    raw: s.to_string(),
                })
            })
            .transpose();

        parsed
    }

    pub fn optional_u16(&self, key: &str) -> Result<Option<u16>, ConfigError> {
        let raw = self.optional(key);

        let parsed = raw
            .map(|s| {
                let s = s.trim();
                if s.is_empty() {
                    return Err(ConfigError::EmptyValue {
                        key: key.to_string(),
                    });
                }

                s.parse::<u16>().map_err(|_| ConfigError::InvalidU16 {
                    key: key.to_string(),
                    raw: s.to_string(),
                })
            })
            .transpose();

        parsed
    }

    pub fn optional_bool(&self, key: &str) -> Result<Option<bool>, ConfigError> {
        let raw = self.optional(key);
        let matched = raw
            .map(|raw| {
                let s = raw.trim();
                if s.is_empty() {
                    return Err(ConfigError::EmptyValue {
                        key: key.to_string(),
                    });
                }

                let norm = s.to_ascii_lowercase();

                match norm.as_str() {
                    "true" | "1" => Ok(true),
                    "false" | "0" => Ok(false),
                    _other => {
                        return Err(ConfigError::InvalidBool {
                            key: key.to_string(),
                            raw: norm.to_string(),
                        });
                    }
                }
            })
            .transpose();
        matched
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn hm(pairs: &[(&str, &str)]) -> HashMap<String, String> {
        pairs
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect()
    }

    // ---------- required ----------

    #[test]
    fn required_ok() {
        let map = hm(&[("DB_URL", "postgres://x")]);
        let kv = Kv::new(&map);

        let v = kv.required("DB_URL").unwrap();
        assert_eq!(v, "postgres://x");
    }

    #[test]
    fn required_missing_key() {
        let map = hm(&[]);
        let kv = Kv::new(&map);

        let err = kv.required("DB_URL").unwrap_err();
        assert_eq!(
            err,
            ConfigError::MissingKey {
                key: "DB_URL".to_string()
            }
        );
    }

    #[test]
    fn required_empty_value_is_error() {
        let map = hm(&[("DB_URL", "")]);
        let kv = Kv::new(&map);

        let err = kv.required("DB_URL").unwrap_err();
        assert_eq!(
            err,
            ConfigError::EmptyValue {
                key: "DB_URL".to_string()
            }
        );
    }

    // ---------- optional ----------

    #[test]
    fn optional_some() {
        let map = hm(&[("MODE", "dev")]);
        let kv = Kv::new(&map);

        assert_eq!(kv.optional("MODE"), Some("dev"));
    }

    #[test]
    fn optional_none() {
        let map = hm(&[]);
        let kv = Kv::new(&map);

        assert_eq!(kv.optional("MODE"), None);
    }

    // ---------- required_u16 ----------

    #[test]
    fn required_u16_ok() {
        let map = hm(&[("PORT", "8080")]);
        let kv = Kv::new(&map);

        let v = kv.required_u16("PORT").unwrap();
        assert_eq!(v, 8080);
    }

    #[test]
    fn required_u16_invalid() {
        let map = hm(&[("PORT", "nope")]);
        let kv = Kv::new(&map);

        let err = kv.required_u16("PORT").unwrap_err();
        assert_eq!(
            err,
            ConfigError::InvalidU16 {
                key: "PORT".to_string(),
                raw: "nope".to_string()
            }
        );
    }

    #[test]
    fn required_u16_empty_value_is_empty_value_error() {
        let map = hm(&[("PORT", "")]);
        let kv = Kv::new(&map);

        let err = kv.required_u16("PORT").unwrap_err();
        assert_eq!(
            err,
            ConfigError::EmptyValue {
                key: "PORT".to_string()
            }
        );
    }

    // ---------- optional_u32 ----------

    #[test]
    fn optional_u32_none_when_missing() {
        let map = hm(&[]);
        let kv = Kv::new(&map);

        let v = kv.optional_u32("MAX_CONNECTIONS").unwrap();
        assert_eq!(v, None);
    }

    #[test]
    fn optional_u32_ok() {
        let map = hm(&[("MAX_CONNECTIONS", "10")]);
        let kv = Kv::new(&map);

        let v = kv.optional_u32("MAX_CONNECTIONS").unwrap();
        assert_eq!(v, Some(10));
    }

    #[test]
    fn optional_u32_ok_trims_whitespace() {
        let map = hm(&[("MAX_CONNECTIONS", "  42 \n")]);
        let kv = Kv::new(&map);

        let v = kv.optional_u32("MAX_CONNECTIONS").unwrap();
        assert_eq!(v, Some(42));
    }

    #[test]
    fn optional_u32_empty_after_trim_is_error() {
        let map = hm(&[("MAX_CONNECTIONS", "   ")]);
        let kv = Kv::new(&map);

        let err = kv.optional_u32("MAX_CONNECTIONS").unwrap_err();
        assert_eq!(
            err,
            ConfigError::EmptyValue {
                key: "MAX_CONNECTIONS".to_string()
            }
        );
    }

    #[test]
    fn optional_u32_invalid() {
        let map = hm(&[("MAX_CONNECTIONS", "abc")]);
        let kv = Kv::new(&map);

        let err = kv.optional_u32("MAX_CONNECTIONS").unwrap_err();
        assert_eq!(
            err,
            ConfigError::InvalidU32 {
                key: "MAX_CONNECTIONS".to_string(),
                raw: "abc".to_string()
            }
        );
    }

    // ---------- optional_u16 ----------

    #[test]
    fn optional_u16_none_when_missing() {
        let map = hm(&[]);
        let kv = Kv::new(&map);

        let v = kv.optional_u16("PORT").unwrap();
        assert_eq!(v, None);
    }

    #[test]
    fn optional_u16_ok() {
        let map = hm(&[("PORT", "8080")]);
        let kv = Kv::new(&map);

        let v = kv.optional_u16("PORT").unwrap();
        assert_eq!(v, Some(8080));
    }

    #[test]
    fn optional_u16_ok_trims_whitespace() {
        let map = hm(&[("PORT", "  123 \t")]);
        let kv = Kv::new(&map);

        let v = kv.optional_u16("PORT").unwrap();
        assert_eq!(v, Some(123));
    }

    #[test]
    fn optional_u16_empty_after_trim_is_error() {
        let map = hm(&[("PORT", "   ")]);
        let kv = Kv::new(&map);

        let err = kv.optional_u16("PORT").unwrap_err();
        assert_eq!(
            err,
            ConfigError::EmptyValue {
                key: "PORT".to_string()
            }
        );
    }

    #[test]
    fn optional_u16_invalid() {
        let map = hm(&[("PORT", "nope")]);
        let kv = Kv::new(&map);

        let err = kv.optional_u16("PORT").unwrap_err();
        assert_eq!(
            err,
            ConfigError::InvalidU16 {
                key: "PORT".to_string(),
                raw: "nope".to_string()
            }
        );
    }

    // ---------- optional_bool ----------

    #[test]
    fn optional_bool_none_when_missing() {
        let map = hm(&[]);
        let kv = Kv::new(&map);

        let v = kv.optional_bool("DEBUG").unwrap();
        assert_eq!(v, None);
    }

    #[test]
    fn optional_bool_true_variants() {
        let map = hm(&[
            ("A", "true"),
            ("B", "1"),
            ("C", " TRUE "),
            ("D", "\tTrUe\n"),
        ]);
        let kv = Kv::new(&map);

        assert_eq!(kv.optional_bool("A").unwrap(), Some(true));
        assert_eq!(kv.optional_bool("B").unwrap(), Some(true));
        assert_eq!(kv.optional_bool("C").unwrap(), Some(true));
        assert_eq!(kv.optional_bool("D").unwrap(), Some(true));
    }

    #[test]
    fn optional_bool_false_variants() {
        let map = hm(&[
            ("A", "false"),
            ("B", "0"),
            ("C", " FALSE "),
            ("D", "\nFaLsE\t"),
        ]);
        let kv = Kv::new(&map);

        assert_eq!(kv.optional_bool("A").unwrap(), Some(false));
        assert_eq!(kv.optional_bool("B").unwrap(), Some(false));
        assert_eq!(kv.optional_bool("C").unwrap(), Some(false));
        assert_eq!(kv.optional_bool("D").unwrap(), Some(false));
    }

    #[test]
    fn optional_bool_empty_after_trim_is_error() {
        let map = hm(&[("DEBUG", "   ")]);
        let kv = Kv::new(&map);

        let err = kv.optional_bool("DEBUG").unwrap_err();
        assert_eq!(
            err,
            ConfigError::EmptyValue {
                key: "DEBUG".to_string()
            }
        );
    }

    #[test]
    fn optional_bool_invalid() {
        let map = hm(&[("DEBUG", "yes")]);
        let kv = Kv::new(&map);

        let err = kv.optional_bool("DEBUG").unwrap_err();
        assert_eq!(
            err,
            ConfigError::InvalidBool {
                key: "DEBUG".to_string(),
                raw: "yes".to_string()
            }
        );
    }
}

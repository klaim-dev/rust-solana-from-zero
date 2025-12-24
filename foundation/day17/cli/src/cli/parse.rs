use crate::app::error::CliError;
use std::collections::HashMap;

pub fn parse_kv(tokens: &[&str]) -> Result<HashMap<String, String>, CliError> {
    let map = tokens.iter().try_fold(HashMap::new(), |mut map, token| {
        let trimmed_token = token.trim();
        let (raw_key, raw_value) = trimmed_token.split_once('=').ok_or(CliError::BadToken {
            token: token.to_string(),
        })?;

        if raw_key.trim().is_empty() {
            return Err(CliError::EmptyKey {
                token: token.to_string(),
            });
        }

        if raw_value.trim().is_empty() {
            return Err(CliError::EmptyValue {
                token: token.to_string(),
            });
        }

        let key = raw_key.to_lowercase();

        if map.contains_key(&key) {
            return Err(CliError::DuplicateKey { key });
        }

        let value = normalize_value(&key, raw_value)?;

        map.insert(key, value);

        Ok(map)
    });

    map
}

fn normalize_value(key: &str, raw: &str) -> Result<String, CliError> {
    let trimmed = raw.trim();

    if let Some(stripped) = trimmed.strip_prefix('"') {
        let Some(inner) = stripped.strip_suffix('"') else {
            return Err(CliError::InvalidQuotedString {
                key: key.to_string(),
                value: raw.to_string(),
            });
        };

        if inner.contains('"') {
            return Err(CliError::InvalidQuotedString {
                key: key.to_string(),
                value: raw.to_string(),
            });
        }

        return Ok(inner.to_string());
    }

    Ok(trimmed.to_string())
}

fn opt_u64(map: &HashMap<String, String>, key: &str) -> Result<Option<u64>, CliError> {
    let value = map.get(key);
    match value {
        None => Ok(None),
        Some(value) => {
            let parsed = value
                .trim()
                .parse::<u64>()
                .map_err(|_| CliError::InvalidInt {
                    key: key.to_string(),
                    raw: value.clone(),
                })?;
            Ok(Some(parsed))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_kv_happy_path() {
        let tokens = vec!["sku=abc", "price=10", "name=\"Hello\""];
        let map = parse_kv(&tokens).unwrap();
        assert_eq!(*map.get("sku").unwrap(), "abc".to_string());
        assert_eq!(*map.get("price").unwrap(), "10".to_string());
        assert_eq!(*map.get("name").unwrap(), "Hello".to_string());
    }

    #[test]
    fn parse_kv_no_equal() {
        let tokens = vec!["sku=abc", "broken"];
        let err = parse_kv(&tokens).err().unwrap();
        assert_eq!(
            err,
            CliError::BadToken {
                token: "broken".to_string()
            }
        );
    }

    #[test]
    fn parse_kv_duplicate() {
        let tokens = vec!["SKU=abc", "sku=def"];
        let err = parse_kv(&tokens).err().unwrap();
        assert_eq!(
            err,
            CliError::DuplicateKey {
                key: "sku".to_string()
            }
        );
    }

    #[test]
    fn parse_kv_empty_key() {
        let tokens = vec!["=123"];
        let err = parse_kv(&tokens).err().unwrap();
        assert_eq!(
            err,
            CliError::EmptyKey {
                token: "=123".to_string()
            }
        );
    }

    #[test]
    fn parse_kv_bad_quote() {
        let tokens = vec!["name=\"Hello"];
        let err = parse_kv(&tokens).err().unwrap();
        assert_eq!(
            err,
            CliError::InvalidQuotedString {
                key: "name".to_string(),
                value: "\"Hello".to_string()
            }
        )
    }

    #[test]
    fn opt_u64_key_missing() {
        let mut map = HashMap::new();
        map.insert("price".to_string(), "10".to_string());
        assert_eq!(opt_u64(&map, "sell"), Ok(None));
    }

    #[test]
    fn opt_u64_happy_path() {
        let mut map = HashMap::new();
        map.insert("price".to_string(), "10".to_string());
        assert_eq!(opt_u64(&map, "price"), Ok(Some(10)));
    }

    #[test]
    fn opt_u64_key_letters() {
        let mut map = HashMap::new();
        map.insert("price".to_string(), "abc".to_string());
        assert_eq!(
            opt_u64(&map, "price"),
            Err(CliError::InvalidInt {
                key: "price".to_string(),
                raw: "abc".to_string()
            })
        );
    }

    #[test]
    fn parse_kv_empty_value() {
        let tokens = vec!["sku="];
        let err = parse_kv(&tokens).err().unwrap();
        assert_eq!(
            err,
            CliError::EmptyValue {
                token: "sku=".to_string()
            }
        );
    }

    #[test]
    fn parse_kv_quoted_string_with_quotes_inside() {
        let tokens = vec![r#"name="test"value""#];
        let err = parse_kv(&tokens).err().unwrap();
        assert_eq!(
            err,
            CliError::InvalidQuotedString {
                key: "name".to_string(),
                value: r#""test"value""#.to_string()
            }
        );
    }

    #[test]
    fn parse_kv_multiple_quoted_strings() {
        let tokens = vec![r#"name="Hello World""#, r#"desc="Test Description""#];
        let map = parse_kv(&tokens).unwrap();
        assert_eq!(*map.get("name").unwrap(), "Hello World".to_string());
        assert_eq!(*map.get("desc").unwrap(), "Test Description".to_string());
    }

    #[test]
    fn parse_kv_case_insensitive_keys() {
        let tokens = vec!["SKU=abc", "Name=test"];
        let map = parse_kv(&tokens).unwrap();
        assert_eq!(*map.get("sku").unwrap(), "abc".to_string());
        assert_eq!(*map.get("name").unwrap(), "test".to_string());
    }

    #[test]
    fn parse_kv_trimmed_values() {
        let tokens = vec!["sku=  abc  ", "name=  test  "];
        let map = parse_kv(&tokens).unwrap();
        assert_eq!(*map.get("sku").unwrap(), "abc".to_string());
        assert_eq!(*map.get("name").unwrap(), "test".to_string());
    }

    #[test]
    fn parse_kv_empty_tokens() {
        let tokens: Vec<&str> = vec![];
        let map = parse_kv(&tokens).unwrap();
        assert!(map.is_empty());
    }

    #[test]
    fn opt_u64_with_whitespace() {
        let mut map = HashMap::new();
        map.insert("price".to_string(), "  100  ".to_string());
        assert_eq!(opt_u64(&map, "price"), Ok(Some(100)));
    }

    #[test]
    fn opt_u64_negative_number() {
        let mut map = HashMap::new();
        map.insert("price".to_string(), "-10".to_string());
        assert_eq!(
            opt_u64(&map, "price"),
            Err(CliError::InvalidInt {
                key: "price".to_string(),
                raw: "-10".to_string()
            })
        );
    }

    #[test]
    fn opt_u64_overflow() {
        let mut map = HashMap::new();
        map.insert("price".to_string(), "99999999999999999999".to_string());
        assert!(opt_u64(&map, "price").is_err());
    }
}

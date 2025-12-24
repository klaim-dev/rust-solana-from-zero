use std::{collections::HashMap, str::FromStr};

use catalog::domain::item::{Category, ItemId, Sku};

use crate::{app::error::CliError, cli::parse::parse_kv};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    Create(CreateArgs),

    /// get id=<...>
    GetById {
        id: ItemId,
    },

    /// get sku=<...>
    GetBySku {
        sku: Sku,
    },

    /// update id=<...> [sku=...] [name=...] [category=...] [price_cents=...] [active=...]
    Update {
        id: ItemId,
        changes: UpdateItemArgs,
    },

    /// delete id=<...>
    Delete {
        id: ItemId,
    },

    /// list [category=...] [active=...] [sku=...] [name_contains=...] [min_price=...] [max_price=...]
    List {
        filter: ListFilter,
    },

    Help,
    Exit,
}

impl FromStr for Command {
    type Err = CliError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let raw = s.trim();
        if raw.is_empty() {
            return Err(CliError::EmptyLine);
        }
        let mut tokens = raw.split_whitespace();
        let cmd = tokens.next().ok_or(CliError::UnknownCommand {
            cmd: raw.to_string(),
        })?;
        let rest = tokens.collect::<Vec<&str>>();
        let normal_cmd = cmd.to_ascii_lowercase();

        match normal_cmd.as_str() {
            "get" => {
                let map: HashMap<String, String> = parse_kv(&rest)?;
                ensure_allowed_keys(&map, &["id", "sku"])?;

                let has_id = map.contains_key("id");
                let has_sku = map.contains_key("sku");

                match (has_id, has_sku) {
                    (false, false) => {
                        return Err(CliError::MissingField { field: "id|sku" });
                    }
                    (true, true) => {
                        return Err(CliError::MutuallyExclusiveFields {
                            a: "id".to_string(),
                            b: "sku".to_string(),
                        });
                    }
                    (true, false) => {
                        let raw = map
                            .get("id")
                            .ok_or(CliError::MissingField { field: "id" })?;
                        let id = raw.parse::<ItemId>().map_err(|_| CliError::BadValue {
                            field: "id",
                            value: raw.clone(),
                        })?;
                        Ok(Command::GetById { id })
                    }
                    (false, true) => {
                        let raw = map
                            .get("sku")
                            .ok_or(CliError::MissingField { field: "sku" })?;
                        let sku = raw.parse::<Sku>().map_err(|_| CliError::BadValue {
                            field: "sku",
                            value: raw.clone(),
                        })?;
                        Ok(Command::GetBySku { sku })
                    }
                }
            }
            "create" => {
                let map = parse_kv(&rest)?;
                ensure_allowed_keys(&map, &["sku", "name", "category", "price", "active"])?;

                let raw_sku = req_str(&map, "sku")?;
                let sku = raw_sku.parse::<Sku>().map_err(|_| CliError::BadValue {
                    field: "sku",
                    value: raw_sku.to_string(),
                })?;
                let name = req_str(&map, "name")?.to_string();
                let raw_cat = req_str(&map, "category")?;
                let category =
                    raw_cat
                        .parse::<Category>()
                        .map_err(|_| CliError::InvalidCategory {
                            raw: raw_cat.to_string(),
                        })?;
                let raw_price = req_str(&map, "price")?;
                let price = raw_price.parse::<u64>().map_err(|_| CliError::InvalidInt {
                    key: "price".to_string(),
                    raw: raw_price.to_string(),
                })?;
                let active = opt_bool(&map, "active")?.unwrap_or(true);
                Ok(Command::Create(CreateArgs {
                    sku,
                    name,
                    category,
                    price_cents: price,
                    active,
                }))
            }
            "update" => {
                let map = parse_kv(&rest)?;
                ensure_allowed_keys(&map, &["id", "sku", "name", "category", "price", "active"])?;
                let raw_id = req_str(&map, "id")?;
                let id = raw_id.parse::<ItemId>().map_err(|_| CliError::BadValue {
                    field: "id",
                    value: raw_id.to_string(),
                })?;
                let sku = if let Some(value) = opt_str(&map, "sku") {
                    Some(value.parse::<Sku>().map_err(|_| CliError::BadValue {
                        field: "sku",
                        value: value.to_string(),
                    })?)
                } else {
                    None
                };
                let name = if let Some(value) = opt_str(&map, "name") {
                    Some(value.trim().to_string())
                } else {
                    None
                };

                let category = if let Some(value) = opt_str(&map, "category") {
                    Some(
                        value
                            .parse::<Category>()
                            .map_err(|_| CliError::InvalidCategory {
                                raw: value.to_string(),
                            })?,
                    )
                } else {
                    None
                };

                let price = if let Some(value) = opt_str(&map, "price") {
                    Some(value.parse::<u64>().map_err(|_| CliError::InvalidInt {
                        key: "price".to_string(),
                        raw: value.to_string(),
                    })?)
                } else {
                    None
                };

                let active = opt_bool(&map, "active")?;

                let changes = UpdateItemArgs {
                    sku,
                    name,
                    category,
                    price_cents: price,
                    active,
                };
                if changes == UpdateItemArgs::default() {
                    return Err(CliError::BadValue {
                        field: "update",
                        value: "no_changes".into(),
                    });
                }

                Ok(Command::Update { id, changes })
            }
            "list" => {
                let map = parse_kv(&rest)?;
                ensure_allowed_keys(&map, &["sku", "category", "active", "min", "max", "name"])?;
                let sku = if let Some(value) = opt_str(&map, "sku") {
                    Some(value.parse::<Sku>().map_err(|_| CliError::BadValue {
                        field: "sku",
                        value: value.to_string(),
                    })?)
                } else {
                    None
                };

                let category = if let Some(value) = opt_str(&map, "category") {
                    Some(
                        value
                            .parse::<Category>()
                            .map_err(|_| CliError::InvalidCategory {
                                raw: value.to_string(),
                            })?,
                    )
                } else {
                    None
                };

                let name = if let Some(value) = opt_str(&map, "name") {
                    Some(value.trim().to_string())
                } else {
                    None
                };

                let active = opt_bool(&map, "active")?;
                let min = if let Some(value) = opt_str(&map, "min") {
                    Some(value.parse::<u64>().map_err(|_| CliError::BadValue {
                        field: "min",
                        value: value.to_string(),
                    })?)
                } else {
                    None
                };

                let max = if let Some(value) = opt_str(&map, "max") {
                    Some(value.parse::<u64>().map_err(|_| CliError::BadValue {
                        field: "max",
                        value: value.to_string(),
                    })?)
                } else {
                    None
                };

                let filter = ListFilter {
                    category,
                    active,
                    sku,
                    name_contains: name,
                    min_price: min,
                    max_price: max,
                };

                if let (Some(min), Some(max)) = (min, max) {
                    if min > max {
                        return Err(CliError::InvalidRange { min, max });
                    }
                }
                Ok(Command::List { filter })
            }

            "delete" => {
                let map = parse_kv(&rest)?;
                ensure_allowed_keys(&map, &["id"])?;
                let raw_id = req_str(&map, "id")?;
                let id = raw_id.parse::<ItemId>().map_err(|_| CliError::BadValue {
                    field: "id",
                    value: raw_id.to_string(),
                })?;
                Ok(Command::Delete { id })
            }

            "help" => {
                if !rest.is_empty() {
                    return Err(CliError::BadToken {
                        token: rest[0].to_string(),
                    });
                }
                Ok(Command::Help)
            }
            "exit" => {
                if !rest.is_empty() {
                    return Err(CliError::BadToken {
                        token: rest[0].to_string(),
                    });
                }
                Ok(Command::Exit)
            }
            _ => Err(CliError::UnknownCommand {
                cmd: cmd.to_string(),
            }),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateArgs {
    pub sku: Sku,
    pub name: String,
    pub category: Category,
    pub price_cents: u64,
    pub active: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct UpdateItemArgs {
    pub sku: Option<Sku>,
    pub name: Option<String>,
    pub category: Option<Category>,
    pub price_cents: Option<u64>,
    pub active: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ListFilter {
    pub category: Option<Category>,
    pub active: Option<bool>,
    pub sku: Option<Sku>,
    pub name_contains: Option<String>,
    pub min_price: Option<u64>,
    pub max_price: Option<u64>,
}

pub fn ensure_allowed_keys(
    map: &HashMap<String, String>,
    allowed: &[&str],
) -> Result<(), CliError> {
    for key in map.keys() {
        if !allowed.contains(&key.as_str()) {
            return Err(CliError::UnknownField { field: key.clone() });
        }
    }
    Ok(())
}

pub fn req_str<'a>(
    map: &'a HashMap<String, String>,
    key: &'static str,
) -> Result<&'a str, CliError> {
    let value = map.get(key).ok_or(CliError::MissingField { field: key })?;
    Ok(value.as_str())
}

pub fn opt_str<'a>(map: &'a HashMap<String, String>, key: &'static str) -> Option<&'a str> {
    match map.get(key) {
        None => None,
        Some(raw) => Some(raw),
    }
}

pub fn opt_bool(
    map: &HashMap<String, String>,
    key: &'static str,
) -> Result<Option<bool>, CliError> {
    match map.get(key) {
        None => Ok(None),
        Some(raw) => {
            let parsed = raw.parse::<bool>().map_err(|_| CliError::BadValue {
                field: key,
                value: raw.clone(),
            })?;
            Ok(Some(parsed))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_str_create() {
        let expected = CreateArgs {
            sku: "book".parse::<Sku>().unwrap(),
            name: "Alice".to_string(),
            category: Category::Books,
            price_cents: 100,
            active: true,
        };

        let input = r#"create sku=book name="Alice" category=books price=100"#;
        assert_eq!(Command::from_str(input), Ok(Command::Create(expected)));
    }

    #[test]
    fn from_str_get_id() {
        let expected = Command::GetById {
            id: "1".parse::<ItemId>().unwrap(),
        };
        assert_eq!(Command::from_str("get id=1"), Ok(expected));
    }

    #[test]
    fn from_str_get_sku() {
        let expected = Command::GetBySku {
            sku: "AbC".parse::<Sku>().unwrap(),
        };
        assert_eq!(Command::from_str("get sku=abc"), Ok(expected));
    }

    #[test]
    fn from_str_get_unknown_field() {
        assert_eq!(
            Command::from_str("get abc=1"),
            Err(CliError::UnknownField {
                field: "abc".to_string()
            })
        );
    }
    #[test]
    fn from_str_get_without_arguments() {
        assert_eq!(
            Command::from_str("get"),
            Err(CliError::MissingField { field: "id|sku" })
        );
    }
    #[test]
    fn from_str_get_sku_and_id() {
        assert_eq!(
            Command::from_str("get id=1 sku=abc"),
            Err(CliError::MutuallyExclusiveFields {
                a: "id".to_string(),
                b: "sku".to_string()
            })
        );
    }

    #[test]
    fn from_str_create_with_all_fields() {
        let expected = CreateArgs {
            sku: "test-sku".parse::<Sku>().unwrap(),
            name: "TestItem".to_string(),
            category: Category::Electronics,
            price_cents: 5000,
            active: false,
        };
        let input =
            r#"create sku=test-sku name=TestItem category=electronics price=5000 active=false"#;
        assert_eq!(Command::from_str(input), Ok(Command::Create(expected)));
    }

    #[test]
    fn from_str_create_missing_required_field() {
        assert_eq!(
            Command::from_str("create sku=test name=test"),
            Err(CliError::MissingField { field: "category" })
        );
    }

    #[test]
    fn from_str_create_unknown_field() {
        assert_eq!(
            Command::from_str("create sku=test name=test category=books invalid=value"),
            Err(CliError::UnknownField {
                field: "invalid".to_string()
            })
        );
    }

    #[test]
    fn from_str_update_all_fields() {
        let id = "1".parse::<ItemId>().unwrap();
        let expected = UpdateItemArgs {
            sku: Some("new-sku".parse::<Sku>().unwrap()),
            name: Some("NewName".to_string()),
            category: Some(Category::Grocery),
            price_cents: Some(3000),
            active: Some(false),
        };
        let input =
            r#"update id=1 sku=new-sku name=NewName category=grocery price=3000 active=false"#;
        assert_eq!(
            Command::from_str(input),
            Ok(Command::Update {
                id,
                changes: expected
            })
        );
    }

    #[test]
    fn from_str_update_partial_fields() {
        let id = "1".parse::<ItemId>().unwrap();
        let expected = UpdateItemArgs {
            sku: None,
            name: Some("UpdatedName".to_string()),
            category: Some(Category::Books),
            price_cents: None,
            active: None,
        };
        let input = r#"update id=1 name=UpdatedName category=books"#;
        assert_eq!(
            Command::from_str(input),
            Ok(Command::Update {
                id,
                changes: expected
            })
        );
    }

    #[test]
    fn from_str_update_no_changes() {
        assert_eq!(
            Command::from_str("update id=1"),
            Err(CliError::BadValue {
                field: "update",
                value: "no_changes".into()
            })
        );
    }

    #[test]
    fn from_str_update_missing_id() {
        assert_eq!(
            Command::from_str("update name=test"),
            Err(CliError::MissingField { field: "id" })
        );
    }

    #[test]
    fn from_str_delete() {
        let id = "42".parse::<ItemId>().unwrap();
        assert_eq!(
            Command::from_str("delete id=42"),
            Ok(Command::Delete { id })
        );
    }

    #[test]
    fn from_str_delete_missing_id() {
        assert_eq!(
            Command::from_str("delete"),
            Err(CliError::MissingField { field: "id" })
        );
    }

    #[test]
    fn from_str_list_no_filters() {
        let expected = ListFilter {
            category: None,
            active: None,
            sku: None,
            name_contains: None,
            min_price: None,
            max_price: None,
        };
        assert_eq!(
            Command::from_str("list"),
            Ok(Command::List { filter: expected })
        );
    }

    #[test]
    fn from_str_list_with_filters() {
        let expected = ListFilter {
            category: Some(Category::Books),
            active: Some(true),
            sku: Some("test-sku".parse::<Sku>().unwrap()),
            name_contains: Some("rust".to_string()),
            min_price: Some(100),
            max_price: Some(1000),
        };
        let input = r#"list category=books active=true sku=test-sku name=rust min=100 max=1000"#;
        assert_eq!(
            Command::from_str(input),
            Ok(Command::List { filter: expected })
        );
    }

    #[test]
    fn from_str_list_invalid_range() {
        assert_eq!(
            Command::from_str("list min=1000 max=100"),
            Err(CliError::InvalidRange {
                min: 1000,
                max: 100
            })
        );
    }

    #[test]
    fn from_str_help() {
        assert_eq!(Command::from_str("help"), Ok(Command::Help));
        assert_eq!(Command::from_str("  help  "), Ok(Command::Help));
    }

    #[test]
    fn from_str_help_with_args() {
        assert_eq!(
            Command::from_str("help extra"),
            Err(CliError::BadToken {
                token: "extra".to_string()
            })
        );
    }

    #[test]
    fn from_str_exit() {
        assert_eq!(Command::from_str("exit"), Ok(Command::Exit));
        assert_eq!(Command::from_str("  exit  "), Ok(Command::Exit));
    }

    #[test]
    fn from_str_exit_with_args() {
        assert_eq!(
            Command::from_str("exit now"),
            Err(CliError::BadToken {
                token: "now".to_string()
            })
        );
    }

    #[test]
    fn from_str_empty_line() {
        assert_eq!(Command::from_str(""), Err(CliError::EmptyLine));
        assert_eq!(Command::from_str("   "), Err(CliError::EmptyLine));
    }

    #[test]
    fn from_str_unknown_command() {
        assert_eq!(
            Command::from_str("unknown"),
            Err(CliError::UnknownCommand {
                cmd: "unknown".to_string()
            })
        );
    }

    #[test]
    fn from_str_create_invalid_category() {
        assert_eq!(
            Command::from_str("create sku=test name=test category=invalid price=100"),
            Err(CliError::InvalidCategory {
                raw: "invalid".to_string()
            })
        );
    }

    #[test]
    fn from_str_create_invalid_price() {
        assert_eq!(
            Command::from_str("create sku=test name=test category=books price=abc"),
            Err(CliError::InvalidInt {
                key: "price".to_string(),
                raw: "abc".to_string()
            })
        );
    }

    #[test]
    fn from_str_update_invalid_id() {
        assert_eq!(
            Command::from_str("update id=abc name=test"),
            Err(CliError::BadValue {
                field: "id",
                value: "abc".to_string()
            })
        );
    }

    #[test]
    fn from_str_get_invalid_id() {
        assert_eq!(
            Command::from_str("get id=abc"),
            Err(CliError::BadValue {
                field: "id",
                value: "abc".to_string()
            })
        );
    }

    #[test]
    fn from_str_get_invalid_sku() {
        // Empty SKU will be caught by Sku::from_str which returns CatalogError::EmptySku
        // which gets converted to CliError::Catalog
        let result = Command::from_str("get sku=");
        assert!(result.is_err());
    }

    #[test]
    fn opt_bool_true() {
        let mut map = HashMap::new();
        map.insert("active".to_string(), "true".to_string());
        assert_eq!(opt_bool(&map, "active"), Ok(Some(true)));
    }

    #[test]
    fn opt_bool_false() {
        let mut map = HashMap::new();
        map.insert("active".to_string(), "false".to_string());
        assert_eq!(opt_bool(&map, "active"), Ok(Some(false)));
    }

    #[test]
    fn opt_bool_missing() {
        let map = HashMap::new();
        assert_eq!(opt_bool(&map, "active"), Ok(None));
    }

    #[test]
    fn opt_bool_invalid() {
        let mut map = HashMap::new();
        map.insert("active".to_string(), "maybe".to_string());
        assert_eq!(
            opt_bool(&map, "active"),
            Err(CliError::BadValue {
                field: "active",
                value: "maybe".to_string()
            })
        );
    }

    #[test]
    fn req_str_present() {
        let mut map = HashMap::new();
        map.insert("name".to_string(), "test".to_string());
        assert_eq!(req_str(&map, "name"), Ok("test"));
    }

    #[test]
    fn req_str_missing() {
        let map = HashMap::new();
        assert_eq!(
            req_str(&map, "name"),
            Err(CliError::MissingField { field: "name" })
        );
    }

    #[test]
    fn opt_str_present() {
        let mut map = HashMap::new();
        map.insert("name".to_string(), "test".to_string());
        assert_eq!(opt_str(&map, "name"), Some("test"));
    }

    #[test]
    fn opt_str_missing() {
        let map = HashMap::new();
        assert_eq!(opt_str(&map, "name"), None);
    }

    #[test]
    fn ensure_allowed_keys_valid() {
        let mut map = HashMap::new();
        map.insert("sku".to_string(), "test".to_string());
        map.insert("name".to_string(), "test".to_string());
        assert!(ensure_allowed_keys(&map, &["sku", "name"]).is_ok());
    }

    #[test]
    fn ensure_allowed_keys_invalid() {
        let mut map = HashMap::new();
        map.insert("sku".to_string(), "test".to_string());
        map.insert("invalid".to_string(), "test".to_string());
        assert_eq!(
            ensure_allowed_keys(&map, &["sku", "name"]),
            Err(CliError::UnknownField {
                field: "invalid".to_string()
            })
        );
    }
}

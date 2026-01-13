use crate::domain::index::InventoryIndex;
use crate::domain::types::{Item, ItemId, Sku};
use crate::persist::error::ParseLineError;
use crate::persist::error::PersistError;
use std::io;
use std::path::Path;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn read_nonempty_lines(path: &Path) -> io::Result<Vec<String>> {
    let file = File::open(path)?;
    let buf = BufReader::new(file);
    let mut vec = Vec::new();
    for line in buf.lines() {
        match line {
            Ok(line) => {
                let normalized = line.trim_end_matches('\r');
                if !normalized.is_empty() {
                    vec.push(normalized.to_string());
                }
            }
            Err(e) => return Err(e),
        }
    }
    Ok(vec)
}
#[derive(Debug, Default)]
pub struct Parsed {
    id: Option<String>,
    sku: Option<String>,
    name: Option<String>,
    price: Option<String>,
}

pub fn parse_item_line(line: &str) -> Result<Item, ParseLineError> {
    let mut p = Parsed::default();
    
    // Tokenize line handling quoted values
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    let mut after_equals = false;
    
    for ch in line.chars() {
        match ch {
            '"' if after_equals => {
                in_quotes = !in_quotes;
            }
            '=' => {
                current.push(ch);
                after_equals = true;
            }
            ' ' if !in_quotes => {
                if !current.is_empty() {
                    tokens.push(current.clone());
                    current.clear();
                    after_equals = false;
                }
            }
            _ => {
                current.push(ch);
            }
        }
    }
    if !current.is_empty() {
        tokens.push(current);
    }
    
    // Parse tokens
    for tok in tokens {
        let kv = tok.split_once('=');
        let (key, value) = if let Some((k, v)) = kv {
            if k.is_empty() {
                return Err(ParseLineError::InvalidTokenFormat(k.to_string()));
            }
            if v.is_empty() {
                return Err(ParseLineError::EmptyValue { key: k.to_string() });
            }
            (k.to_string(), v.to_string())
        } else {
            return Err(ParseLineError::InvalidTokenFormat(tok.to_string()));
        };
        match key.as_str() {
            "id" => {
                if p.id.is_some() {
                    return Err(ParseLineError::DuplicateKey {
                        key: key.to_string(),
                    });
                }
                p.id = Some(value);
            }
            "sku" => {
                if p.sku.is_some() {
                    return Err(ParseLineError::DuplicateKey {
                        key: key.to_string(),
                    });
                }
                p.sku = Some(value);
            }
            "name" => {
                if p.name.is_some() {
                    return Err(ParseLineError::DuplicateKey {
                        key: key.to_string(),
                    });
                }
                p.name = Some(value);
            }
            "price" => {
                if p.price.is_some() {
                    return Err(ParseLineError::DuplicateKey {
                        key: key.to_string(),
                    });
                }
                p.price = Some(value);
            }
            _ => {
                return Err(ParseLineError::UnknownKey {
                    key: key.to_string(),
                });
            }
        }
    }

    let raw_id =
        p.id.ok_or_else(|| ParseLineError::MissingKey { key: "id" })?;
    let raw_sku = p
        .sku
        .ok_or_else(|| ParseLineError::MissingKey { key: "sku" })?;
    let raw_name = p
        .name
        .ok_or_else(|| ParseLineError::MissingKey { key: "name" })?;
    let raw_price = p
        .price
        .ok_or_else(|| ParseLineError::MissingKey { key: "price" })?;

    let parse_id = raw_id
        .parse::<u64>()
        .map_err(|e| ParseLineError::BadValue {
            key: "id".to_string(),
            reason: e.to_string(),
        })?;
    let id = ItemId::new(parse_id);
    let sku = Sku::try_new(&raw_sku).map_err(|e| ParseLineError::BadValue {
        key: "sku".to_string(),
        reason: e.to_string(),
    })?;
    let price_cents = raw_price
        .parse::<u64>()
        .map_err(|e| ParseLineError::BadValue {
            key: "price".to_string(),
            reason: e.to_string(),
        })?;
    let item =
        Item::try_new(id, sku, &raw_name, price_cents).map_err(|e| ParseLineError::BadValue {
            key: "Item".to_string(),
            reason: e.to_string(),
        })?;
    Ok(item)
}

use std::fmt::Write;

pub fn serialize(idx: &InventoryIndex) -> String {
    let mut all_id: Vec<ItemId> = idx.ids().collect();
    all_id.sort();

    let mut out = String::new();

    for id in all_id {
        // get_by_id should always succeed since we got the ID from ids()
        // but handle it safely to avoid unwrap in production code
        if let Some(item) = idx.get_by_id(id) {
            // Quote name if it contains spaces
            let name = item.get_name();
            let name_str = if name.contains(' ') {
                format!("\"{}\"", name)
            } else {
                name.to_string()
            };

            // writeln! to String is infallible, but the type requires Result
            let _ = writeln!(
                &mut out,
                "id={} sku={} name={} price={}",
                item.get_id(),
                item.get_sku(),
                name_str,
                item.get_price_cents(),
            );
        }
    }

    out
}

pub fn deserialize(text: &str) -> Result<InventoryIndex, PersistError> {
    let mut idx = InventoryIndex::new();
    for (i, raw) in text.lines().enumerate() {
        let line_no = i + 1;
        let line = raw.trim();
        if line.is_empty() {
            continue;
        }
        if line.starts_with('#') {
            continue;
        }
        let item = parse_item_line(line).map_err(|e| PersistError::InvalidLine {
            line_no,
            line: raw.to_string(),
            source: e,
        })?;
        idx.insert(item).map_err(|e| PersistError::Insert {
            line_no,
            line: raw.to_string(),
            source: e,
        })?;
    }
    Ok(idx)
}

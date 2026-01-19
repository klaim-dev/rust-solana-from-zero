use std::{collections::HashMap, fmt::Write, mem::take};

use crate::domain::order::{Order, OrderStatus};
use crate::domain::store::Store;
use crate::domain::types::{OrderId, Qty, Sku};
use crate::persist::error::PersistError;

enum Record {
    Order {
        id: OrderId,
        customer: String,
        status: OrderStatus,
    },
    Item {
        order_id: OrderId,
        sku: Sku,
        qty: Qty,
    },
}

pub fn parse(input: &str) -> Result<Store, PersistError> {
    let mut records = Vec::new();

    for (idx, line) in input.lines().enumerate() {
        let line_no = idx + 1;
        if let Some(rec) = parse_line(line_no, line)? {
            records.push((line_no, rec));
        }
    }

    let mut store = Store::new();

    for (line_no, rec) in records.iter() {
        if let Record::Order {
            id,
            customer,
            status,
        } = rec
        {
            let order =
                Order::new(*id, customer.clone(), *status).map_err(|e| PersistError::Domain {
                    line_no: *line_no,
                    source: e,
                })?;
            store.add_order(order).map_err(|e| PersistError::Domain {
                line_no: *line_no,
                source: e,
            })?;
        }
    }

    for (line_no, rec) in records.iter() {
        if let Record::Item { order_id, sku, qty } = rec {
            store
                .add_item(*order_id, sku.clone(), *qty)
                .map_err(|e| PersistError::Domain {
                    line_no: *line_no,
                    source: e,
                })?;
        }
    }

    Ok(store)
}

fn parse_line(line_no: usize, line: &str) -> Result<Option<Record>, PersistError> {
    let line = line.trim();
    if line.is_empty() || line.starts_with('#') {
        return Ok(None);
    }

    parse_record(line_no, line).map(Some)
}

fn parse_record(line_no: usize, line: &str) -> Result<Record, PersistError> {
    let (kind, rest) = split_at_first_space(line);
    let kv = parse_kv_fields(line_no, rest)?;

    let rec = match kind {
        "ORDER" => parse_order(line_no, &kv)?,
        "ITEM" => parse_item(line_no, &kv)?,
        _ => return Err(PersistError::UnknownKind { line_no }),
    };

    Ok(rec)
}

fn parse_order(line_no: usize, kv: &HashMap<String, String>) -> Result<Record, PersistError> {
    let raw_id = required_field(kv, line_no, "id")?;
    let raw_customer = required_field(kv, line_no, "customer")?;
    let raw_status = required_field(kv, line_no, "status")?;

    if raw_customer.trim().is_empty() {
        return Err(PersistError::InvalidFieldValue {
            line_no,
            field: "customer",
            input: raw_customer.to_string(),
        });
    }

    let id = OrderId::new(parse_u64_field(line_no, "id", raw_id)?)
        .map_err(|e| PersistError::Domain { line_no, source: e })?;
    let status = parse_status(line_no, raw_status)?;

    Ok(Record::Order {
        id,
        customer: raw_customer.to_string(),
        status,
    })
}

fn parse_item(line_no: usize, kv: &HashMap<String, String>) -> Result<Record, PersistError> {
    let raw_order_id = required_field(kv, line_no, "order_id")?;
    let raw_sku = required_field(kv, line_no, "sku")?;
    let raw_qty = required_field(kv, line_no, "qty")?;

    let order_id = OrderId::new(parse_u64_field(line_no, "order_id", raw_order_id)?)
        .map_err(|e| PersistError::Domain { line_no, source: e })?;
    let sku =
        Sku::new(raw_sku.to_string()).map_err(|e| PersistError::Domain { line_no, source: e })?;
    let qty = Qty::new(parse_u32_field(line_no, "qty", raw_qty)?)
        .map_err(|e| PersistError::Domain { line_no, source: e })?;

    Ok(Record::Item { order_id, sku, qty })
}

fn parse_status(line_no: usize, input: &str) -> Result<OrderStatus, PersistError> {
    match input.trim().to_ascii_lowercase().as_str() {
        "draft" => Ok(OrderStatus::Draft),
        "confirmed" => Ok(OrderStatus::Confirmed),
        "cancelled" | "canceled" => Ok(OrderStatus::Cancelled),
        _ => Err(PersistError::InvalidStatus {
            line_no,
            input: input.to_string(),
        }),
    }
}

fn required_field<'a>(
    kv: &'a HashMap<String, String>,
    line_no: usize,
    field: &'static str,
) -> Result<&'a str, PersistError> {
    kv.get(field)
        .map(|value| value.as_str())
        .ok_or(PersistError::MissingField { line_no, field })
}

fn parse_u64_field(line_no: usize, field: &'static str, input: &str) -> Result<u64, PersistError> {
    input.parse::<u64>().map_err(|_| PersistError::InvalidInt {
        line_no,
        field,
        input: input.to_string(),
    })
}

fn parse_u32_field(line_no: usize, field: &'static str, input: &str) -> Result<u32, PersistError> {
    input.parse::<u32>().map_err(|_| PersistError::InvalidInt {
        line_no,
        field,
        input: input.to_string(),
    })
}

fn parse_kv_fields(line_no: usize, rest: &str) -> Result<HashMap<String, String>, PersistError> {
    let tokens = split_kv_tokens(line_no, rest)?;
    let mut kv = HashMap::new();

    for token in tokens {
        let (key, raw) = token.split_once('=').ok_or(PersistError::MissingEquals {
            line_no,
            token: token.to_string(),
        })?;
        let key = key.trim();
        if key.is_empty() {
            return Err(PersistError::EmptyKey { line_no });
        }

        let raw = raw.trim();
        let value = if raw.starts_with('"') {
            if raw.len() < 2 || !raw.ends_with('"') {
                return Err(PersistError::UnclosedQuote { line_no });
            }
            &raw[1..raw.len() - 1]
        } else {
            raw
        };

        if kv.contains_key(key) {
            return Err(PersistError::DuplicateField {
                line_no,
                field: key.to_string(),
            });
        }
        kv.insert(key.to_string(), value.to_string());
    }

    Ok(kv)
}

fn split_at_first_space(line: &str) -> (&str, &str) {
    let res = line.split_once(' ');
    match res {
        Some((kind, rest)) => (kind, rest.trim_start()),
        None => (line.trim(), ""),
    }
}

fn split_kv_tokens(line_no: usize, rest: &str) -> Result<Vec<String>, PersistError> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut in_quote = false;

    for ch in rest.chars() {
        match ch {
            '"' => {
                in_quote = !in_quote;
                current.push(ch);
            }
            ' ' | '\t' if !in_quote => {
                if !current.is_empty() {
                    tokens.push(take(&mut current));
                }
            }
            _ => current.push(ch),
        }
    }
    if in_quote {
        return Err(PersistError::UnclosedQuote { line_no });
    }
    if !current.is_empty() {
        tokens.push(current);
    }

    Ok(tokens)
}

pub fn serialize(store: &Store) -> String {
    let mut out = String::new();
    for order in ordered_orders(store.list_all()) {
        for line in order_lines(order) {
            let _ = writeln!(&mut out, "{line}");
        }
    }
    out
}

pub(crate) fn format_order(order: &Order) -> String {
    order_lines(order).join("\n")
}

pub(crate) fn format_orders<'a>(orders: impl IntoIterator<Item = &'a Order>) -> String {
    let mut blocks = Vec::new();
    for order in ordered_orders(orders) {
        blocks.push(format_order(order));
    }
    blocks.join("\n")
}

fn status_to_str(status: OrderStatus) -> &'static str {
    match status {
        OrderStatus::Draft => "draft",
        OrderStatus::Confirmed => "confirmed",
        OrderStatus::Cancelled => "cancelled",
    }
}

fn quote_always(value: &str) -> String {
    format!("\"{}\"", value)
}

fn ordered_orders<'a>(orders: impl IntoIterator<Item = &'a Order>) -> Vec<&'a Order> {
    let mut orders: Vec<_> = orders.into_iter().collect();
    orders.sort_by_key(|order| order.get_id());
    orders
}

fn order_lines(order: &Order) -> Vec<String> {
    let customer = quote_always(order.get_customer());
    let status = status_to_str(order.get_status());
    let mut lines = Vec::new();
    lines.push(format!(
        "ORDER id={} customer={} status={}",
        order.get_id(),
        customer,
        status
    ));

    let mut items: Vec<_> = order.get_items().collect();
    items.sort_by(|(left, _), (right, _)| left.get().cmp(right.get()));
    for (sku, qty) in items {
        lines.push(format!(
            "ITEM order_id={} sku={} qty={}",
            order.get_id(),
            sku.get(),
            qty.get()
        ));
    }
    lines
}

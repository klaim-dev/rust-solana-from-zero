use std::{collections::HashMap, fmt::Write, mem::take};

use crate::domain::order::{Order, OrderStatus};
use crate::domain::store::Store;
use crate::domain::types::{OrderId, Qty, Sku};
use crate::persist::error::PersistError;

struct OrderRecord {
    id: OrderId,
    customer: String,
    status: OrderStatus,
}

struct ItemRecord {
    order_id: OrderId,
    sku: Sku,
    qty: Qty,
}

enum ParsedRecord {
    Order(OrderRecord),
    Item(ItemRecord),
}

#[derive(Default)]
struct ParsedRecords {
    orders: Vec<(usize, OrderRecord)>,
    items: Vec<(usize, ItemRecord)>,
}

impl ParsedRecords {
    fn push(&mut self, line_no: usize, record: ParsedRecord) {
        match record {
            ParsedRecord::Order(order) => self.orders.push((line_no, order)),
            ParsedRecord::Item(item) => self.items.push((line_no, item)),
        }
    }
}

pub fn parse(input: &str) -> Result<Store, PersistError> {
    let ParsedRecords { orders, items } = parse_records(input)?;
    let mut store = Store::new();

    apply_orders(&mut store, orders)?;
    apply_items(&mut store, items)?;

    Ok(store)
}

fn parse_records(input: &str) -> Result<ParsedRecords, PersistError> {
    let mut records = ParsedRecords::default();

    for (idx, line) in input.lines().enumerate() {
        let line_no = idx + 1;
        if let Some(record) = parse_line(line_no, line)? {
            records.push(line_no, record);
        }
    }

    Ok(records)
}

fn apply_orders(store: &mut Store, orders: Vec<(usize, OrderRecord)>) -> Result<(), PersistError> {
    for (line_no, record) in orders {
        let order = Order::new(record.id, record.customer, record.status)
            .map_err(|e| PersistError::Domain { line_no, source: e })?;
        store
            .add_order(order)
            .map_err(|e| PersistError::Domain { line_no, source: e })?;
    }

    Ok(())
}

fn apply_items(store: &mut Store, items: Vec<(usize, ItemRecord)>) -> Result<(), PersistError> {
    for (line_no, record) in items {
        store
            .add_item(record.order_id, record.sku, record.qty)
            .map_err(|e| PersistError::Domain { line_no, source: e })?;
    }

    Ok(())
}

fn parse_line(line_no: usize, line: &str) -> Result<Option<ParsedRecord>, PersistError> {
    let line = line.trim();
    if line.is_empty() || line.starts_with('#') {
        return Ok(None);
    }

    parse_record(line_no, line).map(Some)
}

fn parse_record(line_no: usize, line: &str) -> Result<ParsedRecord, PersistError> {
    let (kind, rest) = split_at_first_space(line);
    let kv = parse_kv_fields(line_no, rest)?;

    match kind {
        "ORDER" => parse_order(line_no, &kv).map(ParsedRecord::Order),
        "ITEM" => parse_item(line_no, &kv).map(ParsedRecord::Item),
        _ => Err(PersistError::UnknownKind { line_no }),
    }
}

fn parse_order(line_no: usize, kv: &HashMap<String, String>) -> Result<OrderRecord, PersistError> {
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

    Ok(OrderRecord {
        id,
        customer: raw_customer.to_string(),
        status,
    })
}

fn parse_item(line_no: usize, kv: &HashMap<String, String>) -> Result<ItemRecord, PersistError> {
    let raw_order_id = required_field(kv, line_no, "order_id")?;
    let raw_sku = required_field(kv, line_no, "sku")?;
    let raw_qty = required_field(kv, line_no, "qty")?;

    let order_id = OrderId::new(parse_u64_field(line_no, "order_id", raw_order_id)?)
        .map_err(|e| PersistError::Domain { line_no, source: e })?;
    let sku =
        Sku::new(raw_sku.to_string()).map_err(|e| PersistError::Domain { line_no, source: e })?;
    let qty = Qty::new(parse_u32_field(line_no, "qty", raw_qty)?)
        .map_err(|e| PersistError::Domain { line_no, source: e })?;

    Ok(ItemRecord { order_id, sku, qty })
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
    let mut kv = HashMap::with_capacity(tokens.len());

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
    let orders = ordered_orders(store.list_all());
    let mut out = String::with_capacity(estimate_capacity(&orders));

    for order in orders {
        write_order_lines(&mut out, order, true);
    }

    out
}

pub(crate) fn format_order(order: &Order) -> String {
    let mut out = String::new();
    write_order_lines(&mut out, order, false);
    out
}

pub(crate) fn format_orders<'a>(orders: impl IntoIterator<Item = &'a Order>) -> String {
    let orders = ordered_orders(orders);
    let mut out = String::with_capacity(estimate_capacity(&orders));

    for (idx, order) in orders.into_iter().enumerate() {
        if idx > 0 {
            out.push('\n');
        }
        write_order_lines(&mut out, order, false);
    }

    out
}

fn estimate_capacity(orders: &[&Order]) -> usize {
    let mut lines = 0usize;
    for order in orders {
        lines += 1 + order.get_items().count();
    }
    lines * 48
}

fn status_to_str(status: OrderStatus) -> &'static str {
    match status {
        OrderStatus::Draft => "draft",
        OrderStatus::Confirmed => "confirmed",
        OrderStatus::Cancelled => "cancelled",
    }
}

fn ordered_orders<'a>(orders: impl IntoIterator<Item = &'a Order>) -> Vec<&'a Order> {
    let mut orders: Vec<_> = orders.into_iter().collect();
    orders.sort_by_key(|order| order.get_id());
    orders
}

fn write_order_lines(out: &mut String, order: &Order, trailing_newline: bool) {
    let customer = order.get_customer();
    let status = status_to_str(order.get_status());
    let _ = write!(
        out,
        "ORDER id={} customer=\"{}\" status={}",
        order.get_id(),
        customer,
        status
    );

    let mut items: Vec<_> = order.get_items().collect();
    items.sort_by(|(left, _), (right, _)| left.get().cmp(right.get()));
    for (sku, qty) in items {
        let _ = write!(
            out,
            "\nITEM order_id={} sku={} qty={}",
            order.get_id(),
            sku.get(),
            qty.get()
        );
    }

    if trailing_newline {
        out.push('\n');
    }
}

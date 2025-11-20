# ✅ DAY 7

# Day 7 — `Result`, `?`, `map_err`, `thiserror` + `Display`

Public learning **skeleton** for Foundation Day 7 — **no solutions** in this repo.

## Focus of the day

- Honest error handling with `Result<T, E>` instead of `panic!/unwrap`
- Pattern: **clean domain error enum** + human-readable `Display`
- Using `?` for clean error propagation
- Using `map_err` to convert foreign errors (`ParseIntError`, etc.) into domain errors
- Library returns `Result`, `main` decides how to show or log errors

## We build on

- Day 6 — Option, match, if-let  
- Day 4–5 — ownership & borrowing  
- Day 1–3 — string handling, clean APIs

---

## 🎯 0) Super Task — `order_parser` module (spec only)

Parse lines like:

```
id;name;qty;price
1;coffee;10;4.50
2;tea;abc;3.00
3; ;5;2.00
```

Into:

- `Vec<Order>` on success  
- A specific domain error (`OrderParseError`) on failure

### Domain Model

```rust
pub struct Order {
    pub id: u32,
    pub name: String,
    pub qty: u32,
    pub price_cents: u32, // 450 == 4.50
}
```

### Domain Errors (via thiserror)

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum OrderParseError {
    #[error("empty line")]
    EmptyLine,

    #[error("invalid field count: expected 4 fields, got {0}")]
    InvalidFieldCount(usize),

    #[error("id must be a positive integer: {0}")]
    InvalidId(String),

    #[error("name must not be empty")]
    EmptyName,

    #[error("quantity must be a non-negative integer: {0}")]
    InvalidQty(String),

    #[error("price must be a number with 2 decimals: {0}")]
    InvalidPrice(String),
}
```

### Baseline API

```rust
pub fn parse_order_line(line: &str) -> Result<Order, OrderParseError>;

pub fn parse_orders(input: &str) -> Result<Vec<Order>, OrderParseError>;
```

### Stretch API (line-aware error)

```rust
#[derive(thiserror::Error, Debug)]
pub enum OrdersError {
    #[error("line {line}: {source}")]
    LineError {
        line: usize,
        #[source]
        source: OrderParseError,
    }
}

pub fn parse_orders_with_line_info(input: &str) -> Result<Vec<Order>, OrdersError>;
```

### Invariants

- No unwrap/expect in production code
- All parsing uses `Result`, `?`, and `map_err`
- Errors contain only the needed owned data
- Parsing functions remain pure; I/O is in `main`

### Acceptance Criteria

**`parse_order_line` handles:**
- empty line
- invalid field count
- invalid id / qty / price
- empty name

**`parse_orders`:**
- collects all valid orders
- stops at first error

**Testing:**
- ≥ 6 tests, ≥ 3 negative

---

## 🧠 1) MVT — Minimal Theory

### 1. Result basics

```rust
fn safe_div(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 { Err("division by zero".into()) }
    else { Ok(a / b) }
}
```

### 2. The `?` operator

```rust
fn parse_id(raw: &str) -> Result<u32, std::num::ParseIntError> {
    let v = raw.parse::<u32>()?;
    Ok(v)
}
```

### 3. `map_err`

```rust
raw.parse::<u32>()
    .map_err(|e| OrderParseError::InvalidId(e.to_string()))
```

### 4. thiserror

```rust
#[derive(thiserror::Error, Debug)]
pub enum ConfigError {
    #[error("missing key: {0}")]
    MissingKey(String),
}
```

### 5. Option vs Result

- `Option<T>` — maybe value, no reason
- `Result<T, E>` — value or explicit reason

Use `Result` for external input validation.

---

## 🔬 2) Micro Tasks (no solutions here)

1. `parse_non_negative(raw: &str) -> Result<u32, String>`
2. `parse_positive_i32` using `?` and `map_err`
3. `parse_pair("a,b")`
4. `must_have_value` (Option → Result)
5. Compare Option-returning and Result-returning APIs

---

## 🧩 3) Mini Task — `line_config` parser

```rust
#[derive(thiserror::Error, Debug)]
pub enum LineConfigError {
    #[error("empty line")]
    EmptyLine,
    #[error("missing '=' separator")]
    MissingSeparator,
    #[error("empty key")]
    EmptyKey,
}

pub fn parse_line(line: &str) -> Result<(String, String), LineConfigError>;
```

### Requirements

- blank/whitespace → `EmptyLine`
- no '=' → `MissingSeparator`
- empty/whitespace key → `EmptyKey`
- trim key and value
- ≥ 4 tests

---

## 🚀 4) Super Task Implementation (90 min)

### Project structure

```
day07_result/
  src/
    main.rs
    parser.rs
  Cargo.toml
```

### `parser.rs` skeleton

```rust
pub fn parse_order_line(line: &str) -> Result<Order, OrderParseError> {
    // TODO
}

pub fn parse_orders(input: &str) -> Result<Vec<Order>, OrderParseError> {
    // TODO
}

pub fn parse_orders_with_line_info(input: &str) -> Result<Vec<Order>, OrdersError> {
    // TODO
}
```

### Tests

- happy-path
- empty line
- invalid field count
- bad id / qty / price / empty name
- parse_orders success
- parse_orders error
- (optional) correct line number in `OrdersError`

---

## 📊 Coverage Matrix

| Topic               | Location             |
| ------------------- | -------------------- |
| Result<T, E>        | all parser functions |
| `?` operator        | parse helpers        |
| `map_err`           | id/qty/price parsing |
| thiserror + Display | domain errors        |
| no unwrap           | parser.rs            |
| negative tests      | unit tests           |

---

## 🧱 Senior Checklist

- ❌ no unwrap/expect in production
- ✅ domain error enum
- ✅ readable Display via thiserror
- ✅ foreign errors wrapped via map_err
- ✅ explicit handling of all invalid cases
- ✅ negative tests present

---

## 📘 Decision Log (after coding)

Write 8–10 lines:

- why specific error variants
- where unwrap was removed
- policy for empty name / price format
- why parser stops on first error
- where thiserror simplified code
- what utilities could be extracted later

---

## 🔁 Retrospective

1. Can you explain Option vs Result using `parse_order_line`?
2. Where will you now use `?` automatically?
3. Can you convert a foreign error into a domain error cleanly with `map_err`?

---

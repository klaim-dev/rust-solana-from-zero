# Day 2 — Arithmetic, Logic, `if/else`

Public learning **skeleton** for Day 2 — no solutions.  
Focus: deterministic reasoning with integers, booleans, and branching logic.

---

## Overview

Core topics:

- integer types & safe ranges  
- arithmetic: `+ - * / %`  
- comparisons: `== != < <= > >=`  
- boolean operators: `&& || !`  
- `if/else` as an expression  
- early returns vs nested conditions  
- avoiding hidden panics (`/0`, overflow in debug)  
- pure functions: clear inputs → clear outputs  

This file is a **spec**, not an answer key.  
Your actual implementation lives in your local copy under `rust/foundation/day02/`.

---

## Super Task

Design a minimal **pricing + discount** module that makes you practice:

- integer math on `u32`
- combined boolean conditions
- `if/else` expressions
- simple rule composition

### API (spec)

```rust
pub fn calc_total(base_price_cents: u32, quantity: u32) -> u32;

pub fn apply_discounts(
    total_cents: u32,
    is_vip: bool,
    is_black_friday: bool,
) -> u32;

pub fn is_big_order(final_cents: u32) -> bool;

pub fn label_order(final_cents: u32, is_big: bool) -> String;
// e.g. "OK", "BIG", "DISCOUNTED BIG"

pub fn calc_final_price_cents(
    base_price_cents: u32,
    quantity: u32,
    is_vip: bool,
    is_black_friday: bool,
) -> u32;
```

### Rules (Day 2 scope)

- `calc_total` = `base_price_cents * quantity`
- VIP → 10 % off  
- Black Friday → 20 % off  
- If both → apply sequentially (do not exceed 100 % total)
- “Big order” → `final_cents ≥ 100_00` (= $100)
- Integer math only — no floats

**No panics:**

- `quantity == 0` → return `0`
- never divide by 0
- overflow acknowledged but not exploited yet

**Baseline:**  
All 5 functions compile and behave deterministically.

**Stretch:**  
- Extract discount logic into helpers  
- Introduce an `OrderSummary` struct

```rust
pub struct OrderSummary {
    pub final_cents: u32,
    pub is_big: bool,
    pub label: String,
}
```

---

## Minimal Viable Theory (MVT)

### 1️⃣ Integer types
- `u32` is enough for cents and counts.  
- avoid signed types unless negative values make sense.

### 2️⃣ Arithmetic
- `+ - * / %` work on `u32`  
- `/` → integer division, `%` → remainder  
- overflow panics in debug, wraps in release  
Keep numbers reasonable.

### 3️⃣ Comparisons & booleans
Combine simple relations with logic:

```rust
let has_discount = is_vip || is_black_friday;
let eligible = has_discount && quantity > 0;
```

### 4️⃣ `if` as expression
Rust has no ternary; `if` returns a value:

```rust
let label = if is_big { "BIG" } else { "OK" };
```

### 5️⃣ Early return pattern
Avoid pyramid-shaped nesting:

```rust
pub fn label_order(final_cents: u32, is_big: bool) -> String {
    if final_cents == 0 {
        return "EMPTY".to_string();
    }
    if is_big {
        return "BIG".to_string();
    }
    "OK".to_string()
}
```

Later days will teach cleaner patterns.

---

## Micro Exercises (no solutions)

1. **Safe division**  
   ```rust
   fn safe_div(a: u32, b: u32) -> u32
   ```  
   - if `b == 0` → return `0`  
   - no panics  

2. **Threshold flag**  
   ```rust
   fn is_high_score(score: u32) -> bool
   ```  
   - returns `true` if ≥ 9000  

3. **Composite condition**  
   ```rust
   fn eligible_for_bonus(is_vip: bool, orders: u32) -> bool
   ```  
   - VIP → true if `orders > 0`  
   - non-VIP → true if `orders ≥ 10`

4. **Label by range**  
   ```rust
   fn temperature_label(t: i32) -> &'static str
   ```  
   - `< 0` → "cold"  
   - `0..=25` → "ok"  
   - `> 25` → "hot"

---

## Mini Challenge

Implement:

```rust
pub fn calc_final_price_cents(
    base_price_cents: u32,
    quantity: u32,
    is_vip: bool,
    is_black_friday: bool,
) -> u32
```

Use:
- `calc_total`
- `apply_discounts`
(no duplicated logic)

Add a few `assert_eq!` self-checks:

- no discounts  
- VIP only  
- Black Friday only  
- both flags  
- zero quantity  

---

## Suggested Layout

```text
rust/foundation/day02/
  Cargo.toml
  src/
    main.rs
    pricing.rs
```

**`pricing.rs` (public skeleton)**

```rust
pub fn calc_total(base_price_cents: u32, quantity: u32) -> u32 { todo!() }

pub fn apply_discounts(
    total_cents: u32,
    is_vip: bool,
    is_black_friday: bool,
) -> u32 { todo!() }

pub fn is_big_order(final_cents: u32) -> bool { todo!() }

pub fn label_order(final_cents: u32, is_big: bool) -> String { todo!() }

pub fn calc_final_price_cents(
    base_price_cents: u32,
    quantity: u32,
    is_vip: bool,
    is_black_friday: bool,
) -> u32 { todo!() }
```

**`main.rs` (spec)**  
Create scenarios, print base → final → label.  
Keep `todo!()` in public repo; implement privately.

---

## Acceptance Criteria (Baseline)

- all functions compile (once `todo!()` is replaced)  
- no `unwrap` / `expect`  
- no division by zero  
- readable conditions (no nested pyramids)  
- tests/asserts cover:
  - none / single / combined discounts  
  - zero quantity  

**Stretch:**  
- return `OrderSummary` instead of separate values  
- single place for discount rules  

---

## Coverage Matrix

| Topic        | Where              | Check Method            |
|---------------|--------------------|--------------------------|
| Arithmetic    | `calc_total`       | tests, no panics         |
| Logic         | `apply_discounts`  | branch clarity           |
| `if/else`     | `label_order`      | no nesting               |
| Booleans      | discount flags     | combined conditions      |
| Safety        | all functions      | no `/0`, no unwraps      |

---

## Senior Checklist

- ❌ No `unwrap` / `expect`  
- ✅ No division by zero  
- ✅ Magic numbers centralized (percents, thresholds)  
- ✅ Readable logic ready for extension  
- ✅ Feels like a core billing component, not a tutorial  

---

## Decision Log (complete after coding)

- How you kept conditions readable  
- Potential edge cases (0 qty, double discounts)  
- Ideas for refactoring to config-driven rules  
- Any Rust differences you noticed vs other languages  

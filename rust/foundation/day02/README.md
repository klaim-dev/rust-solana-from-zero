# Day 2 — Arithmetic, Comparisons, Boolean Logic, `if/else` (no `match`, no `Option/Result`)

Public learning **skeleton** for Day 2. No solutions — only structure, tasks, and signatures.
Use this as a workbook starter or a forkable template.

---

## Overview

- Numeric types: `i*`, `u*`, `f32`/`f64` (defaults: `i32`, `f64`); literal suffixes (`42u8`, `1_000`)
- Arithmetic & overflow semantics:
  - Debug: overflow → panic
  - Release: overflow → wrap (unless overflow checks enabled)
  - Explicit strategies: `saturating_*`, `wrapping_*`, `overflowing_*` (pairs)
- Comparisons & boolean logic: `== != < <= > >=`, `&& || !` (short-circuiting)
- `if/else` as an expression
- Casts with `as`:
  - int → uint: negatives wrap modulo 2ⁿ (e.g., `-1i8 as u8 == 255`)
  - float → int: saturates (out-of-range clamps; `NaN` → `0`)
  - integer division vs floating-point (`as f64`)
- Constants & shadowing: `const ADULT: u32 = 18;` vs `let` and `mut`

> Day 2 intentionally avoids `match`, loops, and `Option/Result`. Those arrive later.

---

## Learning Goals

Learn to:
- Choose and combine arithmetic strategies (`saturating_*` / `wrapping_*`) safely
- Write clear multi-branch `if/else` logic as expressions
- Use short-circuiting boolean logic to guard computations
- Perform explicit casts and safe divisions with guards
- Assemble a small policy report without panics or hidden overflow

---

## Day Plan

1. **Theory checklist** — confirm core ideas.
2. **Micro‑exercises** — short, focused tasks.
3. **Mini‑challenge** — pure functions with clear criteria.
4. **Super‑task (skeleton)** — policy report + summary.
5. **Wrap‑up** — coverage matrix, decision log.

---

## Deliverables

- `day2/README.md` (this file)
- `src/` skeleton with `domain/`
- Function signatures filled with `todo!()` (compiles; panics at runtime)

> Tip: If this folder is not inside a Cargo project yet, run `cargo init` at the repo root.

---

## Micro‑Exercises (no solutions)

1. **Boolean logic & short‑circuiting**  
   Compute a `must_verify_age` flag based on two conditions (e.g., `age < 21 || name_is_short`).  
   Show that the right-hand side does not run when the left side is `true` with `||`.

2. **Overflow semantics**  
   On `u8`, demonstrate `+ 250` via `wrapping_add` vs `saturating_add`.  
   Record expected outputs; note the difference between debug (panic on overflow) and release (wrap).

3. **`if` as an expression**  
   Compute `tier: &str` via `if/else if/else` (no `match`):  
   `<18 => "minor"`, `18..=25 => "young_adult"`, `26..=64 => "adult"`, `>=65 => "senior"`.

4. **Casts & division**  
   Compute `avg_name_bytes_per_year: f64 = name_len as f64 / age as f64` with a guard for `age == 0` → return `0.0`.

5. **Thresholded discount**  
   `discount_percent: u8` from age (e.g., `>=65 → 20`, else `0`) plus a small **bonus** for long names using `saturating_add` (cap at `50`).

---

## Mini‑Challenge

Create `policy/basics.rs` with pure functions (no printing):

```rust
// Signatures (no implementations)
pub fn tier(age: u32) -> &'static str { /* TODO */ }
pub fn must_verify_age(age: u32, name_len: usize) -> bool { /* TODO */ }
pub fn discount_percent(age: u32, name_len: usize) -> u8 { /* TODO */ }
pub fn risk_score(age: u32, name_len: usize) -> u8 { /* TODO */ } // use saturating/wrapping
pub fn avg_name_bytes_per_year(age: u32, name_len: usize) -> f64 { /* TODO */ }
```

**Criteria:** functions are pure, agree with the micro‑exercise behavior.

(Short `assert_eq!` self‑checks are fine locally; formal unit tests start later.)

---

## Super‑Task (skeleton only)

**Goal:** a tiny **policy engine** that computes age‑based rules/flags and scores for `User` (from Day 1).  
**Use only:** `if/else`, comparisons, boolean logic, `as` casts, and `saturating_*` / `wrapping_*`.

**Input:**

```rust
User { name: String, age: u32 }
```

**Output:**

```rust
pub struct PolicyReport {
    pub tier: &'static str,              // "minor" | "young_adult" | "adult" | "senior"
    pub discount_percent: u8,            // 0..=50 (saturating cap)
    pub must_verify_age: bool,           // based on age and name
    pub risk_score: u8,                  // arithmetic via saturating/wrapping
    pub summary: String,                 // formatted line
}
```

**Engine:**

```rust
// policy/engine.rs
pub fn build_report(u: &crate::domain::user::User) -> PolicyReport {
    /* TODO: call basics.rs helpers; build `summary` via `format!` */
}
```

**Acceptance (baseline):**
- `build_report(&User)` returns a populated `PolicyReport`
- Logic uses only allowed constructs; no `unwrap`/`expect`
- Arithmetic uses explicit `saturating_*` / `wrapping_*` where overflow can occur
- `summary` contains the key computed fields

**Stretch:**
- Extract thresholds to `const` values (e.g., `ADULT_AGE`, `SENIOR_AGE`)
- Add a smooth discount formula: `base + bonus = min(50)` with `as f64` and rounding to `u8`; guard `age == 0`

---

## Repository Structure

```
day2/
  README.md
  src/
    main.rs
    domain/
      mod.rs
      user.rs
    policy/
      mod.rs
      basics.rs
      engine.rs
```

---

## Code Skeleton (compiles; runtime calls will `todo!()`)

**`src/main.rs`**
```rust
mod domain;
mod policy;

fn main() {
    println!("Day 2 skeleton — implement TODOs in policy::basics and policy::engine");
    // Intentionally no runtime logic here for Day 2.
}
```

**`src/domain/mod.rs`**
```rust
pub mod user;
```

**`src/domain/user.rs`**
```rust
pub struct User {
    pub name: String,
    pub age: u32,
}
```

**`src/policy/mod.rs`**
```rust
pub mod basics;
pub mod engine;
```

**`src/policy/basics.rs`**
```rust
pub fn tier(_age: u32) -> &'static str {
    todo!("Return "minor" | "young_adult" | "adult" | "senior" via if/else")
}

pub fn must_verify_age(_age: u32, _name_len: usize) -> bool {
    todo!("Use boolean logic and short-circuiting")
}

pub fn discount_percent(_age: u32, _name_len: usize) -> u8 {
    todo!("Threshold by age; add small bonus; saturating cap at 50")
}

pub fn risk_score(_age: u32, _name_len: usize) -> u8 {
    todo!("Compute with explicit saturating_/wrapping_ arithmetic")
}

pub fn avg_name_bytes_per_year(_age: u32, _name_len: usize) -> f64 {
    todo!("Return 0.0 when age == 0; otherwise name_len as f64 / age as f64")
}
```

**`src/policy/engine.rs`**
```rust
pub struct PolicyReport {
    pub tier: &'static str,
    pub discount_percent: u8,
    pub must_verify_age: bool,
    pub risk_score: u8,
    pub summary: String,
}

pub fn build_report(_u: &crate::domain::user::User) -> PolicyReport {
    todo!("Call basics::* helpers and build a formatted summary with format!")
}
```

---

## Guidelines

- Allowed today: arithmetic, comparisons, boolean logic, `if/else`, casts with `as`
- Avoid today: `match`, loops, `Option/Result`, `unwrap`/`expect`
- Prefer explicit overflow handling (`saturating_*` / `wrapping_*`)
- Use `if` as an expression for clean multi-branch logic


---

# Day 3 — Functions, Parameters/Return, Tuples, `#[cfg(test)]` (First Unit Tests)

Public learning **skeleton** for Day 3 (no solutions).  
Builds on Day 1 (types, strings, memory) and Day 2 (arithmetic, `if/else`, boolean logic).

---

## Overview

- Function signatures: `fn f(arg: T) -> U`
- Passing by value vs by reference (`&str`, `&T`); avoiding `&mut` today
- Return as an expression (last line without `return`/`;`)
- Tuples for multiple return values
- Purity: no printing/mutation inside pure functions
- Modules & visibility: `mod`, `pub`, `use`
- Tests with `#[cfg(test)]`, `#[test]`, `use super::*`, `assert_eq!`
- Floating-point checks with a tolerance: `assert!((a - b).abs() < 1e-9)`

> Intentional constraints for Day 3: **functions only** (no `impl`/methods),  
> **no** `Option/Result` yet (Days 6–7), **no** loops/iterators today.

---

## Learning Goals

Learn to:
- Design small, pure calculation functions with clear signatures
- Return values idiomatically (expression style)
- Use tuples for multi-value returns
- Organize code into modules with minimal `pub` surface
- Write focused unit tests (including `f64` tolerance checks)

---

## Day Plan

1. **Theory checklist** — confirm signature/return rules, purity, visibility.
2. **Micro-exercises** — short tasks to cement syntax and behavior.
3. **Mini-challenge** — a set of pure functions in `metrics/basics.rs` with tests.
4. **Super-task (skeleton)** — aggregator + summary in `metrics/report.rs` with tests.
5. **Wrap-up** — coverage matrix, senior checklist, decision log.

---

## Deliverables

- `day3/README.md` (this file)
- `src/metrics/basics.rs` — pure functions + local tests (no solutions here)
- `src/metrics/report.rs` — aggregator + formatting + local tests
- Uses `domain/user.rs` from Day 1 and logic ideas from Day 2

> Tip: keep all functions **pure**. Printing happens only in `main`.

---

## Micro-Exercises (no solutions)

1. **Return as expression**  
   Rewrite a simple function so the last line is the value (no `return` / trailing `;`).

2. **Tuples**  
   Discuss two signatures for “bytes and first char”:
   - ASCII-only today: `fn bytes_and_first_ascii(name: &str) -> (usize, char)`
   - Unicode-safe (for a later day): `fn bytes_and_first_char(name: &str) -> (usize, Option<char>)`  
   *(Design only; do not implement the `Option` version today.)*

3. **References vs ownership**  
   Prefer `fn avg_name_bytes_per_year(name_len: usize, age: u32) -> f64` over passing `&String`. Explain why.

4. **Purity**  
   Take a function that prints inside and refactor it to return a value instead of side effects.

5. **First test**  
   Write `#[test] fn avg_zero_age_is_zero()` with `f64` tolerance:  
   `assert!((avg - 0.0).abs() < 1e-9)`.

---

## Mini-Challenge

Create `metrics/basics.rs` with **signatures only** (no implementations here):

```rust
// metrics/basics.rs — signatures only
pub fn tier(age: u32) -> &'static str { /* TODO */ }

pub fn must_verify_age(age: u32, name_len: usize) -> bool { /* TODO */ }

pub fn discount_percent(age: u32, name_len: usize) -> u8 { /* TODO: saturating cap (≤ 50) */ }

pub fn risk_score(age: u32, name_len: usize) -> u8 {
    /* TODO: choose explicit strategy from Day 2 (wrapping / saturating) */
}

pub fn avg_name_bytes_per_year(age: u32, name_len: usize) -> f64 {
    /* TODO: guard age == 0 → 0.0; else name_len as f64 / age as f64 */
}
```

Add a colocated test module:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    // 1–2 tests per function; include boundaries from Day 2:
    // 17/18/25/26/64/65; age == 0; long name → discount cap 50;
    // one test demonstrating chosen overflow strategy for risk_score.
    //
    // For f64:
    // assert!((actual - expected).abs() < 1e-9);
}
```

**Criteria:** functions are pure and independently testable.

---

## Super-Task (skeleton only)

Assemble a small **metrics** module that aggregates Day-2-style logic into a single report.

**Structure**
```
src/
  main.rs
  domain/
    user.rs          // from Day 1
  metrics/
    basics.rs        // pure functions + tests (mini-challenge)
    report.rs        // aggregator + formatting + tests
```

**Signatures (report)**
```rust
// metrics/report.rs — signatures only
pub struct UserMetrics {
    pub tier: &'static str,
    pub must_verify_age: bool,
    pub discount_percent: u8,
    pub risk_score: u8,
    pub avg_name_bytes_per_year: f64,
}

pub fn build_metrics(u: &crate::domain::user::User) -> UserMetrics { /* TODO */ }

pub fn format_summary(m: &UserMetrics, u: &crate::domain::user::User) -> String {
    /* TODO: build one formatted line with key fields */
}

#[cfg(test)]
mod tests {
    use super::*;
    // At least: 1 happy + 2 edge tests (e.g., age == 18/65; empty name ⇒ name_len == 0; age == 0)
}
```

**Acceptance (baseline)**
- `build_metrics` composes values **only via** `metrics::basics`
- `format_summary` returns a single, readable line
- **Tests required**:
  - in `metrics::basics`: 1–2 per function (incl. boundaries)
  - in `metrics::report`: ≥1 happy + 2 edge
- No `unwrap`/`expect`; printing only in `main`

**Stretch**
- Extract thresholds into `const` (e.g., `ADULT_AGE`, `SENIOR_AGE`) in `metrics/consts.rs`
- Extra invariant test (e.g., `discount_percent <= 50` always)

---

## Coverage Matrix

```
Topic                           Where                                Check
--------------------------------------------------------------------------------------------
Function signatures / return    basics::* / report::build_*          tests per function
Tuples                          micro-exercise                        compiles / destructuring
Pure functions                  basics::*                             no println!, no side effects
Modules & visibility            metrics::{basics, report}             used from main; minimal pub surface
Unit tests (`#[cfg(test)]`)     basics::tests / report::tests         1–2 per fn, happy+edge in report
f64 tolerance                   avg_name_bytes_per_year               assert!((a-b).abs() < 1e-9)
```

---

## Guidelines

- Keep functions pure; return values instead of printing/mutating
- Use expression returns (last line without `return`)
- Keep the public API minimal and domain-oriented
- Test boundaries and edge cases explicitly

---


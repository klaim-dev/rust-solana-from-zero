# Day 7 — Result / `?` / `ok_or` / `map_err`, `thiserror` + `Display` (Parsing Layer)

Public learning **skeleton** for Day 7 (no solutions).  
Goal: add a safe **parsing/input layer** on top of Day 6, convert system parsing errors into **domain errors** with clean user/tech messages.  
External crate allowed today: **`thiserror`** (for `Error` derive and stable `Display`).

---

## Overview

- `Result<T, E>` vs `Option<T>` (absence ≠ domain failure)
- `?` for early returns; flatten nested conditionals
- `ok_or` / `ok_or_else` to convert `Option → Result`
- `map_err` to translate system errors into domain/parse errors
- `thiserror` derive for readable `Error` + `Display`
- Money ops use **checked** arithmetic (`u64::checked_add/sub`); no silent mint/burn
- No `unwrap/expect/panic` in the production path

> Builds on Day 6: reuse `TransferError`, `validate_transfer`, `apply_transfer_inplace_checked`.

---

## Learning Goals

Learn to:
- Design a **parse layer** that returns typed errors (no booleans)
- Use `?`, `ok_or`, `map_err` idiomatically in facades
- Keep parsing and business errors **separate** (`ParseError` vs `TransferError`)
- Normalize error messages for **users** and **devs**
- Simulate vs apply transfers (side‑effect free vs mutating) cleanly

---

## Day Plan

1. **Theory checklist** — `Result/Option`, `?`, `ok_or`, `map_err`, `thiserror` basics.
2. **Micro‑exercises** — tiny tasks for each primitive.
3. **Mini‑challenge** — error normalization module (`user_message` / `tech_message`) + `ParseError`.
4. **Super‑task (skeleton)** — parsing facades + simulate/apply APIs.
5. **Wrap‑up** — coverage matrix, senior checklist, decision log.

---

## Deliverables

- `day7/README.md` (this file)
- Signatures only (no implementations) for: parse errors, error formatting, safe service facades
- Suggested tests: ≥ 6 negative + 1 happy for services; full branch tests for error formatting

> Add to `Cargo.toml`:
> ```toml
> [dependencies]
> thiserror = "1"
> ```

---

## Micro‑Exercises (no solutions)

1. Turn `slice.get(i)` into a `Result` via `ok_or(...)?`.
2. Parse `u64` with `str::parse::<u64>()` and convert the error using `map_err` → `ParseError::InvalidAmountFormat`.
3. Show why `checked_add/sub` is correct for balances (vs `saturating_*`).
4. Replace `if opt.is_none() { return Err(..) }` with `let Some(v) = opt else { return Err(..) };`.
5. In tests, validate variants with `matches!`: `assert!(matches!(res, Err(TransferError::SameOwner)))`.

---

## Mini‑Challenge — Error Normalization

Create a **parse error** and normalize **domain messages**. Signatures only:

```rust
// defi/parse_error.rs
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ParseError {
    #[error("invalid amount format")]
    InvalidAmountFormat,
    #[error("invalid index format")]
    InvalidIndexFormat,
    #[error("invalid mint format")]
    InvalidMintFormat,
}

// Optional: mapping policy (keep ParseError separate from TransferError by default)
impl From<ParseError> for crate::defi::error::TransferError {
    fn from(e: ParseError) -> Self {
        // TODO: either keep them separate in the service layer,
        // or collapse into a high-level input error if desired.
        crate::defi::error::TransferError::InvalidInput(e.to_string())
    }
}
```

```rust
// defi/error_format.rs
use crate::defi::error::TransferError;

/// Stable user-facing strings (exhaustive match; no `_` wildcard).
pub fn user_message(e: &TransferError) -> &'static str {
    /* TODO */
}

/// Technical detail string with embedded fields (have/need, mints, indices).
pub fn tech_message(e: &TransferError) -> String {
    /* TODO */
}

#[cfg(test)]
mod tests {
    use super::*;
    // One test per TransferError branch, including data-bearing variants like
    // InsufficientFunds, MintMismatch, IndexOutOfBounds, etc.
}
```

**Criteria:** exhaustive `match`, stable wording, tests cover every branch.

---

## Super‑Task (skeleton only)

Public **service facades**: parse input, get disjoint borrows, simulate (no mutation), apply (with mutation).

**Structure**
```
src/
  main.rs
  defi/
    models.rs              // from Days 4–6
    error.rs               // TransferError (Day 6) — no new domain variants here
    parse_error.rs         // NEW (for input)
    validate.rs            // Day 6
    inplace_checked.rs     // Day 6
    error_format.rs        // Mini‑challenge
    service.rs             // NEW: parsing + facades
```

**Signatures (no bodies)**
```rust
// defi/service.rs
use crate::defi::models::{TokenAccount, Pubkey, Lamports, Transfer};
use crate::defi::error::TransferError;
use crate::defi::parse_error::ParseError;

// 1) Parsing primitives (strings → domain types)
pub fn parse_lamports(s: &str) -> Result<Lamports, ParseError> { /* TODO: u64::from_str / str::parse, map_err */ }
pub fn parse_index(s: &str)   -> Result<usize, ParseError> { /* TODO */ }
// For mints: choose a format (e.g., fixed 64-hex) or leave TODO and document policy.
pub fn parse_mint(s: &str)    -> Result<Pubkey, ParseError> { /* TODO */ }

// 2) Disjoint &mut by indices
pub fn get_two_mut_checked<'a>(
    xs: &'a mut [TokenAccount],
    i: usize,
    j: usize,
) -> Result<(&'a mut TokenAccount, &'a mut TokenAccount), TransferError> {
    // TODO: split_at_mut + Option→Result (ok_or / let-else); forbid i == j
}

// 3) Simulation (no mutation)
pub fn simulate_transfer(
    xs: &mut [TokenAccount],
    i: usize,
    j: usize,
    amount: Lamports,
    mint: Pubkey,
) -> Result<(Lamports, Lamports), TransferError> {
    // TODO: validate_transfer(...)?; checked_sub / checked_add → map None → domain error;
    // return new balances only (do not write back)
}

// 4) Application (with mutation)
pub fn apply_transfer(
    xs: &mut [TokenAccount],
    i: usize,
    j: usize,
    amount: Lamports,
    mint: Pubkey,
) -> Result<Transfer, TransferError> {
    // TODO: get_two_mut_checked(...)?; apply_transfer_inplace_checked(...)?; return receipt
}
```

**Acceptance (baseline)**
- Facades use `Result` + `?` + `ok_or` + `map_err`; **no** `unwrap/expect/panic`
- `simulate_transfer` is side‑effect free; `apply_transfer` mutates **after** validation
- All monetary ops are `checked_*`
- Error normalization (`user_message` / `tech_message`) has branch coverage

**Suggested tests (≥ 7)**
1. Happy path: `apply_transfer` updates balances correctly
2. `ZeroAmount`
3. `SameOwner`
4. `MintMismatch`
5. `InsufficientFunds`
6. `IndexOutOfBounds` / (optional) `SameIndex`
7. `OverflowToBalance` near `u64::MAX`
8. Verify `simulate_transfer` does **not** change inputs
9. Three `parse_*` tests mapping to `ParseError::*`

---

## Coverage Matrix

```
Topic                         Where                           Check
---------------------------------------------------------------------------
Result / ?                    service.rs / validate.rs        chains w/o pyramids
ok_or / let-else              get_two_mut_checked             IndexOutOfBounds path
map_err                       parse_*                         Invalid*Format errors
checked_add/sub               simulate/apply                  Overflow / InsufficientFunds
thiserror + Display           parse_error.rs / error.rs       human-readable texts
No unwrap/expect/panic        all production code             review + tests
```

---

## Guidelines

- Keep **ParseError** and **TransferError** separate; collapse only at boundaries if needed
- Prefer `?` over nested `if` chains
- All money arithmetic must be `checked_*`
- Normalize messages in one place (`error_format.rs`)


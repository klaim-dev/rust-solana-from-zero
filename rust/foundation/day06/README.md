# Day 6 — Errors as Contracts: `Result<T, E>`, `Option<T>`, `?`, Domain Error Enum

Public learning **skeleton** for Day 6 (no solutions).  
Solana/Web3 DeFi context: safe token transfers with explicit error handling.  
No external crates (no `anyhow`/`thiserror`) — **standard library only**.

---

## Overview

- When to use `Result<T, E>` vs `Option<T>`
- The `?` operator for early returns (flattening nested conditionals)
- Converting between `Option` and `Result` (`ok_or`, `ok_or_else`)
- Mapping results: `map`, `map_err`, `and_then`
- Checked arithmetic for money: `u64::checked_add/sub` (vs `saturating_*`)
- Domain error design: a flat `enum` + human‑readable `Display`
- No panics in the production path (`panic!/unwrap/expect` forbidden)

---

## Learning Goals

Learn to:
- Replace boolean gates with **typed error contracts**
- Use `?` for readable error propagation
- Convert `Option` ↔ `Result` at module boundaries
- Apply **checked** arithmetic for balances (no implicit minting/burning)
- Express domain invariants as **errors** with helpful messages

---

## Day Plan

1. **Theory checklist** — `Result` vs `Option`, `?`, conversions, checked math.
2. **Micro‑exercises** — short tasks for each primitive.
3. **Mini‑challenge** — `validate_transfer` + error enum and `Display`.
4. **Super‑task (skeleton)** — in‑place transfer with `Result`, plus index‑based API.
5. **Wrap‑up** — coverage matrix, senior checklist, decision log.

---

## Deliverables

- `day6/README.md` (this file)
- Signatures only (no implementations) for: errors, validation, in‑place checked transfer
- Suggest tests (happy + ≥ 6 negative), printing only in higher layers

> Constraints for Day 6: **std only**, no external error crates; forbid `panic!/unwrap/expect` in the production path.

---

## Micro‑Exercises (no solutions)

1. **Option → Result**  
   Turn `slice.get(i)` / `slice.get_mut(i)` into a `Result` via `ok_or(...)?`.

2. **Checked arithmetic**  
   Compare `checked_add/sub` with `saturating_*` on `u64`. Choose the correct semantics for **crediting** balances.

3. **`map_err`**  
   Convert a placeholder system error (e.g., parse/cast) into `TransferError::...`.

4. **`?` chain**  
   Build a three‑step chain: `get_two_mut` → `validate_transfer` → perform the transfer.

5. **Formatting a failure**  
   `match` on a `Result` and produce a concise, human string using `Display` for the error.

---

## Mini‑Challenge

Create a **domain error** and a **validator** (signatures only):

```rust
// defi/error.rs — domain errors (signatures only here)
#[derive(Debug, PartialEq, Eq)]
pub enum TransferError {
    SameOwner,
    MintMismatch { expected: Pubkey, from: Pubkey, to: Pubkey },
    InsufficientFunds { have: Lamports, need: Lamports },
    ZeroAmount,
    IndexOutOfBounds { len: usize, i: usize, j: usize },
    SameIndex,
    OverflowToBalance, // u64 overflow when crediting `to`
    AliasingDetected,  // defensive contract for identical &mut
}

impl core::fmt::Display for TransferError {
    // TODO: human‑readable messages for each variant
}
```

```rust
// defi/validate.rs — pure checks (signatures only here)
use crate::defi::models::{TokenAccount, Pubkey, Lamports};
use crate::defi::error::TransferError;

pub fn validate_transfer(
    from:   &TokenAccount,
    to:     &TokenAccount,
    amount: Lamports,
    mint:   Pubkey,
) -> Result<(), TransferError> {
    /* TODO: enforce invariants; return specific errors; no printing */
}

// Optional helper for readability
pub fn ensure(cond: bool, err: TransferError) -> Result<(), TransferError> {
    /* TODO */
}
```

**Criteria (mini):**
- All invariants are expressed as **errors** (no boolean gates):
  `SameOwner`, `MintMismatch`, `ZeroAmount`, `InsufficientFunds`, `OverflowToBalance` (where applicable)
- No `panic!/unwrap`
- ≥ 3 negative tests for `validate_transfer` (suggested): same owner, mint mismatch, zero amount

---

## Super‑Task (skeleton only)

Move the in‑place transfer engine to a **checked** `Result`‑based API.

**Structure**
```
src/
  main.rs
  defi/
    models.rs
    error.rs
    validate.rs
    inplace_checked.rs
```

**Signatures (no bodies here)**
```rust
// defi/inplace_checked.rs
use crate::defi::models::{TokenAccount, Pubkey, Lamports};
use crate::defi::error::TransferError;
use crate::defi::validate::validate_transfer;

pub fn get_two_mut<'a>(
    xs: &'a mut [TokenAccount],
    i: usize,
    j: usize,
) -> Option<(&'a mut TokenAccount, &'a mut TokenAccount)> {
    /* TODO: split_at_mut; return None for out‑of‑bounds or i == j */
}

pub fn apply_transfer_inplace_checked(
    from:   &mut TokenAccount,
    to:     &mut TokenAccount,
    amount: Lamports,
    mint:   Pubkey,
) -> Result<(), TransferError> {
    /* TODO: validate_transfer(..)?; use checked_sub / checked_add; map None → specific error */
}

pub fn apply_transfer_by_index_checked(
    xs:     &mut [TokenAccount],
    i:      usize,
    j:      usize,
    amount: Lamports,
    mint:   Pubkey,
) -> Result<(), TransferError> {
    /* TODO: get_two_mut(..).ok_or(TransferError::IndexOutOfBounds { len: xs.len(), i, j })?;
             apply_transfer_inplace_checked(..)?; Ok(()) */
}
```

**Acceptance (baseline)**
- All former `assert!/debug_assert!` paths now return **meaningful errors**
- No `panic!/unwrap/expect` in the production path
- `?` used idiomatically; conversions via `ok_or`, `map_err` where appropriate
- `checked_add/sub` used for balances (no silent mint/burn)
- `get_two_mut` does not produce overlapping `&mut` (use `split_at_mut`)

**Suggested tests (≥ 7)**
1. Happy path: successful transfer
2. `ZeroAmount` → error
3. `SameOwner` → error
4. `MintMismatch { .. }` → error
5. `InsufficientFunds { .. }` → error
6. `IndexOutOfBounds { .. }` for any index out of range
7. `OverflowToBalance` with `to.amount` near `u64::MAX`
8. (Optional) `SameIndex` for `i == j` in `get_two_mut`

**Stretch**
- Stable, clear `Display` messages per error
- Tiny utility `format_error(e: &TransferError) -> String`

---

## Coverage Matrix

```
Topic                               Where                                  Check
-----------------------------------------------------------------------------------------------
Result / Option                     validate.rs / inplace_checked.rs        compiles; tests
`?` early return                    validate + apply_*_checked              no nested pyramids
Option → Result (`ok_or`)           get_two_mut → apply_by_index_checked    IndexOutOfBounds path
map / map_err / and_then            conversions & plumbing                  negative cases mapped
checked_add / checked_sub           balance changes                         overflow / insufficient
Domain Error enum + Display         error.rs                                readable messages
No unwrap/expect/panic              production code                         code review + tests
```

---

## Guidelines

- Use `Result` for **contract violations**; `Option` for **absence** (e.g., index lookup)
- Prefer `?` over deeply nested `if` chains
- Use **checked** arithmetic for money; avoid `saturating_*` when crediting balances
- Keep validation **pure** and side‑effect free; printing at higher layers

---


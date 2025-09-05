# Day 4 — Ownership & Move, `Copy` / `Clone`, Partial Move, `Drop` (Solana/Web3 context)

Public learning **skeleton** for Day 4 (no solutions).  
Focus: ownership semantics with Solana‑style models (`Pubkey`, token accounts, transfers).  
No lifetimes or `&mut` today — we stay in a functional, immutable style.

---

## Overview

- Move semantics by default for owning types
- `Copy` for plain data, `Clone` for explicit deep copies
- Partial move and destructuring of structs
- `Drop` (RAII) and scope end
- Clean, side‑effect‑free functions (formatting outside the core logic)

**Domain context (minimal models):**
- `Pubkey([u8; 32])` — a simple wrapper over a 32‑byte public key
- `TokenAccount { owner: Pubkey, mint: Pubkey, amount: u64 }`
- `Transfer { from: Pubkey, to: Pubkey, mint: Pubkey, amount: u64 }`

---

## Learning Goals

Learn to:
- Decide when values **move**, when they **copy**, and when to **clone** explicitly
- Perform **partial moves** from structs and rebuild values without unnecessary clones
- Keep core logic **pure** (no printing or mutation)
- Understand when and in which order values are **dropped**

---

## Day Plan

1. **Theory checklist** — move/Copy/Clone/Drop, partial move patterns.
2. **Micro‑exercises** — short tasks demonstrating each concept.
3. **Mini‑challenge** — `can_transfer` as a pure gate function.
4. **Super‑task (skeleton)** — a functional transfer engine that consumes accounts and returns new ones.
5. **Wrap‑up** — coverage matrix, senior checklist, decision log.

---

## Deliverables

- `day4/README.md` (this file)
- Signatures only (no implementations) for a tiny DeFi transfer engine
- Pure functions; printing lives in `main` or dedicated formatters

> Intentional constraints for Day 4: **no** `Option/Result` (arrive later), **no** loops/iterators today, **no** `&mut`.

---

## Micro‑Exercises (no solutions)

1. **Move vs Copy**  
   Show that `u64` (and other simple scalars) are `Copy` while `String` moves.  
   After a move, the original binding is no longer usable.

2. **`Pubkey` as `Copy`/`Clone`**  
   Wrap `[u8; 32]` in `Pubkey` and derive `Copy, Clone`.  
   Pass a `Pubkey` into two functions; verify the original remains usable.

3. **Partial move from `TokenAccount`**  
   Destructure a `TokenAccount`, move out specific fields, and use the rest without cloning.  
   Rebuild a new `TokenAccount` with minimal copies.

4. **`Clone` when justified**  
   Add a non‑`Copy` field (e.g., a `String` label) in a toy struct and demonstrate an explicit, necessary `clone()` vs refactoring to avoid it.

5. **Observe `Drop`**  
   Create a small type with `Drop` (or use scoped logging) and show the drop order when multiple moves occur.

---

## Mini‑Challenge

Create the core check as a pure function (signatures only):

```rust
// models.rs — minimal domain types (signatures only in README)
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Pubkey(pub [u8; 32]);

pub struct TokenAccount {
    pub owner: Pubkey,
    pub mint: Pubkey,
    pub amount: u64,
}

// engine.rs — micro core (signatures only)
pub fn can_transfer(from: &TokenAccount, to: &TokenAccount, amount: u64, mint: Pubkey) -> bool {
    /* TODO: read-only validation; no printing; returns bool */
}
// No Option/Result today — the caller is responsible for checking and calling the transfer.
```

**Expected considerations:** exact mint equality, `from.amount >= amount`, and a business guard such as `from.owner != to.owner`.

---

## Super‑Task (skeleton only)

Build a **pure transfer engine** that **consumes** input accounts and returns **new** accounts along with a `Transfer` record.

**Structure**
```
src/
  main.rs
  defi/
    models.rs   // Pubkey, TokenAccount, Transfer
    engine.rs   // can_transfer, transfer_consume, account_summary
```

**Signatures (no bodies here)**
```rust
// defi/models.rs
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Pubkey(pub [u8; 32]);

pub struct TokenAccount {
    pub owner: Pubkey,
    pub mint: Pubkey,
    pub amount: u64,
}

pub struct Transfer {
    pub from: Pubkey,
    pub to: Pubkey,
    pub mint: Pubkey,
    pub amount: u64,
}

// defi/engine.rs
use super::models::{Pubkey, TokenAccount, Transfer};

pub fn can_transfer(from: &TokenAccount, to: &TokenAccount, amount: u64, mint: Pubkey) -> bool {
    /* TODO: read-only checks, return bool */
}

pub fn transfer_consume(
    from:   TokenAccount,
    to:     TokenAccount,
    amount: u64,
    mint:   Pubkey,
) -> (TokenAccount, TokenAccount, Transfer) {
    /* TODO: pure function — take ownership, return new accounts; no printing */
}

pub fn account_summary(acc: &TokenAccount) -> String {
    /* TODO: formatting only (use format!), keep the engine pure */
}

#[cfg(test)]
mod tests {
    use super::*;
    // At least 3 tests: 1 happy + 2 edge:
    // - exact amount
    // - amount == 0
    // - wrong mint
    // - identical owner
}
```

**Acceptance (baseline)**
- `can_transfer` captures business rules (no `Result` today)
- `transfer_consume` takes ownership and returns **new** accounts (demonstrates move)
- No printing inside the engine; formatting isolated in `account_summary` or `main`
- Tests present (≥ 3 total)

**Stretch**
- Use partial move while rebuilding accounts (minimize copies)
- Add `Lamports(u64)` as a transparent newtype (domain clarity)
- Provide a small `Drop` demo in a dedicated test or helper (not in prod logic)

---

## Coverage Matrix

```
Topic                      Where                                 Check
-----------------------------------------------------------------------------------------
Move by default            transfer_consume(from, to)            original bindings unusable after move
Copy                       Pubkey, u64                           no unnecessary clones
Clone (explicit)           rare fields / demo                    clone() used intentionally
Partial move               account rebuild                        no extra clones; destructuring/replace
Drop                       scoped demo / test                    clear drop order understanding
Pure functions             can_transfer, account_summary          no prints, no mutation
Tests                      engine::tests                          ≥ 3 (happy + edges)
```

---

## Guidelines

- Prefer moves for owned data; reserve `Clone` for justified duplication
- Keep the core engine pure (formatting outside)
- Derive `Copy` for small, plain data (e.g., `[u8; 32]` wrappers)
- Use destructuring and `std::mem` tools to avoid unnecessary clones

---

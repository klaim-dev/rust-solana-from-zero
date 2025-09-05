# Day 5 — Borrowing & References: `&T` / `&mut T`, Borrow Scopes, Aliasing, Reborrow, Slices `&[T]` / `&mut [T]`

Public learning **skeleton** for Day 5 (no solutions).  
Solana/Web3 context: token accounts and transfers.  
No manual lifetime annotations today and **no** `Option/Result` (arrive on Days 6–7). Methods (`impl`) come on Day 9.

---

## Overview

- Shared vs exclusive references: `&T` and `&mut T`
- Borrow **scope** (from the borrow to the last use) and how to shorten it
- Aliasing rules: one `&mut` **or** many `&`, but not both at the same time
- Reborrow: `&*r`, `&mut *r` to pass a reference onward temporarily
- Slices as borrows: `&[T]`, `&mut [T]` and element borrowing
- Splitting a mutable slice into two disjoint `&mut` via `split_at_mut`
- (Mention) Autoref/Autoderef; full method ergonomics later

---

## Learning Goals

Learn to:
- Mutate in place with `&mut` while preserving borrow rules
- Reason about and **shrink borrow scopes** using blocks and shadowing
- Reborrow safely to delegate work without taking ownership
- Use `&mut [T]` and `split_at_mut` to obtain two non‑overlapping mutable borrows
- Keep core logic pure (no printing) and side‑effect discipline explicit

---

## Day Plan

1. **Theory checklist** — reference & borrow rules; scope; aliasing.
2. **Micro‑exercises** — short tasks that surface common borrow errors.
3. **Mini‑challenge** — a small in‑place transfer core with signatures.
4. **Super‑task (skeleton)** — in‑place transfer engine (`&mut`‑based).
5. **Wrap‑up** — coverage matrix, senior checklist, decision log.

---

## Deliverables

- `day5/README.md` (this file)
- Signatures only (no implementations) for in‑place transfer operations
- Tests suggested (but not included here) — keep printing in `main`

> Constraints for Day 5: **no** `Option/Result`, **no** manual lifetime annotations, **no** ownership‑moving APIs here (that was Day 4).

---

## Micro‑Exercises (no solutions)

1. **One `&mut` vs many `&`**  
   Demonstrate that holding `&mut` and `&` to the **same** value at once does not compile; fix by **narrowing scope** with a block `{ ... }`.

2. **Reborrow**  
   A function takes `&mut TokenAccount`; reborrow and pass `&mut` into a helper without losing overall control of the original borrow.

3. **Borrow on a field**  
   Take `let bal = &mut acc.amount; *bal += 1;` and explain why reads like `acc.owner` are forbidden while `bal` is alive.

4. **Mutable slices**  
   `let xs: &mut [u64] = &mut arr;` — mutate elements; explain why two overlapping `&mut` to the same slice region are rejected.

5. **`split_at_mut`**  
   From `&mut [TokenAccount]`, obtain two **disjoint** `&mut` (for indices `i` and `j`) and explain the `i != j` requirement.

---

## Mini‑Challenge

Create signatures for an in‑place transfer core (no printing, no ownership moves):

```rust
// defi/inplace.rs — signatures only
use super::models::{Pubkey, Lamports, TokenAccount};

pub fn can_transfer(
    from: &TokenAccount,
    to: &TokenAccount,
    amount: Lamports,
    mint: Pubkey,
) -> bool {
    /* TODO: read‑only checks; return bool */
}

pub fn apply_transfer_inplace(
    from: &mut TokenAccount,
    to: &mut TokenAccount,
    amount: Lamports,
    mint: Pubkey,
) {
    /* TODO: assume can_transfer == true; mutate only; no printing */
}

// Stretch
pub fn apply_transfer_by_index(
    accounts: &mut [TokenAccount],
    i: usize,
    j: usize,
    amount: Lamports,
    mint: Pubkey,
) {
    /* TODO: split_at_mut to obtain two disjoint &mut; forbid i == j */
}
```

**Criteria:** pure mutation via references, no printing, `i != j` for the stretch API.

---

## Super‑Task (skeleton only)

Build an **in‑place token transfer engine** (no ownership transfer).

**Structure**
```
src/
  main.rs
  defi/
    models.rs          // from Day 4: Pubkey(Copy), Lamports(Copy), TokenAccount
    inplace.rs         // can_transfer / apply_transfer_inplace / (stretch) apply_transfer_by_index
```

**Acceptance (baseline)**
- `can_transfer` validates invariants: same mint, different owners, sufficient balance, `amount > 0`
- `apply_transfer_inplace` decreases `from` and increases `to` amounts correctly
- No printing inside the engine; formatting happens in `main` or a summary helper
- **Tests recommended (≥ 4):** happy path; `amount == 0`; wrong `mint` (do not call apply); `from == to` (forbidden)
- Borrowing rules respected — no overlapping `&mut` to the same account

**Stretch**
- `apply_transfer_by_index` using `split_at_mut` for two disjoint `&mut`
- `debug_assert!` contracts inside `apply_*` mirroring invariants (still no `Result`)

---

## Coverage Matrix

```
Topic                               Where                                  Check
-----------------------------------------------------------------------------------------------
& / &mut, borrow scope              inplace::apply_transfer_inplace        compiles; no aliasing
One &mut or many &                  micro #1                               error → fixed by scoping
Field borrow                        micro #3                               denied concurrent access
Mutable slices                      micro #4                               element updates
split_at_mut (disjoint)             inplace::apply_transfer_by_index        i != j; no panic
Reborrow                            micro #2                               forward &mut safely
Purity / no printing                inplace.rs                              all logic is pure
Tests                               module tests (suggested)                ≥ 4 happy + edges
```

---

## Guidelines

- Prefer **narrow** borrow scopes; close borrows early with blocks or shadowing
- Keep core logic pure; move formatting outside
- Use `split_at_mut` to obtain two disjoint mutable borrows from one slice
- Avoid mixed `&` + `&mut` to the same value within the same scope

---



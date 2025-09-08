# Day 8 — Consolidation (Days 4–7) + Refactor to `?` and a Clean Error Flow

Public learning **skeleton** for Day 8 (no solutions).  
Goal: refactor the DeFi engine (Days 4–7) into a **single, clean flow** of data and errors.

---


## Overview

Refactor toward one clear pipeline:

- **Single source of truth** for obtaining two disjoint `&mut` by indices.
- **Centralized invariants**: no bypasses; validation lives in one place.
- All public entrypoints return **`Result`** and use **`?`**, `ok_or`, `map_err`.
- **Money**: only `checked_add` / `checked_sub` (no `saturating_*` for crediting).
- **Normalized error messaging** (user/tech) with stable wording.
- **No** `unwrap`/`expect`/`panic` in production code.

---

## Acceptance (Baseline)

- `service::get_two_mut_checked` is the **only** way to obtain a pair of `&mut` by indices; every call site uses it.
- `inplace_checked::apply_transfer_inplace_checked` **always** calls `validate_transfer(...)` internally.
- `service::simulate_transfer` is **provably non‑mutating** (include an invariant test).
- All money ops use `checked_add/sub` with mapping to domain errors.
- Tests: ≥ 1 happy + ≥ 6 negative (ZeroAmount, SameOwner, MintMismatch, InsufficientFunds, IndexOutOfBounds / SameIndex, OverflowToBalance).
- Production path contains **no** `unwrap` / `expect` / `panic`.

**Stretch**

- Tiny `ensure(cond, err)` helper for readability.
- Summary metrics in `day08_diff.md` (count of `?`, duplicates removed, LoC reduction).

---

## Day Plan

1. **MVT (quick, 30–40 min)** — refresh only; no new theory.
2. **Micro‑exercises (45 min)** — target real hotspots from your codebase.
3. **Mini‑challenge (30–45 min)** — audit errors & messages.
4. **Super‑task (90 min)** — refactor “onto `?`”, align data/error flow, produce artifacts.

---

## MVT — Quick Refresh (hands‑on)

1. **Move/Copy/Clone (D4)** — where `Copy` is required (`Pubkey`, `Lamports`), where **not** to replace logic with `Clone`.
2. **`&` / `&mut` (D5)** — borrow scopes; `split_at_mut` ⇒ two disjoint `&mut`.
3. **Option/Result (D6)** — `ok_or`, `?`, `checked_*` ⇒ domain errors; remove `unwrap`.
4. **Error messages (D7)** — `thiserror` + `Display`, stabilized user/tech texts.

> Format: tiny case → anti‑example → minimal fix → takeaway.

---

## Micro‑Exercises (no solutions)

1. **Index logic split** — find all places creating `&mut` pairs **without** the shared helper; replace with `service::get_two_mut_checked`.
2. **Direct in‑place calls** — ensure all paths go through `apply_transfer_inplace_checked` (and thus `validate_transfer?`).
3. **One style of early return** — in public APIs, normalize to `?` or `return Err(..);` (choose one consistent style).
4. **Simulation purity** — add a test that `simulate_transfer` does **not** change balances.
5. **Error generator** — review all `checked_add/sub`; ensure `None →` `OverflowToBalance` / `InsufficientFunds` (never `saturating_*`).

---

## Mini‑Challenge — Error & Message Audit

- List **every** `TransferError` and `ParseError` variant; verify they are all covered in `error_format` with an **exhaustive** `match` (no `_` wildcard) **and** have tests.
- Stabilize user/tech message wording.
- Append an “error dictionary” to the end of `day08_diff.md` with the final phrasing.

---

## Super‑Task — Refactor “onto `?`” and Align the Flow

> Produce patch‑point changes only (no full solutions). Keep implementations in your codebase.

**Patch plan**

- **Indices** — keep a **single** helper:  
  `service::get_two_mut_checked(&mut [TokenAccount], i, j) -> Result<(&mut TokenAccount, &mut TokenAccount), TransferError>`  
  Replace all direct `split_at_mut` or manual branching with this call.

- **Central validation** — inside  
  `inplace_checked::apply_transfer_inplace_checked(...) -> Result<(), TransferError>`  
  call `validate_transfer(...)?` **first**, so invariants can’t be bypassed.

- **Simulation** — guarantee no mutation (compute with `checked_*` on locals only). Add an invariant test.

- **Error flow** — in public APIs: collapse pyramids into `?`; `Option→Result` via `ok_or`; external errors via `map_err`.

- **Kill duplicates** — remove any alternative helpers (e.g., a second `get_two_mut` in another module).

**Artifacts**

- `day08_refactor_before/` — pre‑patch snapshot.
- `day08_refactor_after/` — post‑patch snapshot.
- `day08_diff.md` — list of key diffs (what & why).

---

## Coverage Matrix

```
Topic                          Where                               Check
---------------------------------------------------------------------------------------------
Move/Copy/Clone                models + clone audit                no unnecessary `Clone`
& / &mut / split_at_mut        service::get_two_mut_checked        single helper used globally
Option/Result, `?`             all service facades                 no pyramids; consistent early returns
ok_or / map_err                indices, parsing                    Option→Result; external→domain errors
checked_* (money)              simulate/apply                      overflow / insufficient mapped
Error normalization            error_format + tests                exhaustive match, no `_`
No unwrap/expect/panic         production code                     grep + code review
Simulation non-mutating        service::simulate_transfer          invariant test passes
Diff “before/after”            day08_diff.md                       key changes explained
```

---

## Senior Checklist

- ❌ No `unwrap` / `expect` / `panic` in production.
- ✅ Single path to obtain `&mut` pairs by indices.
- ✅ Validation always precedes mutation.
- ✅ Money ops use `checked_*` only.
- ✅ ≥ 7 tests (1 happy + 6 negative), plus a “simulation does not mutate” test.
- ✅ `day08_diff.md` explains rationale and benefits (readability/reliability).

---

## Decision Log (8–10 lines, template)

1. Removed duplicate index logic → `get_two_mut_checked` is the single source of truth.
2. Centralized validation in in‑place operations → invariants cannot be bypassed.
3. Replaced nested conditionals with `?`; `Option→Result` via `ok_or`.
4. Money arithmetic moved to `checked_*`; overflow paths tested.
5. Simulation made side‑effect free (added invariant test).
6. Error messages stabilized (user & tech).
7. Purged `unwrap/expect/panic` from the production path.
8. Documented key patches & wins in `day08_diff.md` (fewer duplicates, simpler flow).

---

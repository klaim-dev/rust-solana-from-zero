
# 📅 Day 20 — Derive Mastery: Types That Work in Systems

**Theme:** `#[derive]` as a *system contract*, not syntactic sugar
**Focus:** `Debug, Clone, Copy, Eq, Hash, Default, PartialEq/Ord`
**Artifact:** `inventory_index` — domain index with secondary keys and deterministic sorting

---

## 🎯 Goal of the Day

Day 20 is the transition from *“structs that compile”* to **types that are operational in real systems**.

The goal was to build a small but realistic domain module where types:

* can be used as **HashMap keys**
* are **sortable by business rules**
* are **cheap to copy when appropriate**
* enforce **domain invariants at construction**
* have **predictable equality, hashing, and ordering**
* integrate cleanly with tests, logs, and indexes

This day is about understanding that `#[derive]` is not convenience —
it is a **semantic commitment**.

---

## 🧱 What Was Built

### Mini-project: `inventory_index`

A small in-memory index with:

* **Primary index:** `ItemId → Item`
* **Secondary index:** `Sku → ItemId`
* **Deterministic sorting** by domain rules
* **Strict invariants** (no duplicate IDs or SKUs)
* **Clean error contracts**

This mirrors real backend components:

* repositories
* caches
* read models
* in-memory projections

---

## 🗂️ Project Structure

```text
src/
  lib.rs
  index.rs              # InventoryIndex + insertion / lookup / sorting
  domain/
    mod.rs
    types.rs            # ItemId, Sku, Item, SortSpec
    sort.rs             # SortKey / NameKey (manual Ord)
    error.rs            # Domain errors (SkuErr, ItemErr)
tests/
  index.rs              # Integration tests (12+)
```

Clear separation:

* **domain/** — pure domain logic (no HashMap state)
* **index.rs** — storage + invariants
* **tests/** — black-box verification

---

## 🧠 Core Design Decisions

### 1. `ItemId` as a newtype

```rust
pub struct ItemId(u64);
```

**Derives:** `Copy, Eq, Hash, Ord`

Why:

* safe HashMap key
* cheap to copy
* stable tie-breaker in sorting
* future-proof (can become UUID/NonZero/etc.)

Creation via:

```rust
ItemId::new(u64)
```

→ preserves invariants and hides representation.

---

### 2. `Sku` enforces invariants at the boundary

```rust
Sku::try_new(&str) -> Result<Sku, SkuErr>
```

Rules:

* trimmed
* lowercased
* empty rejected

**Derives:** `Clone, Eq, Hash`
**Not `Copy`** — owns heap memory (`String`).

This guarantees:

> *If you have a `Sku`, it is always valid.*

---

### 3. Status-only domain errors

```rust
enum SkuErr { InvalidSku }
enum ItemErr { InvalidName }
```

Why:

* no allocations in error paths
* stable, testable error contracts
* ready for HTTP mapping later

Errors describe **what went wrong**, not user input.

---

### 4. Secondary index via IDs, not duplicated entities

```text
by_id  : ItemId → Item
by_sku : Sku    → ItemId
```

Why:

* single source of truth
* no data duplication
* no risk of divergence
* cheap lookups

This is the canonical **secondary index pattern**.

---

### 5. Sorting via explicit keys (manual `Ord`)

Two internal keys:

#### `SortKey<'a>`

```text
price DESC → name ASC → id ASC
```

#### `NameKey<'a>`

```text
name ASC → id ASC
```

Why **manual `Ord`**:

* `derive(Ord)` only follows field order
* business ordering ≠ structural ordering
* explicit intent > implicit behavior

Keys are:

* `pub(crate)` (not public API)
* cheap (`&str` + `ItemId`)
* deterministic

Sorting uses:

```rust
sort_by_cached_key(...)
```

→ keys computed once per element.

---

## 🧪 Test Coverage

12 integration tests validate all contracts.

### Derive & Type Contracts

* `item_id_hashmap_key_works`
* `sku_normalizes_and_compares`

### Index Behavior

* `insert_and_get_by_id`
* `get_by_sku_finds_inserted_item`
* `get_by_sku_invalid_returns_none`

### Sorting

* `list_sorted_price_desc_name_asc_id_asc`
* `list_sorted_name_asc_id_asc`
* `list_sorted_on_empty_index_returns_empty`

### Negative Cases (≥3)

* `duplicate_id_is_error`
* `duplicate_sku_is_error`
* `sku_try_new_rejects_empty`
* `item_try_new_rejects_empty_name`

---

## 📊 Coverage Matrix

| Subtopic               | Location                    | Verified by Test    |
| ---------------------- | --------------------------- | ------------------- |
| Debug / observability  | all domain types            | failing test output |
| Clone vs Copy          | ItemId vs Sku               | compile + tests     |
| Eq + Hash              | ItemId, Sku                 | HashMap tests       |
| Invariants             | Sku::try_new, Item::try_new | negative tests      |
| Manual Ord             | domain/sort.rs              | sorting tests       |
| Deterministic ordering | list_sorted                 | sorting tests       |

---

## ✅ Senior DoD Checklist (Day 20)

* ❌ no `unwrap/expect` in prod code
* ✅ derives chosen intentionally
* ✅ manual `Ord` where business logic applies
* ✅ secondary index via IDs
* ✅ ≥3 negative tests
* ✅ deterministic sorting
* ✅ no representation leaks
* ✅ clean module boundaries

---

## 📝 Decision Log

1. Used `ItemId` as a `Copy` newtype to enable safe keys and tie-breakers.
2. Kept `Sku` non-`Copy` due to heap ownership.
3. Enforced all domain invariants at construction.
4. Chose status-only errors to keep contracts stable.
5. Stored `Sku → ItemId`, not `Sku → Item`, to avoid duplication.
6. Implemented manual `Ord` to reflect business ordering.
7. Used `sort_by_cached_key` to avoid recomputation.
8. Kept sorting keys internal (`pub(crate)`).
9. Returned `Option` from lookup APIs for ergonomic callers.
10. Validated behavior exclusively via black-box tests.

---

## 🔚 Outcome

By the end of Day 20, the system contains **types that are safe, comparable, hashable, sortable, testable, and production-ready**.

This is the foundation required for:

* repositories
* caches
* query engines
* API layers
* real backend services


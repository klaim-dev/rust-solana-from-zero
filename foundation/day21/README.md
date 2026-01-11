# 📅 Day 21 — Filesystem & Buffering

**Atomic persistence for a pure Rust domain**

---

## 🎯 Goal of the Day

Day 21 is the **first real I/O day** in the Rust Foundation.

The goal was to take an existing **pure domain** (`InventoryIndex`) and add a **safe persistence layer** on top of it, while strictly following production-grade rules:

* no domain ↔ filesystem coupling
* buffered I/O (read & write)
* atomic file writes (no corrupted state)
* readable, contextual errors
* deterministic output
* no `unwrap` / `expect` in production code

This day models how **real systems persist state safely** without leaking infrastructure concerns into business logic.

---

## 🧱 Architecture Overview

The crate is intentionally split into **three clean layers**:

```
domain/        → pure business logic
persist/format → string ↔ domain mapping (no I/O)
persist/fs     → filesystem I/O only
```

### Key rule

> **The domain must not know where data comes from.
> The filesystem must not know what the data means.**

---

## 🧠 What Was Built

### 1. Domain (existing from Day 20)

* `Item`, `ItemId`, `Sku`
* `InventoryIndex`

  * enforces invariants:

    * unique `ItemId`
    * unique `Sku`
    * consistent secondary index
* no filesystem, no strings, no I/O

---

### 2. Persistence Core (string-based)

Located in `persist/format` and `persist/core`.

#### Serialization

```rust
pub fn serialize(idx: &InventoryIndex) -> String
```

* deterministic output
* items sorted by `ItemId`
* stable format:

  ```
  id=1 sku=abc-123 name=Widget price=500
  ```

Why no `Result`?

* serialization operates only on **already valid domain data**
* any failure here would be a **bug**, not a runtime error

---

#### Deserialization

```rust
pub fn deserialize(text: &str) -> Result<InventoryIndex, PersistError>
```

Handles:

* empty lines
* comment lines (`# ...`)
* strict parsing of records
* full error context:

  * line number (1-based)
  * original line
  * root cause (`ParseLineError` or `IndexError`)

All domain invariants are enforced **during insert**, not parsing.

---

### 3. Filesystem Layer (I/O only)

Located in `persist/fs`.

#### Load

```rust
pub fn load_from_file(path: &Path) -> Result<InventoryIndex, PersistError>
```

Implementation details:

* `File::open`
* `BufReader + lines()`
* file read into a `String`
* parsing delegated to `deserialize`

Why read the whole file first?

* keeps filesystem logic thin
* all line numbering & validation lives in one place (`deserialize`)

---

#### Save (Atomic!)

```rust
pub fn save_to_file(path: &Path, idx: &InventoryIndex) -> Result<(), PersistError>
```

**Atomic write pattern:**

1. create temp file in same directory
2. write via `BufWriter`
3. flush buffer
4. `sync_all` to disk
5. `rename(temp → target)`

Guarantee:

> If the process crashes at any point,
> the original file is never corrupted.

---

## ⚠️ Error Model

All errors are unified under a single type:

```rust
PersistError
```

Variants:

* `Io` — filesystem errors
* `InvalidLine` — parse failure with line number + content
* `Insert` — domain/index violations
* `InvalidPath` — invalid save target

Errors are:

* human-readable
* structured
* chain original causes via `#[source]`

---

## 🧪 Tests

### Covered scenarios

* roundtrip: `save → load`
* deterministic output
* atomic overwrite (old file replaced safely)
* invalid lines report correct line numbers
* duplicate `id` / `sku` errors
* empty index persistence

Tests use temporary files/directories and never touch real filesystem state.

---

## 📊 Coverage Matrix

| Subtopic                | Location                     | Verified By           |
| ----------------------- | ---------------------------- | --------------------- |
| Buffered reading        | `persist/fs::load_from_file` | FS roundtrip tests    |
| Buffered writing        | `persist/fs::save_to_file`   | FS tests              |
| Atomic save             | temp + rename                | atomic overwrite test |
| Deterministic output    | `serialize` (sort by ItemId) | serialization tests   |
| Parse errors w/ context | `PersistError::InvalidLine`  | invalid line tests    |
| Domain purity           | `domain/*`                   | code structure review |

---

## ✅ Senior DoD Checklist (Day 21)

* ❌ no `unwrap` / `expect` in prod paths
* ✅ buffered I/O everywhere
* ✅ atomic persistence
* ✅ readable, contextual errors
* ✅ ≥3 negative tests
* ✅ domain isolated from FS
* ✅ deterministic output

---

## 🧠 Design Decisions

* **Line-based text format**
  Chosen for transparency and debuggability. Serde comes later.

* **Whole-file deserialize**
  Keeps error handling and line numbering centralized.

* **Atomic writes by rename**
  Industry-standard pattern used by databases and package managers.

* **Sorting by ItemId**
  Persistence order is not a presentation concern.

---

## 🔮 What Would Be Added in v2

* quoted fields (names with spaces)
* schema versioning
* migration support
* file locking for multi-process safety
* binary format / serde support

---

## 🏁 Result

Day 21 delivers a **production-quality persistence layer**:

* safe
* deterministic
* debuggable
* architecturally clean

This is exactly how real Rust services persist domain state without sacrificing correctness or design integrity.

---
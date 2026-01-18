# Orders CLI — Domain-Driven Rust Project (Day 25)

A production-style CLI application for managing orders and items, built in Rust with a **clean layered architecture**, **strong domain invariants**, and **safe persistence**.

This project is part of a structured Rust learning journey focused on **real backend engineering**, not toy examples.

---

## ✨ Features

* Create and manage orders
* Add / remove items
* Query orders (by id, by customer)
* Calculate total quantity per order
* File-based persistence with **atomic saves**
* Deterministic text format (human-readable)
* Robust CLI with strict validation
* Full test coverage (unit, integration, e2e)

---

## 🧱 Architecture Overview

The project follows a **strict layered architecture**:

```
CLI → App → Domain → Persist
```

### Layers

#### 1. Domain

Pure business logic and invariants:

* `Order`, `OrderStatus`
* Strongly typed identifiers (`OrderId`, `Sku`, `Qty`)
* `Store` as an in-memory aggregate root
* No IO, no parsing, no CLI concerns

**Key properties:**

* All invariants enforced at construction
* No `unwrap` in business paths
* Errors are explicit and typed

---

#### 2. Persist

Responsible for storage and serialization:

* Custom line-based text format
* Manual parser (no serde)
* Atomic file writes (`.tmp` + rename)
* Deterministic serialization order

**Why manual parsing?**

* Full control over error reporting
* Precise validation
* No hidden behavior

---

#### 3. App

Application service layer:

* Coordinates domain + persistence
* Defines `AppCommand`
* Returns `AppOutput` (render-ready)
* No CLI parsing, no file paths

---

#### 4. CLI

Command-line interface:

* Manual argument parsing
* Strict flag validation
* Helpful error messages
* No business logic

---

## 📄 Storage Format

Orders are stored in a plain text file:

```
ORDER id=1 customer="alice" status=draft
ITEM order_id=1 sku=sku1 qty=2
ITEM order_id=1 sku=sku2 qty=1
```

### Properties

* Human-readable
* Deterministic ordering
* Safe to edit manually
* Forward-compatible

---

## 🚀 CLI Usage

```
orders <command> [options]
```

### Commands

```
add-order   --id <id> --customer <name>
add-item    --id <id> --sku <sku> --qty <qty>
remove-item --id <id> --sku <sku>
show        --id <id>
list        [--customer <name>]
total       --id <id>
```

### Options

```
--file <path>   override storage file
-h, --help      show help
```

### Examples

```bash
orders add-order --id 1 --customer alice
orders add-item --id 1 --sku sku1 --qty 2
orders show --id 1
orders list
orders total --id 1
```

---

## 🛡️ Validation & Safety

### Domain guarantees

* `OrderId` ≠ 0
* `Sku` must be non-empty, trimmed, no whitespace
* `Qty` > 0
* No duplicate order IDs
* No items in cancelled orders

### Persistence guarantees

* Atomic writes (no partial files)
* Strict parsing with line numbers
* No silent data loss

### CLI guarantees

* Unknown flags rejected
* Invalid combinations rejected
* Clear error messages
* Help always available

---

## 🧪 Testing Strategy

### Test layers

* **Domain tests** — invariants and edge cases
* **Persist tests** — parsing, serialization, round-trip
* **App tests** — command execution, errors
* **CLI tests** — argument parsing
* **E2E tests** — full file-based workflow

All tests are green:

```bash
cargo test
```

---

## 🧠 Design Decisions

* No global state
* No hidden IO
* No macros for parsing
* No `unwrap` in production paths
* Errors are part of the API
* Text format over JSON (intentional)

---

## 🎯 Learning Goals of This Day

* Practice **domain-driven design** in Rust
* Build a real CLI without frameworks
* Understand ownership across layers
* Write safe persistence code
* Design for extensibility, not hacks

---

## 📦 Project Status

✅ Feature-complete
✅ Fully tested
✅ Ready for extension

Possible next steps:

* Versioned storage format
* Order confirmation workflow
* Pricing logic
* Async backend port (Axum)

---

## 🧭 About This Project

This project is part of a **30-day Rust foundation** focused on becoming a **production-ready backend engineer**, not just learning syntax.

Every day produces a **real artifact**, not throwaway code.

---

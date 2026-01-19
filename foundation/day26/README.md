# 🧱 Day 26 — Refactor & Cleanliness

## Orders v1 as a Production-Ready CLI Template (Rust Foundation)

This repository contains **Orders v1**, a file-based CLI application written in Rust and refined on **Day 26** of the *Rust Foundation* track.

Day 26 is not about new features.
It is about **turning working code into a reusable, clean, and maintainable template**.

> **Theme of the day:**
> *Refactor, audit, and formalize quality — before moving forward.*

---

## 🎯 Day 26 Goal

Transform `orders_v1` (Day 25 capstone) into:

* a **clean architectural template**
* with **explicit contracts**
* **deterministic behavior**
* **stable errors**
* and **zero accidental complexity**

This project is intended to be **copied forward** into:

* backend services (Axum / SQLx),
* internal CLI tools,
* Solana indexers & admin utilities,
* long-living infrastructure code.

---

## 📦 What This Project Is (and Is Not)

### ✅ It **is**

* a production-quality CLI example
* an architecture reference
* a persistence & error-handling template
* a testable, deterministic system

### ❌ It is **not**

* a toy demo
* a feature playground
* a framework experiment

The value is in **discipline**, not size.

---

## 🗂️ Project Structure

```
orders_v1/
├─ src/
│  ├─ domain/        # Pure business logic (no IO, no CLI)
│  ├─ app/           # Orchestration layer (load → apply → save)
│  ├─ persist/       # File format + atomic persistence
│  ├─ cli/           # Argument parsing & usage errors
│  └─ main.rs        # Thin entry point
│
├─ tests/
│  ├─ cli_args.rs
│  ├─ service_inmemory.rs
│  ├─ persist_roundtrip.rs
│  └─ e2e_file_repo.rs
│
├─ docs/
│  └─ cleanliness_checklist.md
│
├─ Cargo.toml
└─ README.md
```

---

## 🧠 Architectural Boundaries (Hard Rules)

### `domain`

* no filesystem
* no environment variables
* no CLI knowledge
* no printing
* **only business rules & invariants**

### `persist`

* handles file format and IO
* atomic writes (`tmp → flush → fsync → rename`)
* deterministic serialization
* no business decisions

### `app`

* orchestrates flow
* converts errors
* applies commands to domain
* no parsing, no direct IO

### `cli`

* parses arguments
* validates flags
* reports usage errors
* **zero domain logic**

### `main`

* parse args
* build service
* run command
* print result
* exit with correct code

---

## 📄 Data Format (Deterministic by Design)

The persistence format is **line-based and stable**:

```
ORDER id=1 customer="alice" status=draft
ITEM order_id=1 sku=sku1 qty=2
```

Guarantees:

* orders are sorted by `OrderId`
* items are sorted by `Sku`
* same input → same output

This enables:

* golden file tests
* stable diffs
* predictable long-term storage

---

## 🚀 CLI Usage

```bash
orders <command> [options]
```

### Commands

```text
add-order   --id <id> --customer <name>
add-item    --id <id> --sku <sku> --qty <qty>
remove-item --id <id> --sku <sku>
show        --id <id>
list        [--customer <name>]
total       --id <id>
```

### Options

```text
--file <path>    override storage file
-h, --help       show help
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

## ⚠️ Error & Exit Code Contract

### Exit Codes

| Code | Meaning                   |
| ---: | ------------------------- |
|    0 | success                   |
|    1 | runtime error (domain/IO) |
|    2 | usage / CLI error         |

### Error Layers

* `UsageError` — CLI misuse, flags, help
* `DomainError` — business rule violations
* `PersistError` — file / format / IO problems
* `AppError` — unified application error

All errors:

* are human-readable (`Display`)
* include context
* have stable wording (tests rely on this)

---

## 🧪 Testing Strategy

### Unit tests

* domain invariants
* CLI parsing & validation
* service logic (in-memory repo)

### Integration tests

* persistence round-trip
* deterministic serialization

### End-to-end tests

* real temp directories
* real filesystem IO
* full command execution path

Run all checks:

```bash
cargo fmt
cargo clippy -- -D warnings
cargo test
```

---

## 🧼 Day 26 Artifact — Cleanliness Checklist

📄 `docs/cleanliness_checklist.md`

This document defines:

* repository hygiene rules
* architectural boundaries
* error & exit contracts
* determinism requirements
* testing discipline

It is intentionally **boring and strict**.

> This checklist is the project’s **“dark matter”**:
> invisible, but it holds everything together.

---

## 📊 Coverage Matrix (Days 21–26)

| Day | Topic                          | Location        | Verified By        |
| --: | ------------------------------ | --------------- | ------------------ |
|  21 | File IO & atomic writes        | `persist/`      | e2e tests          |
|  22 | CLI parsing & usage errors     | `cli/`          | cli tests          |
|  23 | Error contracts & Display      | `app/error.rs`  | unit tests         |
|  24 | Mini-architecture patterns     | whole crate     | review             |
|  25 | Orders capstone implementation | all layers      | full test suite    |
|  26 | Refactor & cleanliness         | docs + refactor | checklist + clippy |

---

## 🧭 Why This Matters

This project trains:

* architectural thinking
* discipline before scale
* error contracts
* deterministic systems
* testable IO boundaries

These skills transfer **directly** to:

* backend services,
* blockchain tooling,
* long-term infrastructure code.

---

## ✅ Status (Day 26 Done)

* ✅ tests green
* ✅ clippy clean (`-D warnings`)
* ✅ no `unwrap` / `expect` in prod paths
* ✅ deterministic output
* ✅ checklist documented
* ✅ ready to reuse as template

---


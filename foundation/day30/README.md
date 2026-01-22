# 🦀 Rust Foundation — 30 Days (Production-style)

**Rust backend engineer documenting a structured journey**  
from language fundamentals → production-ready patterns → backend systems.

This repository is **not a tutorial dump**.  
It is a **curated, engineering-style foundation** focused on correctness, ownership, errors, tests, and clean architecture.

> Code > comfort.  
> Contracts > shortcuts.  
> No `unwrap` in production paths.

---

## 🎯 Purpose of this repository

This repo demonstrates that I can:

- reason about **ownership, borrowing, lifetimes**
- design **clear APIs** with `&str` / `String`, `Option` / `Result`
- build **CLI tools** with deterministic behavior
- handle **files, config, errors** in a production-style way
- write **tests**, including negative cases
- refactor code safely
- use **tooling** (fmt, clippy, benchmarks, CI basics)

Everything here is written as if it were going to be **maintained**, not just compiled once.

---

## 🧱 Structure

```text
foundation/
├── day01_basics/
├── day07_result/
├── day16_catalog/
├── day21_fs/
├── day22_cli/
├── day23_config/
├── day24_arch/
├── day25_orders/
├── day26_refactor/
├── day27_lifetimes/
├── day29_tooling/
└── README.md  ← you are here
````

Each folder is a **self-contained crate or artifact** with its own scope and contracts.

---

## ⭐ Curated projects (start here)

### 1️⃣ Orders CLI — Capstone (Days 25–26)

**What it is:**
A production-style CLI application for managing orders with persistence.

**Key topics:**

* domain modeling
* file persistence (atomic save)
* error contracts (`thiserror` + `Display`)
* CLI parsing and validation
* deterministic output
* refactor in PR-style (Day 26)

📁 `day25_orders/`
📁 `day26_refactor/`

Run:

```bash
cd day25_orders
cargo test
cargo run -- --help
```

---

### 2️⃣ Catalog with indexes (Day 16)

**What it is:**
In-memory catalog with **primary + secondary indexes**.

**Key topics:**

* `HashMap` ownership patterns
* avoiding N+1 logic
* clean CRUD API
* test coverage

📁 `day16_catalog/`

---

### 3️⃣ Config loader (Day 23)

**What it is:**
Fail-fast configuration loader with precedence:

```
ENV > TOML file > defaults
```

**Key topics:**

* config layering
* typed errors
* human-readable messages
* no silent fallback

📁 `day23_config/`

---

### 4️⃣ CLI parsing without magic (Day 22)

**What it is:**
Manual CLI argument parsing with explicit contracts.

**Key topics:**

* `std::env::args`
* usage errors vs domain errors
* exit codes
* deterministic behavior

📁 `day22_cli/`

---

### 5️⃣ Tooling & benchmarks (Day 29)

**What it is:**
A tooling-ready crate with formatting, linting, tests, and a micro-benchmark.

**Key topics:**

* `rustfmt`
* `clippy -D warnings`
* `criterion` benchmarks
* `Vec::with_capacity` performance
* CI draft

📁 `day29_tooling/`
📄 `bench_report.md`

---

## 🧠 Engineering standards used everywhere

These rules are enforced **by design**, not by accident:

* ❌ no `unwrap` / `expect` in production paths
* ✅ `Option` / `Result` model real states
* ✅ errors are **typed** and **human-readable**
* ✅ functions accept `&str`, return `String` only when owning data
* ✅ deterministic output formats
* ✅ tests include negative and edge cases
* ✅ clean boundaries (`domain / app / infra` where applicable)

---

## 🧪 Testing policy

* Early days: `assert_eq!` for fast feedback
* From Day 3 onward: **unit tests**
* Negative cases included:

  * empty input
  * invalid formats
  * boundary conditions
* Some days use `compile_fail` doctests to prove lifetime rules

Run tests:

```bash
cargo test
```

---

## 📐 Capability matrix (proof-based)

| Skill                 | Evidence     | Notes                        |
| --------------------- | ------------ | ---------------------------- |
| Ownership & borrowing | days 1–5     | no accidental clones         |
| Error modeling        | day07, day25 | `thiserror`, clean Display   |
| Collections & indexes | day16        | HashMap + secondary index    |
| CLI parsing           | day22        | no magic, explicit contracts |
| Config layering       | day23        | env > file > defaults        |
| Files & persistence   | day21, day25 | atomic save                  |
| Architecture          | day24        | enum + trait pipeline        |
| Refactoring           | day26, day28 | PR-style, behavior preserved |
| Lifetimes             | day27        | real traps + fixes           |
| Tooling & perf        | day29        | fmt / clippy / bench         |

---

## ▶️ How to run

Requirements:

* Rust stable (edition 2021)

Example:

```bash
cd day25_orders
cargo test
cargo run -- --help
```

Formatting & linting:

```bash
cargo fmt
cargo clippy -- -D warnings
```

Benchmarks:

```bash
cd day29_tooling
cargo bench
```

---

## 🚀 What’s next

This foundation feeds directly into:

### Backend on Rust (40 days)

* Axum
* SQLx + Postgres
* AppState / AppError
* tracing + OpenTelemetry
* migrations
* integration tests
* latency budgets
* CI/CD + Docker

After backend → **Advanced Rust + Solana / Web3**.

---

## 📌 Philosophy

This repo exists to answer one question honestly:

> “Can this person write Rust code that survives contact with production?”

No shortcuts.
No hype.
Just code that explains itself.

---

**Status:**
✅ Foundation complete
➡️ Moving to Backend stage



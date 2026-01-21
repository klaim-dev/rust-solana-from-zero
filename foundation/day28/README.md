# 📅 Day 28 — Refactor on Autopilot (PR-style)

> **Goal:** practice production-grade, behavior-preserving refactoring.
>  
> No new features. No new concepts.  
> Only clean code, stable contracts, tests, and clear review communication — exactly how real backend and Solana code evolves.

---

## 🎯 Goal of the Day

Take an existing, working crate (`orders_v1`) and perform a **mechanical refactor** that:

* improves readability and data flow
* reduces duplication and implicit logic
* clarifies layer boundaries
* keeps **all behavior unchanged**

This day is about training the **adult engineering loop**:

> read → detect smells → refactor safely → verify → document like a PR

---

## 📦 Project Context — `orders_v1`

`orders_v1` is a small but realistic CLI application with:

* CLI parsing
* domain model (orders)
* application service layer
* file-based persistence
* centralized error handling
* unit, smoke, and golden tests

Day 28 does **not** add functionality.  
It focuses purely on code quality and maintainability.

---

## 🧠 Refactor Mindset (Minimal Theory)

### Refactor is safe only with contracts

Before touching the code, the following invariants were fixed:

* CLI behavior remains the same
* persistence format is unchanged
* error messages keep the same meaning
* exit codes are stable:
  * `0` — help / successful commands
  * `1` — runtime / IO errors
  * `2` — usage errors

---

### Mechanical vs Semantic Refactor

**Allowed (mechanical):**

* renaming for intent
* extracting helper functions
* simplifying control flow
* removing duplication
* improving data flow

**Not allowed (semantic):**

* behavior changes
* new features
* new dependencies
* format changes

---

## 🔧 What Was Refactored

### 1️⃣ Data flow

The core flow was made explicit and linear:

```

load → apply → save → render

```

* reduced nesting
* early returns on errors
* clear orchestration inside `app::service`

---

### 2️⃣ Helper extraction

* CLI parsing split into small, intent-revealing helpers
* persistence logic separated into parse / serialize helpers
* orchestration logic centralized in one place

---

### 3️⃣ Errors and layer boundaries

Clear separation of responsibility:

* `DomainError` — business rules only
* `PersistError` — IO and format issues
* `AppError` — application-level coordination
* CLI handles only `CliError` and exit codes

Layer boundaries became easier to read and reason about.

---

## 🧪 Tests — Refactor Safety Net

Since behavior must not change, tests protect the refactor.

### Golden Test

Locks the persistence format:

* any accidental format change breaks the test
* refactoring is safe as long as the golden test passes

---

### Smoke Tests

* full pipeline: load → apply → save
* temporary files
* realistic scenarios

---

### Domain & CLI Tests

* negative domain cases
* CLI parsing without panics
* edge-case coverage

---

## 📂 Artifacts

* `src/` — refactored code, behavior unchanged
* `tests/`:
  * golden test
  * smoke tests
  * domain and CLI tests
* `docs/refactor_plan_day28.md`
* `docs/review_notes_day28.md`

---

## ✅ Definition of Done (Senior Level)

* ✅ behavior unchanged
* ✅ persistence format locked by golden test
* ✅ exit codes preserved
* ✅ no `unwrap` / `expect` in production paths
* ✅ `cargo fmt` clean
* ✅ `cargo clippy -- -D warnings` clean
* ✅ all tests green
* ✅ refactor documented PR-style

---

## 🧭 Why This Day Matters

Day 28 trains a skill used **every day** in real projects:

* safe refactors in live code
* confidence when touching existing systems
* clean PRs that reviewers can trust
* long-term maintainability

Without this habit, codebases slowly decay.  
With it, systems stay healthy and evolvable.

---

## 🧾 Decision Log (Summary)

* primary focus: readability and data flow
* behavior protected by tests
* refactor kept strictly mechanical
* larger structural changes intentionally postponed
* code is easier to read and safer to modify

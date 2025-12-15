# 📅 Day 15 — Control Flow Patterns & Absence Semantics

> **Theme:** Nested `match`, `if let`, `let … else`,  
> `Option<Result<T, E>>` vs `Result<Option<T>, E>`

---

## Purpose of the Day

Day 15 is not about syntax tricks.

It is about learning how to **model intent in types** and how to **control absence vs errors explicitly**, without `unwrap`, using Rust’s pattern-matching tools.

The main goal:

> **Make absence and failure impossible to confuse.**

---

## Core Concept

### Errors dominate absence

Rust types force us to answer an important question:

> Is the absence of a value an error — or a valid state?

This day focuses on encoding that answer directly in function signatures.

| Shape | Meaning |
|------|--------|
| `Option<T>` | Value may be absent, and that’s OK |
| `Result<T, E>` | Failure dominates; caller must handle it |
| `Option<Result<T, E>>` | Value may be missing; if present, it may fail |
| `Result<Option<T>, E>` | Failure dominates; absence is a valid outcome |

---

## What Was Built

### 1. `Kv` — key-value access layer (infra)

`Kv` is a thin adapter over raw key-value input (`HashMap<String, String>`).

Its responsibility is **only**:

- presence checks
- trimming
- parsing
- validation
- returning structured errors

`Kv` **does not decide defaults** and **does not know domain rules**.

It exposes a clean API:

- `required(key)` → `Result<&str, ConfigError>`
- `optional(key)` → `Option<&str>`
- typed helpers:
  - `required_u16`
  - `optional_u32`
  - `optional_bool`

This layer is reusable for:
- environment variables
- CLI arguments
- config files
- any key-value input source

---

### 2. Resolver (`build_config`) — decision layer

`build_config` is responsible for **business decisions**, not parsing.

It decides:
- which keys are required
- which keys are optional
- where defaults apply
- how validated values become domain types

`build_config` composes validated data into a domain-level `Config`.

---

### 3. Domain layer

The domain contains:
- `Config`
- `Mode`

Domain types:
- never see raw strings
- never parse input
- never deal with trimming or validation

They receive **only validated, typed values**.

---

## Required vs Optional Keys

The distinction is explicit and intentional.

| Key | Rule | Reason |
|----|-----|-------|
| `DB_URL` | required | Service cannot start without DB |
| `PORT` | optional (defaulted) | Safe default exists |
| `DEBUG` | optional | Feature flag |
| `MAX_CONNECTIONS` | optional | Performance tuning |
| `MODE` | optional | Defaults to `Dev` |

Defaults are applied **only** in the resolver layer.

---

## Error Semantics

All error cases are explicit and typed.

| Case | Result |
|----|------|
| Missing required key | `MissingKey` |
| Empty value | `EmptyValue` |
| Invalid integer | `InvalidInt` |
| Invalid boolean | `InvalidBool` |
| Invalid mode | `InvalidMode` |

There are **no `unwrap`s in production paths**.

---

## Control Flow Patterns Used

### `match` on tuples

Used to express multi-value state combinations clearly.

### `if let`

Used when only one successful pattern matters.

### `let … else`

Used as a **guard**, to:
- exit early
- avoid nesting
- make invalid states explicit

Mental model:

> “If this is invalid → return early.”

---

## Why `Kv` Exists

Without `Kv`, validation logic would be:
- duplicated
- scattered
- tightly coupled to config resolution

`Kv` provides:
- a single source of truth for raw input handling
- predictable error semantics
- easy unit testing

---

## Testing Strategy

Tests should focus on:

### `Kv`
- missing keys
- empty values
- invalid numbers
- invalid booleans
- trimming behavior

### Resolver
- correct defaults
- correct required/optional decisions
- correct domain construction

---

## Coverage Matrix

Covered scenarios include:

- happy path (all fields present)
- defaults only
- missing required key
- empty value
- invalid integer
- invalid boolean
- invalid mode

---

## Key Design Decisions

- Separate infra from decision logic
- Use `Result<Option<T>, E>` for optional typed values
- Avoid building values only for control flow
- No allocations unless entering domain layer
- No hidden defaults in infra

---

## Outcome

By the end of Day 15:

- Absence and errors are clearly distinguished
- Control flow is flat and readable
- Configuration parsing is predictable and testable
- The system is ready to accept input from any key-value source

This is **production-grade configuration handling**, not a toy example.

---

## Status

✅ Day 15 complete  
✅ No `unwrap` in production paths  
✅ Clear separation of concerns  
✅ Senior-level control flow patterns applied

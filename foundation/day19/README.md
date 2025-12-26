# 🧱 Day 19 — Traits & Bounds

### Contracts, Polymorphism, and Clean Module Interfaces (Rust Foundation)

> **Day 19 is where real architecture begins.**
> We stop writing “just working code” and start designing **contracts between modules**:
> how data behaves, how behavior is swapped, and where side effects live.

This day is a direct continuation of **Day 18 (FSM + enum)**.
If Day 18 was about **modeling states**,
Day 19 is about **modeling behavior correctly**.

---

## 🎯 Goal of the Day

Build a small, production-style **`audit_log`** module where:

* domain events are modeled as a strongly typed `enum`
* formatting is abstracted via `trait` (no `if format == ...`)
* filtering is defined via closures
* side effects (writing output) are isolated behind a `Sink`
* core logic remains **pure, testable, and reusable**

---

## 🧠 Core Concepts Covered

### 1. `trait` = Behavior Contract

A `trait` defines **what a type can do**, not **what it is**.

```rust
pub trait Formatter {
    fn format(&self, e: &AuditEvent) -> String;
}
```

The pipeline depends only on the **contract**, never on concrete implementations.

---

### 2. Bounds (`T: Trait`) = Requirements

Trait bounds describe the **minimum required behavior**, not arbitrary constraints.

```rust
pub fn emit_events<F, S, P>(...)
where
    F: Formatter,
    S: Sink,
    P: Fn(&AuditEvent) -> bool,
```

If a bound exists, it is **logically required** by the function.

---

### 3. Pure vs Effectful Code

* `render(...) -> Vec<String>` → **pure function**
* `emit_events(...)` → **side-effectful function**

This separation:

* simplifies testing
* improves readability
* enforces clean architecture

---

### 4. Closures as Behavior

Filtering logic is passed **from the outside**:

```rust
|e| matches!(e, AuditEvent::OrderPaid { .. })
```

The pipeline does not know domain rules — it only applies them.

---

## 🗂️ Project Structure

```
day19_audit_log/
  Cargo.toml
  README.md
  src/
    lib.rs        # public API
    domain.rs     # AuditEvent + Display
    format.rs     # Formatter + Plain / Compact
    sink.rs       # Sink + VecSink
    pipeline.rs   # render + emit_events
  tests/
    audit.rs      # integration tests
```

### Module Responsibilities

| Module     | Responsibility                |
| ---------- | ----------------------------- |
| `domain`   | Domain data and meaning       |
| `format`   | Formatting policies           |
| `sink`     | Side effects (writing output) |
| `pipeline` | Orchestration                 |

---

## 🧩 Domain Events

```rust
pub enum AuditEvent {
    OrderCreated { id: u64 },
    OrderPaid { id: u64, tx: String },
    OrderCancelled { id: u64, reason: String },
}
```

* Strongly typed
* Exhaustive `match`
* Stable, human-readable `Display` implementation

---

## 🎨 Formatting

### `PlainFormatter`

* Delegates to `Display`
* Human-readable, detailed output

### `CompactFormatter`

* Own formatting policy
* Short, dense output (`PAID#42`)
* Does **not** rely on `Display`

Formatters are fully interchangeable — the pipeline never changes.

---

## 🚰 Sink (Side Effects)

```rust
pub trait Sink {
    fn write(&mut self, line: String);
}
```

Implementation:

* `VecSink` — in-memory sink
* Exposes output as `&[String]`
* Prevents external mutation

---

## 🔁 Pipeline

### Pure Layer

```rust
pub fn render(
    events: &[AuditEvent],
    formatter: &impl Formatter,
    filter: impl Fn(&AuditEvent) -> bool,
) -> Vec<String>
```

### Effectful Layer

```rust
pub fn emit_events<F, S, P>(...)
where
    F: Formatter,
    S: Sink,
    P: Fn(&AuditEvent) -> bool,
```

Data flow:

```
events → predicate → formatter → sink
```

No branching, no leakage, no duplication.

---

## 🧪 Testing Strategy

### Unit Tests

* `domain.rs`: `Display` for every variant
* Edge cases (empty strings, zero IDs)

### Integration Tests (`tests/audit.rs`)

Covers:

* plain vs compact formatting
* filtering via closures
* ordering guarantees
* `emit_events` behavior
* sink persistence
* empty inputs
* complex predicates

> Integration tests use **only the public API**, exactly like real consumers.

---

## 📊 Coverage Matrix

| Topic                  | Location                          | Verified By       |
| ---------------------- | --------------------------------- | ----------------- |
| Trait contracts        | `Formatter`, `Sink`               | compile + tests   |
| Bounds                 | `emit_events<F,S,P>`              | integration tests |
| Closures               | `render`, `emit_events`           | filter tests      |
| Enum + match           | `AuditEvent + Display`            | unit tests        |
| Separation of concerns | domain / format / sink / pipeline | end-to-end tests  |

---

## ✅ Senior Definition of Done

* ❌ no `unwrap/expect` in production paths
* ✅ clear trait contracts
* ✅ minimal, meaningful bounds
* ✅ filtering via closures
* ✅ order preserved
* ✅ side effects isolated
* ✅ happy + edge cases tested

---

## 🧾 Decision Log

* Chose traits (`Formatter`, `Sink`) to decouple behavior from data and avoid branching.
* Used generics with bounds to keep polymorphism at compile time.
* Separated pure (`render`) and effectful (`emit_events`) logic for clarity and testability.
* Delegated human-readable output to `Display`, while allowing alternative formatting policies.
* Used closures for filtering to keep the pipeline domain-agnostic.
* Implemented `VecSink` as a deterministic in-memory sink.
* Explicitly verified ordering guarantees in tests.
* Left validation rules (e.g. empty tx/reason) for future domain evolution.
* Next iteration: `make_formatter(kind) -> impl Formatter`, optional `Box<dyn Sink>` for plugin sinks.

---



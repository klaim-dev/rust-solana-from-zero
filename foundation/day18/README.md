# 📅 Day 18 — Enums & Match: Type-Safe Domain States (Order Lifecycle)

> **Foundation Rust — Day 18**
> Transition from “flags & strings” to **type-safe domain architecture** using `enum + match`.

This day introduces **domain-driven state modeling** in Rust.
Instead of booleans, strings, or ad-hoc flags, we model an **Order lifecycle** as a **finite state machine (FSM)** where:

> ❝ *Invalid states are impossible to represent.* ❞

---

## 🎯 Goal of the Day

Build a **production-style domain module** that models an `Order` lifecycle with:

* Exactly one valid state at any time
* Explicit, controlled transitions
* No panics, no `unwrap` in production paths
* All invalid transitions expressed as domain errors
* Business rules enforced by the type system

---

## 🧠 Core Idea

We use:

* `enum OrderState` — to represent **mutually exclusive states**
* `match` — as an **explicit decision table**
* Domain methods (`submit / pay / cancel`) — to control state transitions
* An aggregate (`Order`) to enforce **invariants and editing rules**

This is the **same architectural approach used in real production backends**.

---

## 🧱 Project Structure

```text
day18/
  src/
    lib.rs
    domain/
      mod.rs
      error.rs      # Domain error contract
      state.rs      # OrderState FSM (enum + transitions)
      line_item.rs  # LineItem value object
      order.rs      # Order aggregate root
  tests/
    order_lifecycle.rs
  README.md
```

### Responsibility Split

| Module         | Responsibility                                  |
| -------------- | ----------------------------------------------- |
| `error.rs`     | Domain error contract                           |
| `state.rs`     | Pure FSM (OrderState + transitions)             |
| `line_item.rs` | Item validation & money logic                   |
| `order.rs`     | Aggregate: invariants, editing rules, lifecycle |
| `tests/`       | Integration tests (real usage)                  |

No IO. No time sources. No external systems.
**Pure domain logic.**

---

## 🧩 Domain Model

### Order States (Baseline)

```text
Draft
Submitted { at }
Paid { at, tx_id }
Cancelled { at, reason }
```

### Allowed Transitions

| From      | To        |
| --------- | --------- |
| Draft     | Submitted |
| Draft     | Cancelled |
| Submitted | Paid      |
| Submitted | Cancelled |
| Paid      | ❌ none    |
| Cancelled | ❌ none    |

Any other transition returns a **domain error**.

---

## 🔒 Domain Invariants

### Order

* Exists in **exactly one state**
* Items editable **only in Draft**
* Cannot submit an empty order
* Lifecycle changes only via domain methods

### LineItem

* `sku` trimmed & normalized
* `qty > 0`
* Money arithmetic checked (`checked_mul`)
* No silent overflows

### OrderId

* Must be `> 0`
* Enforced at construction

---

## ⚠️ Error Handling Strategy

All failures are explicit and typed:

* `InvalidTransition`
* `OrderNotEditable`
* `EmptyOrder`
* `DuplicateItem`
* `ItemNotFound`
* `ZeroQuantity`
* `EmptyTxId`
* `EmptyCancelReason`
* `InvariantViolation`

No panics.
No implicit failure modes.

---

## 🧪 Tests

Integration tests cover:

### Happy Paths

* Draft → add items → submit
* Submit → pay
* Draft → cancel
* Submit → cancel
* Quantity update in Draft
* Total calculation

### Negative Paths (≥3)

* Submit empty order
* Edit after submit
* Invalid transitions (Draft → Pay, Paid → Cancel, etc.)
* Quantity set to zero

Tests verify **behavior**, not internal fields.

---

## 🧾 Coverage Matrix

| Subtopic                 | Where               | Verified By         |
| ------------------------ | ------------------- | ------------------- |
| Enum state modeling      | `OrderState`        | integration tests   |
| match completeness       | FSM transitions     | compiler + tests    |
| Invalid transitions      | `InvalidTransition` | negative tests      |
| Editing rules            | `ensure_editable`   | mutate_after_submit |
| Option/Result discipline | validation          | tests               |
| No impossible states     | enum design         | code structure      |

---

## ✅ Senior Definition of Done (Day 18)

* ❌ No `unwrap` / `expect` in production code
* ✅ Enum-based FSM (no strings/flags)
* ✅ All invalid transitions are errors
* ✅ ≥3 negative tests
* ✅ Domain rules enforced by types
* ✅ Clean module boundaries

---

## 🧠 Key Takeaways

* `enum + match` turns business rules into compiler-checked logic
* Domain methods are **policy**, not helpers
* Aggregates protect invariants
* Errors are part of the API, not exceptions
* Rust’s type system is an architectural tool, not just syntax

---

## 🔜 What Comes Next

* Stretch states (`Refunded`, `PaymentFailed`)
* Event emission
* Persistence mapping
* API / backend integration

---

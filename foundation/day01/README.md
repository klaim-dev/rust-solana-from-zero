# Day 1 — Variables, `String` / `&str`, Stack & Heap

Public learning **skeleton** for Day 1 — no solutions.  
Focus: fundamentals that everything else will lean on.

---

## Overview

Core topics:

- `let`, `let mut`, and shadowing
- Primitive types: `u32`, `bool`
- `&str` vs `String` in real APIs
- Intuition: stack vs heap
- String slices + ASCII vs Unicode gotchas
- First self-checks with `assert_eq!`
- No `unwrap` / `expect` in production-style code
- `struct` as a simple data container (no methods yet)

This file is a **public-facing spec**, not an answer sheet.

---

## Super Task

Design a minimal **`User` domain module** that forces you to touch:

- basic types and variables,
- `String` vs `&str`,
- stack/heap intuition,
- simple formatting.

### Domain Model

```rust
pub struct User {
     name: String,
     age: u32,
}
```

### Functions (no impl yet)

```rust
pub fn greet(user: &User) -> String;        // "Hello, Alice (30)!"
pub fn name_len(user: &User) -> usize;     // byte length of name
pub fn user_is_adult(user: &User) -> bool; // age >= 18

// Stretch:
pub fn report_name_layout(user: &User) -> (usize, usize); // (len, cap)
```

### Invariants (Day 1 scope)

- `name` after normalization is not empty (ASCII only for now).
- `age` is `u32` (cannot be negative by type).
- No `unwrap` / `expect` / `panic` in these functions.

**Baseline:**

- `User`
- `greet`
- `name_len`
- `user_is_adult`

**Stretch:**

- `report_name_layout`
- Using `normalize_name` when constructing `User`.

---

## Minimal Viable Theory (MVT)

### 1. `let`, `mut`, shadowing

Immutable by default; `mut` only when actual state changes.

```rust
let x = 10;
let mut y = 5;
y += 1;

let name = "alice";
let name = name.len(); // shadowing with new type is fine
```

### 2. Basic types

Pick types deliberately:

- `u32` for age,
- `bool` for flags,
- `&str` for borrowed views,
- `String` for owned, growable text.

Day 1 rule:  
**Accept** references where possible; **return** owned values when you build new data.

### 3. `String` vs `&str`

- `&str`: borrowed slice; cheap; often function input.
- `String`: owned heap buffer; has `len()` + `capacity()`, can grow.

### 4. Stack vs Heap (intuition only)

- Stack: `User` header, integers, bools.
- Heap: bytes of `name: String`.

Takeaway: `String` is heavier; avoid pointless cloning.

### 5. String slices & UTF-8

- `.len()` → bytes.
- ASCII examples like `"Alice"[..2]` ok.
- For Unicode, slicing by bytes is dangerous; we’ll handle later.

Day 1: **assume ASCII only** when you slice.

### 6. `struct` as a container

```rust
pub struct User {
    pub name: String,
    pub age: u32,
}
```

No methods, no traits, no lifetimes yet.  
Just a clear, predictable shape.

---

## Micro Exercises (no solutions)

1. **Mut vs immut**

   Show where `mut` is required and where not:

   ```rust
   let mut age = 20;
   age += 1;
   ```

2. **`String` vs `&str`**

   - Create a `String` from `&str`.
   - Print `len()` and `capacity()`.
   - Note why `&str` has no `capacity()`.

3. **Stack / Heap trace**

   In a `fn demo(user: User)`, comment where `user` lives and where `user.name` bytes live.

4. **ASCII slice**

   Use `"Alice"[..2]` — explain why it is safe here.

5. **Unicode gotcha**

   Think about `"Алла"[..2]` — explain why this is unsafe in general.

---

## Mini Challenge

Implement:

```rust
pub fn normalize_name(raw: &str) -> String
```

### Requirements

- `trim` whitespace.
- lowercase all letters.
- uppercase first character (ASCII only for Day 1).
- empty/whitespace input → empty string.
- A few `assert_eq!` checks as self-tests.

Examples:

- `"  alice  "` → `"Alice"`
- `"ALICE"` → `"Alice"`

Multi-word behavior (e.g. `"alice bob"`) — your choice; document it.

---

## Suggested Layout

Follow the global repo pattern:

```text
rust/foundation/day01/
  Cargo.toml
  src/
    main.rs
    domain/
      user.rs
    utils/
      normalize.rs
```

**`domain/user.rs` (public skeleton)**

```rust
pub struct User {
     name: String,
     age: u32,
}

pub fn greet(user: &User) -> String {
    todo!()
}

pub fn name_len(user: &User) -> usize {
    todo!()
}

pub fn user_is_adult(user: &User) -> bool {
    todo!()
}

// Stretch
pub fn report_name_layout(user: &User) -> (usize, usize) {
    todo!()
}
```

**`utils/normalize.rs`**

```rust
pub fn normalize_name(raw: &str) -> String {
    // Implement in your local copy.
    todo!()
}
```

> In the public repo this stays as a **spec**.  
> Your local solutions live in your working copy / private fork.

**`main.rs`**

- Create a couple of `User` values.
- Use `normalize_name` before storing `name`.
- Print:
  - `greet(user)`
  - `name_len(user)`
  - `user_is_adult(user)`

---

## Acceptance Criteria (Baseline)

- `User` struct compiles and is used.
- `greet` returns stable, readable text.
- `name_len` returns byte length.
- `user_is_adult` uses `age >= 18` once (no scattered magic numbers).
- No `unwrap` / `expect` in domain or utils.
- Names are descriptive.

**Stretch:**

- `report_name_layout` implemented.
- `normalize_name` consistently used.
- A couple of `assert_eq!` checks or a small test module.

---

## Coverage Matrix

| Topic                 | Where                        | How Checked              |
|-----------------------|-----------------------------|--------------------------|
| `let` / `mut`         | `main.rs`, micros           | compile & intent review  |
| Basic types           | `User`, signatures          | type checks              |
| `String` vs `&str`    | `normalize`, `greet`        | API choices              |
| Stack / heap intuition| comments in `user.rs`       | explicit explanation     |
| String formatting     | `greet`                     | output shape             |
| ASCII/Unicode slices  | micro-exercises             | reasoning only           |
| No unwrap in prod     | all `src`                   | quick scan               |
| Assertions            | `normalize` tests/comments  | run locally              |

---

## Senior Checklist

- ❌ No `unwrap` / `expect` in domain and utils.
- ✅ Clear names and small functions.
- ✅ `User` is minimal and predictable.
- ✅ String logic isolated; domain stays clean.
- ✅ No unnecessary `clone`; prefer borrowing.

---

## Decision Log (to fill after coding)

After you complete Day 1, write 8–10 lines for yourself:

- What clicked about stack vs heap.
- How `String` vs `&str` affected your APIs.
- Where you almost used `unwrap` and how you avoided it.
- How you’d extend `normalize_name` on later days.
- Any “this felt different from other languages” notes.

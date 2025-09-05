# Day 1 — Variables, `String` vs `&str`, Stack & Heap

Public learning **skeleton** for Day 1. No solutions — only structure, tasks, and signatures.
Use this as a workbook starter or a forkable template.

---

## Overview

- Mutability: `let` vs `let mut`
- Memory model: stack vs heap
- `String` (owned, growable) vs `&str` (borrowed view)
- Byte length vs character count
- Safe slicing guidelines (ASCII vs Unicode)
- String formatting with `format!`

---

## Learning Goals

Learn to:
- Describe how `String` is represented (`ptr / len / cap` on the stack; bytes in the heap)
- Choose between `String` and `&str` by ownership and mutability
- Measure byte length and reason about Unicode slicing risks
- Organize a tiny domain module and a utility module
- Meet a simple acceptance checklist without `unwrap`/`expect` in the main path

---

## Day Plan

1. **Theory checklist** — confirm core ideas.
2. **Micro-exercises** — short, focused tasks.
3. **Mini‑challenge** — one function with clear criteria.
4. **Super‑task (skeleton)** — project layout + signatures.
5. **Wrap‑up** — refactor, coverage checklist, short decision log.

---

## Deliverables

- `day1/README.md` (this file)
- `src/` skeleton with `domain/` and `utils/`
- Function signatures filled with `todo!()` (compiles; panics at runtime)

> Tip: If this folder is not in a Cargo project yet, run `cargo init` at the repo root.
> The skeleton compiles but will panic on `todo!()` when executed.

---

## Micro‑Exercises (no solutions)

1. Declare `let mut age = 30;` then change and print it.
2. Create `let s = String::from("Alice");` — print `s.len()` and `s.capacity()`.
   Compare with `let slice: &str = "Alice";` (no `capacity()`).
3. Take an ASCII slice: `"Alice"[..2]` — explain why slicing is **byte‑based**.
4. Try Unicode: slice `"Été"[..2]` or `"你好"[..2]` — observe the behavior.  
   Then get the first 2 **characters** via `.chars().take(2).collect::<String>()`.
5. Build a greeting using `format!("Hello, {} ({})!", name, age)`.

---

## Mini‑Challenge

Implement:

```rust
pub fn normalize_name(raw: &str) -> String
```

**Requirements:**
- Trim leading/trailing whitespace
- First letter uppercase (ASCII), the rest lowercase
- Examples:  
  - `"  alice  "` → `"Alice"`  
  - `"ALICE"` → `"Alice"`

(Optionally add `assert_eq!` self‑checks in your scratch file.)

---

## Super‑Task (skeleton only)

**Domain goal:** a minimal `User` module.

- Model: `User { name: String, age: u32 }`
- Functions (free functions for Day 1):
  - `greet(&User) -> String` — e.g., `"Hello, Alice (30)!"`
  - `name_len(&User) -> usize` — byte length of `name`
  - `report_string_layout(&User) -> (usize /* len */, usize /* cap */)` — for `String`

**Invariants:**
- `name` is non‑empty (ASCII for Day 1)
- `age ≥ 0`

**Acceptance (baseline):**
- `User` exists; `greet` and `name_len` behave correctly for ASCII names
- No `unwrap`/`expect` in the main path
- Clear module boundaries and naming

**Stretch (optional):**
- Implement `report_string_layout`
- Use `normalize_name` during `User` creation

---

## Repository Structure

```
day1/
  README.md
  src/
    main.rs
    domain/
      mod.rs
      user.rs
    utils/
      mod.rs
      normalize.rs
```

---

## Code Skeleton (compiles; runtime calls will `todo!()`)

**`src/main.rs`**
```rust
mod domain;
mod utils;

fn main() {
    println!("Day 1 skeleton — implement the TODOs in domain::user and utils::normalize");
    // Intentionally no runtime logic here for Day 1.
}
```

**`src/domain/mod.rs`**
```rust
pub mod user;
```

**`src/domain/user.rs`**
```rust
pub struct User {
    pub name: String,
    pub age: u32,
}

pub fn greet(_u: &User) -> String {
    todo!("Return a greeting, e.g., "Hello, Alice (30)!"")
}

pub fn name_len(_u: &User) -> usize {
    todo!("Return byte length of the user's name")
}

pub fn report_string_layout(_u: &User) -> (usize, usize) {
    todo!("Return (len, cap) for the user's name String")
}
```

**`src/utils/mod.rs`**
```rust
pub mod normalize;
```

**`src/utils/normalize.rs`**
```rust
pub fn normalize_name(_raw: &str) -> String {
    todo!("Trim, uppercase first letter (ASCII), lowercase the rest")
}
```

---

## Guidelines

- Prefer `format!` over `+` for string building.
- Slicing is **byte‑based**; use ASCII only for direct slices in Day 1.
- For Unicode‑safe prefixes, iterate over `.chars()`.
- Keep public APIs minimal and names domain‑specific.
- Avoid `unwrap` / `expect` in the main path.

---



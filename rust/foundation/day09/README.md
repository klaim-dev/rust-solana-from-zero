# Day 9 — `struct`, `impl`, Methods (`self` / `&self` / `&mut self`)

Public learning **skeleton** for Day 9 (no solutions).  
Focus: designing a domain `User` with **encapsulation**, **constructors**, **invariants**, and **safe methods**.  
No `unwrap/expect/panic` in production code.

> If you haven’t added it yet, Day 7 introduced `thiserror` for readable errors.
> ```toml
> [dependencies]
> thiserror = "1"
> ```

---

## Overview

- `struct` design and **encapsulation** (private fields)
- `impl` blocks and receivers: `self`, `&self`, `&mut self`
- Associated functions (constructors) vs instance methods
- Validations as **contract** (domain invariants at ctor/setters)
- Borrowed returns (`&str` from internal `String` safely)
- Optional method chaining (fluent style)

---

## Learning Goals

Learn to:
- Encapsulate state and expose a **minimal public API**
- Choose the right receiver (`&self` for reads, `&mut self` for mutation, associated fns for creation)
- Enforce invariants in **one place** (ctor + setters)
- Return borrowed views without allocations
- Keep methods **pure** (no printing) except dedicated formatters

---

## Day Plan

1. **Theory checklist** — struct privacy, receivers, ctors vs methods, invariants.
2. **Micro‑exercises** — rewrite free functions into methods.
3. **Mini‑challenge** — method signatures + error enum (no bodies).
4. **Super‑task (skeleton)** — full `User` API (signatures) + tests to target.
5. **Wrap‑up** — coverage matrix, senior checklist, decision log.

---

## Deliverables

- `day9/README.md` (this file)
- Signatures only (no implementations) for: `User`, `impl`, `UserError`, and `normalize::name_ascii`
- Suggested tests: ≥ 1 happy + ≥ 6 negative

---

## Invariants (contract)

- `name`: after **normalize** (`trim` + ASCII titlecase) — non‑empty, length `1..=64`
- `age`: in range `0..=150`
- `email`: simple check — contains `'@'` and `'.'` (no external crates)

---

## Public API — Signatures Only

```rust
// domain/user.rs
#[derive(Debug)]
pub struct User {
    name: String,   // encapsulated
    age:  u32,
    email: String,
}

impl User {
    pub const MAX_NAME: usize = 64;
    pub const MAX_AGE:  u32  = 150;

    // Associated fns (constructors)
    pub fn try_new(name: &str, age: u32, email: &str) -> Result<Self, UserError>;
    /// For tests/internal use only. Skips validation.
    pub fn new_unchecked(name: String, age: u32, email: String) -> Self;

    // Read methods
    pub fn name(&self) -> &str;
    pub fn age(&self) -> u32;            // Copy → return by value
    pub fn email(&self) -> &str;
    pub fn email_domain(&self) -> &str;  // borrowed slice of self.email

    // Mutating methods (with validation)
    pub fn rename(&mut self, name: &str) -> Result<(), UserError>;
    pub fn set_email(&mut self, email: &str) -> Result<(), UserError>;
    pub fn have_birthday(&mut self) -> Result<(), UserError>; // age += 1, do not exceed MAX_AGE

    // Utilities
    pub fn greet(&self) -> String;       // "Hello, Alice (30)!"
    pub fn name_len(&self) -> usize;     // byte length
}

// Simple ASCII normalization utility
pub mod normalize {
    pub fn name_ascii(title: &str) -> String; // trim + first upper + rest lower (ASCII-only)
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum UserError {
    #[error("name is empty after normalization")]
    EmptyName,
    #[error("name is too long (max {max})")]
    NameTooLong { max: usize },
    #[error("age out of range (0..={max})")]
    AgeOutOfRange { max: u32 },
    #[error("email format is invalid")]
    InvalidEmail,
    #[error("age overflow at max")]
    AgeOverflow,
}
```

**Notes**

- **No printing** inside `impl`; only `greet` builds a string.
- Fields are **private**; all access flows through methods.
- Constructors and setters enforce all invariants.

---

## Micro‑Exercises (no solutions)

1. Choose receivers: convert four free functions (`greet`, `name_len`, `rename`, `have_birthday`) into methods with the correct `&self` / `&mut self` signatures.
2. Implement a **borrowed** view: design `email_domain(&self) -> &str` (e.g., via `rfind('@')`).
3. Write `normalize::name_ascii(&str) -> String` (ASCII only) and show why Unicode titlecasing needs a different approach.
4. Demonstrate why `pub name: String` breaks invariants; fix by keeping the field private and validating in `rename`.
5. Replace a panicking `new(...)` with `try_new(...) -> Result<Self, UserError>`.

---

## Mini‑Challenge (signatures only)

- Move free functions into methods: `greet`, `name_len` → `&self`
- Place name normalization into `normalize::name_ascii`
- Create `try_new` to validate all invariants; keep `new_unchecked` for tests only (documented as such)

**Criteria**

- No `println!` in `impl`
- All validations live in constructors/setters; no panics

---

## Super‑Task (skeleton only)

**Structure**
```
src/
  main.rs
  domain/
    user.rs        // struct + impl + UserError + normalize
```

**Target tests (at least 7)**

1. **Happy**: `try_new(" alice ", 30, "a@b.com")` → `name == "Alice"`, `email_domain == "b.com"`
2. **EmptyName** after normalization (`"   "`)
3. **NameTooLong** (65 chars)
4. **AgeOutOfRange** (`age = 200`)
5. **InvalidEmail** (no `@` or no dot)
6. **AgeOverflow**: `age = MAX_AGE`; `have_birthday()` → error
7. **Setters**: `rename(..)` / `set_email(..)` respect invariants (negative cases)

**Acceptance (baseline)**

- Private fields; access only via methods
- Correct receivers (`&self`/`&mut self`) chosen
- All invariants enforced in constructors/setters
- No `unwrap/expect/panic`

**Stretch**

- Fluent chaining variant:
  ```rust
  impl User {
      pub fn rename_mut(&mut self, name: &str) -> Result<&mut Self, UserError> { /* ... */ }
  }
  ```
- Tiny helper to improve readability:
  ```rust
  #[inline]
  fn ensure<T>(cond: bool, err: T) -> Result<(), T> {
      if cond { Ok(()) } else { Err(err) }
  }
  ```

---

## Coverage Matrix

```
Topic                             Where                   Check
----------------------------------------------------------------------------------
struct + encapsulation            domain/user.rs          fields are private
impl + receivers                  User methods            &self / &mut self used correctly
assoc fn vs method                try_new / new_unchecked ctor doesn’t bypass invariants
getters / setters                 name/age/email/rename   borrowed returns; Result in setters
domain invariants                 try_new + setters       negative tests cover violations
no unwrap/expect/panic            entire module           review + compiler
```

---

## Senior Checklist

- ❌ No `unwrap/expect/panic` in production path
- ✅ Private fields; all mutation via methods
- ✅ Borrowed returns (`&str`) uphold lifetimes tied to `&self`
- ✅ ≥ 7 tests: happy + negative branches
- ✅ Clear API: ctor/getters/setters/utilities

---

## Decision Log (8–10 lines, template)

1. Moved free functions into `impl User`; chose correct receivers.
2. Encapsulated fields; validations possible only via ctor/setters.
3. Introduced `try_new`; removed any panicking constructors.
4. Normalized name in ASCII mode; deferred Unicode titlecasing.
5. `email_domain` returns `&str` without allocation.
6. Setters return `Result`; invariants are enforced.
7. Tests cover happy and all negative branches.

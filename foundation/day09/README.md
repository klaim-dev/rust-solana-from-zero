
# Day 9 — `struct`, `impl`, methods, and User validation

Reference notes for Foundation Day 9. This repo already contains an example `User` implementation in `src/main.rs` with tests.

## Table of Contents
- [Repository Status](#repository-status)
- [Goals](#goals)
- [Base Knowledge](#base-knowledge)
- [Super Task: user module](#super-task-user-module)
- [Minimal Theory](#minimal-theory)
- [Micro Tasks](#micro-tasks)
- [Mini Task: account module](#mini-task-account-module)
- [Super Task: user implementation](#super-task-user-implementation)
- [Finish: checklists](#finish-checklists)
- [Retro](#retro)

## Repository Status
- Implemented: `User` domain model and validation in `src/main.rs`, including getters, `is_adult`, `rename`, `set_email`, `activate`, and `deactivate`.
- Tests live in `src/main.rs` under `#[cfg(test)]` and cover happy paths plus negative cases for email/name.
- Not implemented: `with_default_flags` stretch constructor; `account` mini-task (left as an optional exercise).
- `main()` currently prints “Hello, world!”; it does not wire up the domain logic.

## Goals
- Define domain models with `struct`.
- Use `impl`: **associated functions** (`User::new`) vs **methods** (`user.is_adult()`).
- Know when to use `self` / `&self` / `&mut self`.
- Keep invariants in constructors and mutating methods.
- Use `Result` + domain errors (`thiserror`) and `Option` inside methods.
- Avoid `unwrap/expect` in production logic.

## Base Knowledge
- Days 4–5: ownership and borrowing (`&` / `&mut`).
- Day 6: `Option` for “maybe value”.
- Day 7: `Result` + `thiserror` for domain errors.

## Super Task: user module
Build a `user` module with a `User` model and validation in constructors and methods. In this repo the baseline is implemented in `src/main.rs`.

### Model
```rust
pub struct User {
    id: u64,
    name: String,
    email: Option<String>,
    age: u8,
    is_active: bool,
}
```

### Invariants
- `name`: after `trim` it must not be empty (`"  "` → error).
- `age`: must be in `0..=120` (business rule).
- `email` (if provided): contains `'@'`, no spaces, pragmatic check without regex.
- `id`: strictly `> 0`.
- `is_active`: status flag, not part of input validation directly.

### Domain errors
```rust
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum UserError {
    #[error("id must be positive, got {0}")]
    InvalidId(u64),

    #[error("name must not be empty")]
    EmptyName,

    #[error("age must be in 0..=120, got {0}")]
    InvalidAge(u8),

    #[error("invalid email format: {0}")]
    InvalidEmail(String),
}
```

### Baseline API
```rust
impl User {
    /// Full constructor with validation.
    pub fn new(
        id: u64,
        name: String,
        email: Option<String>,
        age: u8,
    ) -> Result<Self, UserError>;

    /// Accessors without unnecessary copies.
    pub fn id(&self) -> u64;
    pub fn name(&self) -> &str;
    pub fn email(&self) -> Option<&str>;
    pub fn age(&self) -> u8;
    pub fn is_active(&self) -> bool;

    /// Business logic.
    pub fn is_adult(&self) -> bool;

    /// Mutations that preserve invariants.
    pub fn rename(&mut self, new_name: &str) -> Result<(), UserError>;
    pub fn set_email(&mut self, new_email: Option<String>) -> Result<(), UserError>;
    pub fn deactivate(&mut self);
    pub fn activate(&mut self);
}
```

### Stretch API
```rust
impl User {
    /// Constructor with trimming and default active state.
    pub fn with_default_flags(
        id: u64,
        raw_name: &str,
        raw_email: Option<&str>,
        age: u8,
    ) -> Result<Self, UserError>;
}
```
Status: not implemented in the current code (left as stretch).

### Acceptance (baseline)
- `User::new` is implemented and enforces all invariants.
- Getters return references (`&str` / `Option<&str>`) without extra copies.
- `rename`, `set_email`, `activate` / `deactivate` work without panics.
- Current suite: 10 tests covering rename/email/is_adult/activation with negative cases for empty names and invalid emails; add id/age edge cases if you extend.

## Minimal Theory

### 1. `struct` as a domain model
```rust
pub struct Point {
    pub x: i32,
    pub y: i32,
}
```
- Named fields suit domain entities (User, Order, Account…).
- Fields can be `pub` or private.

**Takeaway:** a `struct` is where domain invariants live.

### 2. `impl`: methods and associated functions
```rust
impl Point {
    pub fn origin() -> Self {
        Self { x: 0, y: 0 }
    }

    pub fn len_sq(&self) -> i32 {
        self.x * self.y
    }

    pub fn translate(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }
}
```
- Associated functions: no `self`, called as `Type::function()`.
- Methods: first param is `self` / `&self` / `&mut self`, called on a value.

**Takeaway:** `impl` keeps behavior close to data.

### 3. `self` / `&self` / `&mut self`
- `&self`: read-only.
- `&mut self`: mutating.
- `self`: consumes the value and returns something new.

**Rule:** read → `&self`; change → `&mut self`; consume → `self`.

### 4. Validation in `new`
```rust
impl User {
    pub fn new(
        id: u64,
        name: String,
        email: Option<String>,
        age: u8,
    ) -> Result<Self, UserError> {
        // validation + Result
    }
}
```
- Constructor is the right place for invariants.
- Any created `User` is guaranteed valid.

**Takeaway:** bypassing `new` leaks invariants — keep fields private.

### 5. Getters without clones
```rust
impl User {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn email(&self) -> Option<&str> {
        self.email.as_deref()
    }
}
```
- Return `&str` and `Option<&str>` to avoid allocations and ownership headaches.

## Micro Tasks

### Micro 1 — simple struct + method
```rust
pub struct Rect {
    pub width: u32,
    pub height: u32,
}

impl Rect {
    pub fn area(&self) -> u32 {
        // TODO
    }
}
```
Test `area()` with a few values.

### Micro 2 — `new` + getters
```rust
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        // TODO
    }

    pub fn x(&self) -> i32 { /* ... */ }
    pub fn y(&self) -> i32 { /* ... */ }
}
```
- Keep fields private.
- Access only through getters.

### Micro 3 — mutating method
```rust
impl Point {
    pub fn translate(&mut self, dx: i32, dy: i32) {
        // TODO
    }
}
```
- Ensure it compiles: `let mut p = Point::new(...); p.translate(...);`.

### Micro 4 — method consuming `self`
```rust
impl Point {
    pub fn moved(self, dx: i32, dy: i32) -> Self {
        // TODO: return a new Point, consuming self
    }
}
```
- Compare with `translate`: in-place vs “return a new one”.

### Micro 5 — `Option` in methods
```rust
pub struct Profile {
    nickname: Option<String>,
}

impl Profile {
    pub fn nickname_or_default(&self) -> &str {
        // TODO: nickname if Some, otherwise "anonymous"
    }
}
```
- Use `as_deref()` or `map` + `unwrap_or`.

## Mini Task: module `account` (~45 min)
Build a small `account` module (not implemented in this repo; use as an exercise):
```rust
use thiserror::Error;

pub struct Account {
    id: u64,
    owner: String,
    balance_cents: i64,
    is_blocked: bool,
}

#[derive(Error, Debug, PartialEq)]
pub enum AccountError {
    #[error("id must be positive, got {0}")]
    InvalidId(u64),

    #[error("initial balance cannot be negative, got {0}")]
    NegativeInitialBalance(i64),

    #[error("cannot withdraw {amount}, balance is {balance}")]
    InsufficientFunds { amount: i64, balance: i64 },

    #[error("account is blocked")]
    Blocked,
}

impl Account {
    pub fn open(
        id: u64,
        owner: String,
        initial_balance_cents: i64,
    ) -> Result<Self, AccountError>;

    pub fn deposit(&mut self, amount_cents: i64) -> Result<(), AccountError>;
    pub fn withdraw(&mut self, amount_cents: i64) -> Result<(), AccountError>;
    pub fn block(&mut self);
    pub fn unblock(&mut self);
    pub fn balance(&self) -> i64;
}
```

**Rules:**
- `open`: `id == 0` → `InvalidId`; `initial_balance_cents < 0` → `NegativeInitialBalance`.
- `deposit`: define a policy for negative amounts (e.g., reject).
- `withdraw`: if `is_blocked` → `Blocked`; if insufficient funds → `InsufficientFunds`; otherwise decrease balance.
- At least 5 tests (happy path + negative cases).

## Super Task: user implementation (90 min)
File: `src/main.rs` in this repo contains the `user` module. Stretch constructor is still TODO; everything else is in place.

### Target module template
Original skeleton for reference; the implementation in `src/main.rs` already fills in these TODOs (except the stretch constructor).
```rust
use thiserror::Error;

#[derive(Debug, PartialEq)]
pub struct User {
    id: u64,
    name: String,
    email: Option<String>,
    age: u8,
    is_active: bool,
}

#[derive(Error, Debug, PartialEq)]
pub enum UserError {
    #[error("id must be positive, got {0}")]
    InvalidId(u64),

    #[error("name must not be empty")]
    EmptyName,

    #[error("age must be in 0..=120, got {0}")]
    InvalidAge(u8),

    #[error("invalid email format: {0}")]
    InvalidEmail(String),
}

impl User {
    // ...
    // (implementation in src/main.rs; TODO markers were part of the original exercise)
}

#[cfg(test)]
mod tests {
    use super::*;

    // at least 6–8 tests:
    // - new_happy_path
    // - new_invalid_id
    // - new_empty_name
    // - new_invalid_age
    // - new_invalid_email
    // - rename_trims_and_rejects_empty
    // - set_email_clears_with_none
    // - is_adult_works_for_boundaries
}
```

### Acceptance
- Code compiles.
- All invariants are enforced.
- Tests in `src/main.rs` are green; add more if you extend functionality.
- No `unwrap/expect` in production code.

## Finish: checklists

### Coverage Matrix
| Topic                     | Where                       | How verified                                      |
| ------------------------- | --------------------------- | ------------------------------------------------- |
| struct as domain model    | `User` in `src/main.rs`     | type signatures, usage in tests                   |
| impl + associated fn      | `User::new`                 | compilation + tests                               |
| methods (`&self`, `&mut`) | `User` methods              | borrow rules, mutation behavior                   |
| invariants in `new`       | `User::new`                 | negative tests on empty names/invalid emails      |
| Option inside struct      | `email: Option<String>`     | `email()` / `set_email` behavior                  |
| Result + thiserror        | `UserError`                 | pattern matching, Display messages                |
| no unwrap in production   | `src/main.rs` user logic    | code review / grep                                |
| ≥ 3 negative tests        | tests in `src/main.rs`      | empty name, missing `@`, email with spaces        |

### Senior Checklist
- ❌ No `unwrap/expect` in production modules.
- ✅ Structs are domain models, not “bags of fields”.
- ✅ Invariants are enforced in `new`; mutations do not break them.
- ✅ Methods use `&self` / `&mut self` intentionally, without extra clones.
- ✅ Negative cases are covered by tests.

### Decision Log (8–10 lines, your words)
1. Which invariants for `User` you chose and why.
2. Where you wanted to make fields `pub`, but kept them private.
3. What minimal email validation rule you adopted.
4. Why you chose `u8` for age and `u64` for id.
5. How each `UserError` variant helps debugging/logging.
6. Where `&mut self` was required vs where `&self` was enough.

## Retro
1. Explain in 30 seconds the difference between an associated function (`User::new`) and a method (`user.is_adult()`).
2. Does “constructors own invariants” feel natural now?
3. Which 1–2 struct+impl patterns will you apply automatically (private fields + `new` + getters)?

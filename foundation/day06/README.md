# Day 6 — `Option`, `match`, `if let`

Public learning **skeleton** for Day 6 — no solutions.  
Theme: honest handling of “maybe there / maybe not” with `Option<T>`, exhaustive `match`, and `if let` as sugar. No fake sentinel values, no `unwrap` in production logic.

This day builds on Days 1–5: ownership, borrowing, and clean APIs.

---

## Overview

Core topics:

- when to use `Option<T>` instead of fake values (`0`, `""`, `-1`, etc.)
- exhaustive `match` on `Option<T>` (`Some` + `None`)
- `if let` as a compact form when you only care about `Some`
- basic Option methods: `is_some`, `is_none`, `map`, `unwrap_or`, `unwrap_or_else`
- **no `unwrap` / `expect` in production logic**
- negative/empty cases are **normal**, not exceptional

This file is a **spec**, not an answer key.  
Your implementation lives in `rust/foundation/day06/` in your local copy.

---

## 🎯 Super Task — `user_contact` module

Build a small `user_contact` module that models **optional contact info** and forces you to use `Option<T>` instead of fake values.

### Model

```rust
pub struct User {
    pub id: u32,
    pub username: String,
    pub email: Option<String>,
    pub phone: Option<String>,
}
```

### Baseline API

```rust
/// Find a user by id.
pub fn find_user<'a>(users: &'a [User], id: u32) -> Option<&'a User>;

/// Preferred contact (email → phone → None).
pub fn primary_contact<'a>(user: &'a User) -> Option<&'a str>;

/// Label summarizing what contacts exist.
pub fn contact_label(user: &User) -> &'static str;

/// Mask email if present, return None otherwise.
pub fn masked_email(user: &User) -> Option<String>;
```

### Stretch API

```rust
/// First user that has any contact.
pub fn first_reachable<'a>(users: &'a [User]) -> Option<&'a User>;

/// All reachable usernames.
pub fn reachable_usernames(users: &[User]) -> Vec<&str>;
```

### Invariants / Contracts

- No fake sentinel values (`""`, `"none"`, `"n/a"`).
- No `unwrap` / `expect`.
- `contact_label` must handle all 4 combinations:
  - email + phone  
  - email only  
  - phone only  
  - none  
- `masked_email` must never panic, even on short/invalid emails.

**Baseline acceptance**
- ≥5 tests, including ≥3 negative:
  - user not found  
  - user with no contact  
  - masked_email on None  

**Stretch acceptance**
- Both stretch functions implemented + tested.
- Option combinators used where appropriate.

---

## 🧠 MVT — Minimal Viable Theory

1) **What Option really is**
```rust
enum Option<T> { None, Some(T) }
```

2) **Exhaustive match**
```rust
match value {
    Some(v) => { /*...*/ }
    None => { /*...*/ }
}
```

3) **if let**
Use when you only care about Some:
```rust
if let Some(email) = &user.email {
    println!("{email}");
}
```

4) **map / unwrap_or / unwrap_or_else**
Keep control flow clean and avoid `unwrap`.

5) **Anti-patterns**
- empty strings meaning “no contact”
- `.unwrap()` in any public function
- assuming contact always exists

---

## 🔬 Micro Exercises

1) `describe(Option<i32>) -> &str`  
Return `"has value"` / `"empty"`.

2) Count how many elements in `&[Option<i32>]` are `Some`.

3) `squared(Option<i32>) -> Option<i32>` using `.map`.

4) `username_or_guest(Option<String>) -> String` using `unwrap_or_else`.

5) Anti-example:  
Show a function using `unwrap()` on input; rewrite safely.

---

## 🧩 Mini Challenge

```rust
pub fn first_non_empty<'a>(
    primary: Option<&'a str>,
    secondary: Option<&'a str>,
    fallback: Option<&'a str>,
) -> Option<&'a str> {
    // TODO
}
```

Requirements:

- Skip `Some("")`
- Return first real non-empty string
- Add 4 tests:
  - primary present  
  - primary empty, secondary used  
  - fallback only  
  - all empty  

---

## 🗂 Project Layout

```
rust/foundation/day06/
  Cargo.toml
  src/
    main.rs
    user_contact.rs
```

### `user_contact.rs` skeleton

```rust
#[derive(Debug, Clone)]
pub struct User {
    pub id: u32,
    pub username: String,
    pub email: Option<String>,
    pub phone: Option<String>,
}

pub fn find_user<'a>(users: &'a [User], id: u32) -> Option<&'a User> { /* TODO */ }

pub fn primary_contact<'a>(user: &'a User) -> Option<&'a str> { /* TODO */ }

pub fn contact_label(user: &User) -> &'static str { /* TODO */ }

pub fn masked_email(user: &User) -> Option<String> { /* TODO */ }

// Stretch
pub fn first_reachable<'a>(users: &'a [User]) -> Option<&'a User> { /* TODO */ }

pub fn reachable_usernames<'a>(users: &'a [User]) -> Vec<&'a str> { /* TODO */ }

#[cfg(test)]
mod tests {
    // TODO
}
```

---

## ✅ Acceptance Criteria (Baseline)

- No `unwrap` / `expect`
- `Option<T>` instead of fake values
- `match` exhaustive
- `contact_label` covers all cases
- ≥5 tests, ≥3 negative

**Stretch**
- first_reachable + reachable_usernames implemented & tested
- Mini challenge integrated (optional)

---

## 📊 Coverage Matrix

| Topic                       | Where                        | Verified via |
|-----------------------------|------------------------------|--------------|
| Option basics               | all functions                | tests + API  |
| Exhaustive match            | contact_label, primary       | tests        |
| if let                      | helpers, loops               | code review  |
| map / unwrap_or             | masked_email                 | tests        |
| Negative cases              | tests                        | ≥3 required  |
| No unwrap                   | whole module                 | scan/grep    |

---

## 🧱 Senior Checklist

- ❌ No unwrap/expect  
- ✅ Clear Option-based design  
- ✅ Exhaustive match everywhere  
- ✅ Mini challenge done  
- ✅ Test coverage strong, including bad inputs  

---

## 📘 Decision Log (to fill after coding)

- Why Option instead of empty strings?
- Where match improved clarity?
- How you prevented panics?
- What edge cases you supported?
- How you wrote negative tests?

---

## 🔁 Retrospective

1) Explain to someone else why Option is superior to sentinel values.  
2) Where did Option make the code simpler?  
3) Which Option pattern will you now use automatically?



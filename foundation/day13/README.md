# Day 13 — HashMap<K, V>, Indices, and `.entry()`
_Public learning skeleton — implemented in this repo._

Today you switch from “plain Vec” to **associative collections** and proper in-memory indices. The code lives in `src/user.rs` (domain), `src/store.rs` (store + indices), re-exported via `src/lib.rs`, with tests in `tests/user_store.rs`.

> **Foundation plan:**  
> Day 13. HashMap<K, V>  
> Practice: indices by id/email, `.entry()`.  
> Artifact: `src/store.rs` + `src/user.rs` (re-exported via `src/lib.rs`), tests in `tests/user_store.rs`.

---

## 🎯 Focus of the Day

- Understand how `HashMap<K, V>` works: insert, lookup, remove.
- Feel the difference between **“key → value”** and “just a list”.
- Build **primary and secondary indices** (by `id` and by `email`).
- Use `.entry()` without unnecessary clones.
- Accept that `HashMap` is **unordered** — never rely on iteration order.

**Artifact:**  
A `user_store` module (here in `src/store.rs` / `src/user.rs`) with:
- primary index: `id → User`,
- secondary index: `email → id`.

---

## 🎯 0) Super Task (spec only)

**Goal:**  
Build an in-memory `UserStore` that:

- stores users in a primary index `id → User` (e.g. `HashMap<u64, User>`),
- maintains a secondary index `email → id` (e.g. `HashMap<String, u64>`),
- provides fast access both by id and email,
- guarantees invariants: **no duplicate id, no duplicate email**, indices always in sync.

### Domain (conceptual)

- **User**
  - Fields: `id: u64`, `email: String`, `age: u8`, `is_active: bool`.
  - Invariants:
    - `id > 0`,
    - `email` is trimmed, non-empty, contains `'@'`, no spaces,
    - `age` in `0..=120`,
    - new users start with `is_active = true`.

- **UserError** (domain-level):
  - invalid id,
  - invalid email,
  - invalid age.

- **StoreError** (store-level):
  - `DuplicateId(u64)`,
  - `DuplicateEmail(String)`,
  - variant wrapping `UserError` (so you can use `?` in store methods).

### UserStore (structure & API)

- Fields:
  - `users_by_id: HashMap<u64, User>`,
  - `ids_by_email: HashMap<String, u64>`.

- API (baseline, conceptually):

  - `UserStore::new()`  
    Create an empty store.

  - `UserStore::register(id, email, age) -> Result<&User, StoreError>`  
    - validate user data via domain (`User::new`),
    - check duplicate id,
    - check duplicate email (use `.entry()` at least once),
    - on success:
      - insert user into `users_by_id`,
      - insert `email → id` into `ids_by_email`,
      - return a reference to the stored user.

  - `UserStore::get_by_id(id) -> Option<&User>`  
    - read from `users_by_id` only; no panics.

  - `UserStore::get_by_email(email: &str) -> Option<&User>`  
    - normalize (trim) email,
    - look up id in `ids_by_email`,
    - then look up user in `users_by_id`.

  - `UserStore::remove_by_id(id) -> Option<User>`  
    - remove user from `users_by_id`,
    - if found:
      - remove corresponding `email → id` from `ids_by_email`,
      - return the removed `User`,
    - otherwise return `None`.

### Invariants

On every successful registration:

- `users_by_id` contains `id → User`,
- `ids_by_email` contains `email → id`,
- user’s `email` matches the key stored in `ids_by_email`,
- no duplicate ids or emails,
- **no `unwrap` / `expect` / `panic!`** in store logic.

On every successful removal:

- no dangling entries left in `ids_by_email`.

### Acceptance (Baseline)

- At least one use of `.entry()` in `UserStore` (typically for email index).
- Duplicate id → `StoreError::DuplicateId`.
- Duplicate email → `StoreError::DuplicateEmail`.
- `get_by_id` / `get_by_email` never panic, only return `Some` / `None`.
- `remove_by_id` keeps both indices consistent and returns the removed user.

### Stretch (optional)

- `deactivate_by_id(id) -> Result<(), StoreError>`  
  Mark user as inactive by id.

- `all_ids() -> Vec<u64>`  
  Collect all ids from the primary index and return them **sorted ascending** for deterministic output.

---

## 🧠 1) Minimal Theory — HashMap and `.entry()`

**Thesis 1 — HashMap<K, V>: key → value**

- Associative collection with fast access by key.
- Keys implement `Eq + Hash`.
- Core ops: `insert`, `get`, `get_mut`, `remove`.
- `map.get(&key) -> Option<&V>` — safe, no panic.
- `map[&key]` — can panic if key is missing → avoid in production.

> Takeaway: `HashMap` is for fast keyed access; always prefer `get`/`get_mut`/`remove` over direct indexing.

---

**Thesis 2 — Ownership of keys and values**

- `HashMap<K, V>` **owns** both `K` and `V`.
- Inserting a `String` or `User` moves it into the map.
- You must decide what becomes owned by the store (e.g. emails, users).

> Takeaway: the store is the owner of users and emails; design your API around that.

---

**Thesis 3 — `get`, `get_mut`, `remove`**

- `get(&key) -> Option<&V>` — read.
- `get_mut(&key) -> Option<&mut V>` — in-place update.
- `remove(&key) -> Option<V>` — remove and return the value.

> Takeaway: these three are your **safe primitives**; direct indexing is an anti-pattern in store logic.

---

**Thesis 4 — `.entry()`: atomic check-then-insert**

- `.entry(key)` gives:
  - `Occupied(entry)` if key exists,
  - `Vacant(entry)` if not.
- `.or_insert(default)`:
  - inserts `default` only if key is missing,
  - returns `&mut V` in both cases.

Typical use cases:

- counters,
- idempotent inserts (“insert only if missing”),
- eliminating “check then insert” duplication.

> Takeaway: `.entry()` is the standard way to **check and insert in one place**.

---

**Thesis 5 — HashMap is unordered**

- Iteration order is **not guaranteed**.
- It can change across runs and Rust versions.
- For deterministic order, either:
  - use `BTreeMap`, or
  - gather items (e.g. keys) into a `Vec`, sort, then iterate.

> Takeaway: use `HashMap` for speed, not ordering.

---

## 🔬 2) Micro Tasks (spec only)

All micro tasks are specs; you implement them separately in Rust.

1. **Basic HashMap + safe get**  
   Function that:
   - takes `HashMap<u64, String>` and an `id`,
   - returns `Option<&str>` using `get` and `&String -> &str` without clones,
   - no panics, no `unwrap`.

2. **Counter using `.entry()`**  
   - Character frequency for `&str`,
   - `HashMap<char, u32>`,
   - use `.entry(ch).or_insert(0)` pattern.

3. **Remove with feedback**  
   - Helper that removes from `HashMap<u64, String>` by id,
   - returns `bool` (whether something was removed),
   - uses `remove`, no indexing.

4. **Replace indexing with safe access**  
   - Given a function that uses `map[&id]`,
   - redesign it to:
     - never panic on missing keys,
     - return `Option<&str>`,
     - use `get` internally.

5. **Combine Vec and HashMap**  
   - Function that takes `&[User]`,
   - builds index `id → &User` in a `HashMap<u64, &User>`,
   - warm-up for primary index design.

---

## 🧩 3) Mini Task — `IdEmailIndex`

Mini-module **IdEmailIndex**: a small bidirectional index `id ↔ email`.

### Structure

- `IndexError`:
  - `DuplicateId(u64)`,
  - `DuplicateEmail(String)`.

- `IdEmailIndex`:
  - `by_id: HashMap<u64, String>`,
  - `by_email: HashMap<String, u64>`.

### Operations

- `new()` → empty index.

- `insert(id, email) -> Result<(), IndexError>`:
  - if `id` already in `by_id` → `DuplicateId`,
  - if `email` already in `by_email` → `DuplicateEmail`,
  - otherwise insert into both maps.

- `email_of(id) -> Option<&str>`  
- `id_of(email: &str) -> Option<u64>`

### Requirements

- Both directions stay consistent.
- No `unwrap` / `expect` / `panic!`.
- Tests:
  - successful insert + both lookups,
  - duplicate id,
  - duplicate email,
  - lookups for missing ids/emails.

This is a simplified version of `UserStore`.

---

## 🚀 4) Super Task — `UserStore` with Indices

File: `src/store.rs` (store) and `src/user.rs` (domain).  
Module: `user_store` re-exported via `src/lib.rs`.

### 4.1 Domain

- `User` and `UserError` — same invariants as Day 9–10 (id/email/age, `is_active`).
- `StoreError`:
  - `DuplicateId(u64)`,
  - `DuplicateEmail(String)`,
  - variant wrapping `UserError` (for `?`).

### 4.2 Store and indices

- `UserStore`:
  - `users_by_id: HashMap<u64, User>`,
  - `ids_by_email: HashMap<String, u64>`.

**Key methods:**

- `new()`
- `register(id, email, age) -> Result<&User, StoreError>`
- `get_by_id(id) -> Option<&User>`
- `get_by_email(email: &str) -> Option<&User>`
- `remove_by_id(id) -> Option<User>`

**Stretch:**

- `deactivate_by_id(id) -> Result<(), StoreError>`
- `all_ids() -> Vec<u64>` (sorted ascending).

### Constraints

- At least one conscious use of `.entry()` (typically email index).
- No `unwrap` / `expect` / `panic!` in store logic.
- After each `register`:
  - indices are consistent.
- After `remove_by_id`:
  - no “ghost” entries left.

### Test Suite (conceptual)

Minimal:

- `register_happy_path`
- `register_duplicate_id`
- `register_duplicate_email`
- `get_by_email_trims_input`
- `remove_by_id_removes_from_both_indexes`
- `register_invalid_user_propagates_error`

---

## 🧾 5) End of Day 13

### Coverage Matrix

| Subtopic                     | Where              | How it’s verified                     |
|-----------------------------|--------------------|---------------------------------------|
| HashMap basics              | UserStore, mini idx| insert / get / remove tests           |
| `.entry()` usage            | UserStore::register| duplicate handling, code review       |
| Primary/secondary indices   | `users_by_id`, `ids_by_email` | tests for register/get/remove |
| Option / Result APIs        | all store methods  | happy + negative tests                |
| No panics / indexing        | entire store       | only `get` / `entry` / `remove`       |
| ≥ 3 negative cases          | tests              | duplicates, invalid data, missing ids |

### Senior Checklist (Day 13)

- ❌ No `unwrap` / `expect` / `panic!` in `UserStore` logic.  
- ✅ Names are domain-specific (`UserStore`, `users_by_id`, `ids_by_email`).  
- ✅ `.entry()` is used intentionally, not “just because”.  
- ✅ Indices stay consistent after register/remove.  
- ✅ Negative tests cover duplicates, invalid users, and missing keys.

---

### Decision Log (your notes)

Examples:

- Why you chose **two maps** (id → User, email → id) instead of one.
- Where exactly `.entry()` simplified logic vs `contains_key`.
- Which `User` fields are validated and why.
- How you ensured `remove_by_id` keeps both maps in sync.
- Which stretch functions you implemented or skipped (and why).

---

### Retrospective

- Can you explain why `HashMap` is better than `Vec` for lookups by id/email?  
- Do you understand what `.entry()` buys you over manual `if contains_key { ... } else { insert }`?  
- Can you sketch a similar primary + secondary index for another domain (e.g. `ProductStore` with id + SKU)?

---

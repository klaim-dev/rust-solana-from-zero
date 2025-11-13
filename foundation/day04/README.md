# Day 4 — Ownership: move, clone, Copy

Public learning **skeleton** for Day 4 — no solutions.  
Theme: Rust ownership done right — `move` / `clone` / `Copy`. We work like in production: Super Task → MVT → Micro → Mini → Super → Wrap‑up.

Deliverable: `rust/foundation/day04/` + an **Ownership Report** table.

---

## Focus of the Day
- Behavior of **move** (for non‑`Copy` types like `String`).
- When and why to **clone** (explicit, minimal, justified).
- What **`Copy`** means (cheap bitwise copy of simple values).
- Avoiding extra clones with `&T` / `&mut T` and returning references.
- Safe update patterns (find‑by‑id with `&mut`, no double‑borrowing).

> No `Rc/Arc` and no manual lifetimes today—only basics inferred by the compiler.

---

## 🎯 Super Task (contract only; no solutions)

Build a small in‑memory catalog **`Bag`** that demonstrates clean ownership boundaries.

### Model (minimum)

```rust
pub struct Item {
    pub id: u32,      // Copy
    pub name: String, // move/clone
}

pub struct Bag {
    items: Vec<Item>,
}
```

### API (free functions or `impl Bag` — both OK for Day 4)

**Baseline**

```rust
// Add an item without forcing the caller to clone.
pub fn add_item(bag: &mut Bag, id: u32, name: &str) -> bool;

// Return a borrowed &str to the name if found by id (no allocation).
pub fn get_name<'a>(bag: &'a Bag, id: u32) -> Option<&'a str>;

// Rename by id without unnecessary moves.
pub fn rename_item(bag: &mut Bag, id: u32, new_name: &str) -> bool;

// Return list of ids — demonstrates Copy.
pub fn id_list(bag: &Bag) -> Vec<u32>;
```

**Stretch**

```rust
// Consume the bag and return owned names (demonstrates move-out).
pub fn into_names(bag: Bag) -> Vec<String>;

// Return an owned name for a single id (explicit clone when requested).
pub fn get_name_owned(bag: &Bag, id: u32) -> Option<String>;

// Swap two item names (practice &mut without violating borrowing rules).
pub fn swap_names(bag: &mut Bag, id1: u32, id2: u32) -> bool;
```

**Invariants / Contract**
- `add_item` prevents duplicate `id` → returns `false` if `id` exists.
- **No `unwrap/expect`** in production paths.
- `get_name` returns a **reference**; `get_name_owned` returns **ownership** (may clone).
- `id_list` copies only `u32` (cheap `Copy`), no `String` allocations.
- `swap_names` respects borrowing rules (solve via indices / disjoint borrows).

**Acceptance (Baseline)**
- Add, read‑by‑reference, rename, id listing — implemented and **tested**.
- **No unjustified `.clone()`** in hot paths (allocations only where the API intends).

**Stretch Acceptance**
- `into_names` consumes `Bag` and returns `Vec<String>` (move).
- `get_name_owned` clones **only** when requested.
- `swap_names` implemented safely (no aliasing issues).

---

## 🧠 MVT — Minimal Viable Theory

1) **Move**  
Passing/assigning a non‑`Copy` value (e.g., `String`) **moves** ownership. Typical sites: returning from a function, storing into a collection/field. Anti‑pattern: *“use after move”*.

2) **Clone**  
Often heap work → **explicit & costly**; keep it visible and minimal. Use when duplicated ownership is truly required. Anti‑pattern: slapping `.clone()` to “fix” borrow errors.

3) **Copy**  
`u32`, `usize`, `bool`, `char`, `f64`, etc. are `Copy`. If *all* fields are `Copy`, a struct *can* be `Copy` (not deriving it today). Consequence: id lists are cheap to duplicate.

4) **Borrowing vs Clone**  
Prefer `&T` for reads; `&mut T` for targeted mutation. Avoid two simultaneous `&mut` to the same element — use indices / `split_at_mut` patterns.

5) **Return by ref vs by value**  
`&str` from internal `String` = cheap getter (lifetime tied to `&self`/`&Bag`). Returning `String` means move (owning) or **explicit** clone.

6) **Patterns to avoid extra clones**  
Accept `&str` and allocate **inside** when storing; return `&str` for read‑only exposure; rely on `Copy` for id lists.

---

## 🔬 Micro Exercises (no solutions)

1) **Move trap**  
Take `String` by value; try using the original after the call — keep the compiler error as a commented anti‑example (“expected error”).

2) **Borrow instead of clone**  
```rust
fn first_char_ref(s: &String) -> Option<char>
```
Borrow the input; don’t take ownership.

3) **Copy practice**  
From `&[u32]` build `Vec<u32>` — explain in a comment why this is cheap `Copy`, not a clone of heap data.

4) **get_mut update**  
Find item by `id` and update via `&mut` **without** intermediate `.clone()`.

5) **Double `&mut` anti‑example**  
Try mutating two items in the same vector via two `&mut` without splitting — explain the borrow checker error; sketch a fix with indices / disjoint slices.

---

## 🧩 Mini Challenge

One focused pipeline: **“accept by ref → store ownership → read without clones”**.

```rust
/// Build Vec<String> from &[&str] so the caller does **no** cloning.
pub fn collect_names(input: &[&str]) -> Vec<String>;

/// Return a **view** of names without allocating new strings.
pub fn names_view<'a>(names: &'a [String]) -> Vec<&'a str>;
```

**Criteria**
- `collect_names(&["Alice","Bob"])` allocates **inside**; the caller passes `&str`.
- `names_view` returns `&str` slices only (no new `String`).

Add 3–4 asserts including an empty input case.

---

## 🚀 Super Task (90 min, production style)

**Project layout**

```text
rust/foundation/day04/
  src/
    main.rs
    bag.rs
  Cargo.toml
  OWNERSHIP_REPORT.md   // ownership/move/clone/Copy table
```

### `bag.rs` (public skeleton)

```rust
pub struct Item { pub id: u32, pub name: String }
pub struct Bag  { items: Vec<Item> }

// baseline
pub fn add_item(bag: &mut Bag, id: u32, name: &str) -> bool;
pub fn get_name<'a>(bag: &'a Bag, id: u32) -> Option<&'a str>;
pub fn rename_item(bag: &mut Bag, id: u32, new_name: &str) -> bool;
pub fn id_list(bag: &Bag) -> Vec<u32>;

// stretch
pub fn into_names(bag: Bag) -> Vec<String>;
pub fn get_name_owned(bag: &Bag, id: u32) -> Option<String>;
pub fn swap_names(bag: &mut Bag, id1: u32, id2: u32) -> bool;

#[cfg(test)]
mod tests {
    use super::*;

    // At least 5 tests:
    // - add_item: happy + duplicate id (false)
    // - get_name: present/absent
    // - rename_item: present/absent (+ policy for empty new_name?)
    // - id_list: content/order
    // - stretch: into_names consumes bag; swap_names success/failure
}
```

### `main.rs` (demo only)
- Create a `Bag`, add several `Item`s.
- Show `get_name` and `id_list`.
- Rename, show again.
- (Stretch) Demonstrate `into_names(bag)` and note that `bag` is moved and unusable afterward.

**Acceptance (Baseline)**
- Tests are green, ≥5 total with **≥3 negative cases**:
  - duplicate `id`,
  - unknown `id` for read/rename,
  - (your policy) empty `new_name` → reject or normalize.
- No `unwrap/expect`.
- **No stray `.clone()`**—clones appear only in APIs that promise ownership.

**Stretch**
- `swap_names` implemented safely (`split_at_mut`/indices; no aliasing).
- `into_names` tested for move behavior.
- Ownership report present.

---

## 🧾 Wrap‑Up

### `OWNERSHIP_REPORT.md` (table)

| Location                   | Data Type  | Technique | Why                                                |
|---------------------------|------------|-----------|----------------------------------------------------|
| add_item(name: &str)      | String     | move‑in   | create `String` locally and store into `Item`      |
| get_name(&Bag) -> &str    | &str       | borrow    | read‑only view, zero allocations                   |
| rename_item(&mut Bag)     | String     | move‑in   | build new `String`, replace in place               |
| id_list(&Bag) -> Vec<u32> | u32 (Copy) | Copy      | cheap duplication of identifiers                   |
| get_name_owned(&Bag)      | String     | clone     | explicit ownership by request only                 |
| into_names(Bag)           | Vec<String>| move      | consume bag and move out owned names               |
| swap_names(&mut Bag)      | String     | &mut      | swap without extra clones                          |

### Coverage Matrix

| Topic            | Where                               | How Verified                               |
|------------------|-------------------------------------|--------------------------------------------|
| move             | `into_names`                        | compilation + observed behavior            |
| clone            | `get_name_owned`                    | explicit call + contract test              |
| Copy             | `id_list`                           | signatures / `u32` type, tests             |
| &T / &mut T      | `get_name`, `rename_item`, `swap…`  | read/write tests                           |
| No extra clones  | whole module                        | review + grep for `.clone()`               |
| Negative cases   | tests                               | ≥3: duplicate id, unknown id, empty policy |

### Senior Checklist
- ❌ No `unwrap/expect` in production paths.
- ✅ Clones only where **ownership** is the contract.
- ✅ `Copy` used instead of unnecessary clones (`u32` ids).
- ✅ Clear boundaries (logic in `bag.rs`, thin `main`).
- ✅ ≥5 tests, ≥3 of them negative.

### Decision Log (8–10 lines)
- Where you almost used `.clone()` and how a reference solved it.
- Why `id: u32` (`Copy`) simplified the API.
- Your policy for empty names and why.
- How you avoided double `&mut` in `swap_names`.
- Where clone is truly justified.
- What you’d move to shared helpers.
- What to improve in iteration 2 (e.g., indexing instead of linear scan).

### Retrospective (3 prompts)
1. Where did you **avoid `.clone()` on purpose** and what replaced it?  
2. Can you explain **move vs clone vs Copy** in 30s using `id_list` and `get_name_owned`?  
3. Which ownership pattern will you write **automatically** tomorrow?

# Day 5 — Borrowing: `&T` and `&mut T`

Public learning **skeleton** for Day 5 — no solutions.  
Theme: precise, production‑grade borrowing — when to read (`&T`), when to mutate (`&mut T`), how to avoid double‑`&mut`, and how to return references safely.

Deliverable: `rust/foundation/day05/` + a **Borrowing Map** table (`BORROWING_MAP.md`).

---

## Focus of the Day
- Choosing `&T` vs `&mut T` intentionally.
- Reborrowing: `&mut T` → short‑lived `&T` (but not the other way).
- Safe getters: `get` / `get_mut` on slices/Vec.
- **No** returning references to temporaries (no dangling).
- Avoiding “two `&mut` to the same memory”: indices / `split_at_mut` / disjoint regions.
- Mutating structures without unnecessary `clone` allocations.

> No interior mutability (`RefCell`, `Cell`) and no manual lifetimes notation beyond what the compiler infers today.

---

## 🎯 Super Task (contract only; no solutions)

Extend the in‑memory catalog: **`Bag` → `Inventory`** with read/write by references.

### Model
```rust
pub struct Item {
    pub id: u32,      // Copy
    pub name: String, // owned
    pub qty: u32,     // Copy
}

pub struct Inventory {
    items: Vec<Item>,
}
```

### Baseline API (either free fns or `impl Inventory` — fine for Day 5)
```rust
// Read by reference, no allocations.
pub fn get_item<'a>(inv: &'a Inventory, id: u32) -> Option<&'a Item>;

// Mutably borrow a single item for update.
pub fn get_item_mut<'a>(inv: &'a mut Inventory, id: u32) -> Option<&'a mut Item>;

// Increase quantity (if item not found — do nothing).
pub fn add_stock(inv: &mut Inventory, id: u32, delta: u32) -> bool;

// Safe rename (document the policy for empty names in this README).
pub fn rename(inv: &mut Inventory, id: u32, new_name: &str) -> bool;

// Bulk operation: increase qty for a set of ids (without double &mut).
pub fn bulk_add_stock(inv: &mut Inventory, ids: &[u32], delta: u32) -> usize;
```

### Stretch
```rust
// Borrow two *distinct* items at once (no UB).
pub fn get_two_mut<'a>(
    inv: &'a mut Inventory,
    id1: u32,
    id2: u32,
) -> Option<(&'a mut Item, &'a mut Item)>;

// Split inventory into two disjoint mutable slices (demo `split_at_mut`).
pub fn split_by_index<'a>(inv: &'a mut Inventory, mid: usize)
    -> (&'a mut [Item], &'a mut [Item]);

// Read‑only view of names (no string allocations).
pub fn names_view<'a>(inv: &'a Inventory) -> Vec<&'a str>;
```

### Invariants / Contracts
- No `unwrap` / `expect` in production paths.
- `get_*` never copy strings; `rename` allocates only as required for the new name.
- `bulk_add_stock` does not violate borrowing rules (no two `&mut` to the same element).
- `get_two_mut` returns `None` if `id1 == id2` or if one/both items are missing.

**Acceptance (Baseline)**
- `get_item` / `get_item_mut` are lifetime‑correct.
- `add_stock` / `rename` update exactly one found element.
- `bulk_add_stock` safely handles duplicates in `ids` and returns the number of updated items.
- Tests: ≥ 5 total, **≥ 3 negative** (missing id, empty rename policy, duplicate ids in bulk).

**Stretch Acceptance**
- `get_two_mut` proven safe (via indices / `split_at_mut`).
- `split_by_index` demonstrates disjoint mutable regions.
- `names_view` returns borrowed `&str` only.

---

## 🧠 MVT — Minimal Viable Theory

1) **`&T` vs `&mut T`**  
Either many readers (`&T`) or one unique writer (`&mut T`) per region at a time.

2) **Reborrowing**  
From `&mut T` you can create a shorter‑lived `&T`. The reverse is not legal.

3) **Dangling (don’t)**  
Never return a reference to a temporary or to data that won’t outlive the reference.

4) **Collections & borrowing**  
You can’t hold references to elements and mutate the collection’s structure (push/insert/remove) at the same time.

5) **`get` / `get_mut`**  
Prefer safe indexing that returns `Option`, not panicking indexing `[]`.

6) **Mutations & iteration**  
Keep mutation separate from ranges where references are alive; design control flow for borrow‑checker clarity.

---

## 🔬 Micro Exercises (no solutions)

1) **Reborrow for read**  
Accept `&mut String`, read its length via a temporary `&str`, then append a suffix.

2) **`get_mut` update**  
From `&mut Vec<u32>`, call `get_mut(i)` and increment if present; handle out‑of‑bounds without panic.

3) **No push with live ref (anti‑example)**  
Hold a reference to an element, then try `push`; keep the compiler error as a commented example.

4) **Two writes without conflict**  
Update elements at `i` and `j` using sorted indices + `split_at_mut`.

5) **Dangling vs safe return**  
Show a function that *would* return a reference to a temporary (commented out) and a correct alternative returning `String` or a field reference.

---

## 🧩 Mini Challenge

A focused pipeline: **“accept by reference → store ownership → read by reference, no clones.”**

```rust
/// Build Vec<String> from &[&str] so the caller does **no** cloning.
pub fn collect_names(input: &[&str]) -> Vec<String>;

/// Return a read‑only view of names with zero allocations.
pub fn names_view<'a>(names: &'a [String]) -> Vec<&'a str>;
```

**Criteria**
- `collect_names(&["Alice","Bob"])` allocates **inside**; the caller passes `&str`.
- `names_view` returns `&str` slices only (no new `String`).

Include 3–4 asserts, including the empty input case.

---

## 🚀 Super Task (90 min, production style)

**Project layout**
```text
rust/foundation/day05/
  src/
    main.rs
    inventory.rs
  Cargo.toml
  BORROWING_MAP.md
```

### `inventory.rs` (public skeleton — contracts only)
```rust
pub struct Item { pub id: u32, pub name: String, pub qty: u32 }
pub struct Inventory { items: Vec<Item> }

// baseline
pub fn get_item<'a>(inv: &'a Inventory, id: u32) -> Option<&'a Item>;
pub fn get_item_mut<'a>(inv: &'a mut Inventory, id: u32) -> Option<&'a mut Item>;
pub fn add_stock(inv: &mut Inventory, id: u32, delta: u32) -> bool;
pub fn rename(inv: &mut Inventory, id: u32, new_name: &str) -> bool;
pub fn bulk_add_stock(inv: &mut Inventory, ids: &[u32], delta: u32) -> usize;

// stretch
pub fn get_two_mut<'a>(inv: &'a mut Inventory, id1: u32, id2: u32)
    -> Option<(&'a mut Item, &'a mut Item)>;

pub fn split_by_index<'a>(inv: &'a mut Inventory, mid: usize)
    -> (&'a mut [Item], &'a mut [Item]);

pub fn names_view<'a>(inv: &'a Inventory) -> Vec<&'a str>;

#[cfg(test)]
mod tests {
    use super::*;
    // ≥7 tests with ≥3 negatives:
    // - get_item/get_item_mut for missing id
    // - rename with empty name (according to your policy)
    // - get_two_mut with same id or missing ids
    // Positives:
    // - add_stock, bulk_add_stock (including duplicate ids)
    // - split_by_index yields disjoint parts
    // - names_view returns borrowed views only
}
```

### `main.rs` (demo only)
- Initialize `Inventory` with 3–4 items.
- Show reading via `get_item`, updating via `get_item_mut` / `add_stock`.
- Demonstrate `bulk_add_stock` with duplicates in `ids`.
- (Stretch) `get_two_mut` + `core::mem::swap`; `split_by_index` to show two independent regions.

**Acceptance (Baseline)**
- Compiles and tests are green (≥7).
- No `unwrap` / `expect` in production paths.
- No double‑`&mut` conflicts — all resolved via indices / `split_at_mut`.
- No references outliving their data; no references to temporaries.

**Stretch**
- Dual mutable borrows obtained safely (via indices/split).
- Standalone **Borrowing Map** in `.md` is present and accurate.

---

## 🧾 Wrap‑Up

### `BORROWING_MAP.md` (table)
| Location                 | Ref kind        | Why                                  | Alternative                 |
|-------------------------|-----------------|--------------------------------------|-----------------------------|
| get_item                | `&Item`         | read‑only view, zero allocations     | `Option<Item>` (costly)     |
| get_item_mut            | `&mut Item`     | in‑place update without clones       | replace whole `Item`        |
| rename                  | `&mut` + `&str` | update name in place                 | return `String`             |
| add_stock / bulk_add    | `&mut`          | batch safe updates                   | iterate over copies         |
| get_two_mut             | two `&mut`      | two disjoint regions                 | two passes over the vector  |
| split_by_index          | `&mut [ ]`×2    | excludes overlap by construction     | clone `Vec<Item>`           |
| names_view              | `Vec<&str>`     | zero‑alloc read‑only projection      | `Vec<String>` (more memory) |

### Coverage Matrix
| Subtopic                   | Where                      | How verified                           |
|---------------------------|----------------------------|----------------------------------------|
| `&` vs `&mut`             | all API                    | read/write tests                       |
| Reborrowing               | local read‑then‑write      | compile + code review                  |
| No dangling               | all returns                | compiler + visual audit                |
| `get` / `get_mut`         | element access             | negative cases (missing id)            |
| Avoid double `&mut`       | bulk / get_two_mut / split | tests for id1==id2 / overlapping ids   |
| No unnecessary clones     | whole module               | review + grep for `.clone()`           |
| ≥3 negative tests         | tests                      | missing id, empty name, duplicate ids  |

### Senior Checklist
- ❌ No `unwrap` / `expect` in production logic.
- ✅ References never outlive their data.
- ✅ Conflicting `&mut` resolved via indices / `split_at_mut`.
- ✅ No gratuitous `clone`; rely on borrowing.
- ✅ Tests ≥ 7 with ≥ 3 negatives.

### Decision Log (8–10 lines)
- Where a `.clone()` was tempting and how borrowing avoided it.
- Your policy for empty names and how it’s tested.
- How you resolved two mutable borrows (indices/split).
- Where reborrowing simplified code.
- Why `names_view` yields `&str` (not `String`).
- What you'd extract into shared search helpers.
- Plan for iteration 2: secondary index `HashMap<id, idx>` for O(1).

### Retrospective (3 prompts)
1) Where did the borrow‑checker fail first, and how did you fix it properly?  
2) Can you explain in 30s **why** `split_at_mut` is allowed while two `&mut` to overlapping memory are not?  
3) Which `&/&mut` pattern will you start using **automatically** tomorrow?

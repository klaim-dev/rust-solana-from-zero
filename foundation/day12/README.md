# **Day 12 — `Vec<T>`, Slices & Basic Iterators**

*Public learning skeleton — no solutions.*

Today you move from “objects” (structs, errors) to **data pipelines**.
This is the foundation for all later collection-heavy work (Days 13–14, 17, 20, backend SQL rows, etc.).

> **Foundation Plan:**
> Day 12 — Vec<T> and iterators (basics): filtering, mapping, slices.
> Artifact: `day12_vec_iter.rs`.

---

## 🎯 **Focus of the Day**

* Learn the mechanics of **`Vec<T>`** (len, capacity, push/pop).
* Understand **slices** (`&[T]` / `&mut [T]`) and why APIs prefer them.
* Avoid panics: safe access via `.get()` and slice ranges via `.get(start..end)`.
* Drill basic iterator tools: `.iter()`, `.iter_mut()`, `.into_iter()`, `map`, `filter`, `collect`.
* Build a practical module: **`grades`** (scores 0–100) with pure data logic and tests.

**Artifact:**
`day12_vec_iter.rs` with module `grades` + tests.

---

## 🎯 **0) Super Task (spec only)**

Build module `grades` with safe, iterator-based operations:

**Baseline API**

* `curve_scores(scores: &[u32], bonus: i32) -> Vec<u32>`
  Clamp to **0–100**, use `iter() + map + collect`.

* `passing(scores: &[u32]) -> Vec<u32>`
  Keep only `>= 60`.

* `first_failing(scores: &[u32]) -> Option<u32>`
  Use `iter().find()` or a simple loop.

* `window(scores: &[u32], start: usize, len: usize) -> Option<&[u32]>`
  Use `.get(start..end)` (never raw `scores[start..end]`).

* `clamp_in_place(scores: &mut [u32])`
  Use `iter_mut()` or index loop; no panics.

**Stretch**

* `average(scores: &[u32]) -> Option<f64>`
* `top_n(scores: &[u32], n: usize) -> Vec<u32>` (descending, non-mutating).

**Invariants**

* No `unwrap`, `expect`, `panic!` in production code.
* All functions use **slices** for input.
* No panics on bad indices.
* Iterator pipeline is the default way to transform data.

**Acceptance**

* ≥ 6 tests, including negative: empty, OOB window, negative bonus, >100 clamping.

---

## 🧠 **1) Minimal Theory — Vec / Slices / Iterators**

### **Vec<T>**

* Growable heap array: stack header `{ptr, len, cap}` + data on the heap.
* `push` may trigger reallocation; `len != capacity`.

### **Slices: `&[T]` / `&mut [T]`**

* Read-only / mutable views.
* Functions should prefer slices for flexibility (they accept Vec, arrays, or other slices automatically).

### **Safe access**

* `xs[idx]` panics → avoid in production.
* `xs.get(idx)` → `Option<&T>` (safe).
* `xs.get(start..end)` → safe slice window.

### **Iterators**

* `.iter()` → `&T`
* `.iter_mut()` → `&mut T`
* `.into_iter()` → owns `T`
* Use them for mapping, filtering, searching.

### **map / filter / collect**

Core pattern for Day 12:

```
&[T] → iter → map/filter → collect::<Vec<_>>()
```

---

## 🔬 **2) Micro Tasks (3–5 tasks)**

1. **push/pop without panics**
   Implement safe pop with fallback (no unwrap).

2. **Return whole slice**
   `slice_full(&Vec<T>) -> &[T]` using auto-coercion.

3. **Safe element access**
   `safe_get(&[i32], idx)` using `.get().copied()`.

4. **double_all**
   Use `iter + map + collect`.

5. **only_positive**
   Use `filter + collect`.

---

## 🧩 **3) Mini Task — numbers module**

`numbers` module (4–5 small functions):

* `positives(&[i32]) -> Vec<i32>`
* `sum(&[i32]) -> i32` (`iter().sum()` or fold)
* `first_positive(&[i32]) -> Option<i32>`

**Tests:** empty list, all negative, mixed input.

---

## 🚀 **4) Super Task — `day12_vec_iter.rs`**

Single file with module `grades`.
Implement only with iterators, slices, and safe access.
All behavior driven by tests.
No unwrap/expect/panic anywhere.

Tests should cover:

* bonus application (positive & negative)
* clamping >100
* failing/passing logic
* windows (valid & invalid)
* clamp_in_place
* optional: average, top_n

---

## 📊 **Coverage Matrix**

| Topic                  | Where                     | Verified by              |
| ---------------------- | ------------------------- | ------------------------ |
| Vec operations         | micro tasks, curve_scores | push/pop, basic behavior |
| Slices                 | function signatures       | compilation, no copies   |
| Safe access            | window                    | tests for OOB            |
| Iterators              | all functions             | map/filter/find/iter_mut |
| map/filter/collect     | curve_scores, passing     | tests                    |
| Option for empty cases | first_failing, average    | tests                    |
| No unwrap/expect       | entire module             | review                   |
| Negative cases         | tests                     | OOB, empty, clamping     |

---

## 🧱 **Senior Checklist**

* ❌ No unwrap/expect/panic in production code
* ✅ APIs use slices (`&[u32]`, `&mut [u32]`)
* ✅ Iterators used intentionally
* ✅ All boundary cases handled without panics
* ✅ ≥ 3 negative tests

---

## 📘 **Decision Log (your notes)**

Examples:

* Why `&[u32]` API is better than taking `Vec<u32>`.
* Why window returns `Option<&[u32]>`.
* Where iterators made the code cleaner.
* What confused you about slices initially and how you fixed it.

---

## 🔁 **Retrospective**

* Can you explain the difference between Vec/`&[T]`/`&mut [T]`?
* Can you build a slice safely with `.get(start..end)`?
* Can you write from memory the iterator pipeline `iter → map → filter → collect`?

---

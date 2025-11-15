# Day 3 — Functions, Parameters, Tuples, `return`

Public learning **skeleton** for Day 3 — no solutions.  
Focus: clean function design, passing by value vs by reference, multiple-return via tuples, and **unit tests required starting today**.

---

## Overview

- declare & call functions
- pass by value vs `&T` (borrowing)
- return values (including tuples)
- early `return` vs last-expression return
- keep `main` thin; move logic into modules
- **unit tests are mandatory from Day 3**

This day builds on Day 1–2: `String`/`&str`, careful arithmetic, `if/else`, and a domain/utils split.

---

## Super Task

Build a `text_utils` module for **normalizing and analyzing strings**.

**Constraints**
- All functions are **pure** (no I/O inside).
- Signatures are precise (`&str` in; `String`/numbers/tuples out).
- Use tuples for multi-value returns.
- Provide **≥ 3 unit tests** inside the module.

### Baseline API (signatures, no bodies)

```rust
/// Trim edges and collapse internal runs of whitespace into a single space.
pub fn normalize_whitespace(input: &str) -> String;

/// Count words after normalization.
pub fn count_words(input: &str) -> usize;

/// Summarize a string:
/// (char_count, word_count, is_empty_like)
/// `is_empty_like` is true when the trimmed string is empty.
pub fn summarize(input: &str) -> (usize, usize, bool);
```

### Stretch

```rust
/// Build a preview string:
/// - normalize whitespace
/// - if longer than `max_len` chars, cut on char boundary and append "..."
pub fn make_preview(input: &str, max_len: usize) -> String;
```

**Invariants**
- No `unwrap` / `expect` in production logic.
- No panics from invalid slicing (be Unicode-safe when cutting).
- No I/O inside utilities.
- Deterministic results, covered by tests.

---

## Minimal Viable Theory (MVT)

### 1) Function declarations
- `fn name(params) -> Type { ... }`
- One function = one clear purpose.
- Avoid vague names like `do_it` — be specific.

### 2) Value vs reference
- Small `Copy` types (`i32`, `bool`, `usize`) → pass by value.
- Owned or large types (`String`, `Vec<T>`, domain structs) → prefer `&T` for read-only.
- Today: accept inputs as `&str`; return `String` when you build new text.

### 3) Returns & `return`
- Rust returns the **last expression** without a trailing `;`.
- Use explicit `return` only for early exit.

```rust
fn clamp_to_zero(x: i32) -> i32 {
    if x < 0 { return 0; }
    x // implicit return
}
```

### 4) Tuples for multiple return
- Lightweight grouping `(T1, T2, T3, ...)`.
- Destructure at call sites:

```rust
let (chars, words, empty_like) = summarize(s);
```

### 5) Pure helpers, thin `main`
- No printing inside logic.
- Keep utilities reusable and testable; `main` just wires them together.

### 6) Unicode awareness (lightweight today)
- `String::len()` is bytes; `.chars().count()` is Unicode scalar values.
- Avoid slicing by byte index for user-facing text.
- If you implement `make_preview`, iterate over `chars()` to cut safely.

---

## Micro Exercises (no solutions)

1. **Basic add**
   ```rust
   fn add(a: i32, b: i32) -> i32
   ```
   Use `assert_eq!(add(2, 3), 5);` in a quick self-check.

2. **Borrowed string**
   ```rust
   fn shout(s: &str) -> String  // returns "UPPERCASE!"
   ```
   Accept `&str`, not `String`.

3. **Tuple return**
   ```rust
   fn min_max(a: i32, b: i32) -> (i32, i32) // (min, max)
   ```
   Destructure at call site.

4. **Return style**
   Rewrite a tiny function both with explicit `return` and as a last-expression return. Which reads better?

5. **Extract predicate**
   From Day 2, lift a compound condition into:
   ```rust
   fn is_big_order(total_cents: u32) -> bool;
   ```
   Replace inline checks with this helper.

---

## Mini Challenge

Implement a tiny analysis helper:

```rust
/// Returns:
/// - number of Unicode scalar values: `.chars().count()`
/// - number of “words”: split by spaces after trimming/collapsing
pub fn basic_stats(input: &str) -> (usize, usize) {
    // TODO
}
```

**Requirements**
- Input is `&str`.
- `len_chars` via `.chars().count()`.
- `words` via splitting after normalization.
- Add **≥ 3 assertions**:
  - empty string,
  - `"hello world"`,
  - extra spaces & tabs.

This prepares the Super Task.

---

## Project Layout (suggested)

```text
rust/foundation/day03/
  Cargo.toml
  src/
    main.rs
    text_utils.rs
```

### `text_utils.rs` (public skeleton)

```rust
/// Trim + collapse internal whitespace to a single space.
pub fn normalize_whitespace(input: &str) -> String {
    // TODO
}

/// Word count after normalization.
pub fn count_words(input: &str) -> usize {
    // TODO
}

/// (char_count, word_count, is_empty_like)
pub fn summarize(input: &str) -> (usize, usize, bool) {
    // TODO
}

/// Stretch: char-safe preview with ellipsis when truncated.
pub fn make_preview(input: &str, max_len: usize) -> String {
    // TODO
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_basic() {
        // TODO: assert_eq!(normalize_whitespace("  a   b  "), "a b");
    }

    #[test]
    fn count_words_basic() {
        // TODO: assert_eq!(count_words("  alpha   beta  "), 2);
    }

    #[test]
    fn summarize_empty_like() {
        // TODO: let (chars, words, empty) = summarize("   ");
        // TODO: assert_eq!((chars, words, empty), (0, 0, true));
    }
}
```

### `main.rs` (demo only)
- Import the module.
- Demonstrate on a few inputs:
  - original string,
  - `normalize_whitespace`,
  - `count_words`,
  - `summarize`,
  - `make_preview` (if you do the stretch).

**Keep printing in `main` or tests — not inside utilities.**

---

## Acceptance Criteria (Baseline)

- All baseline functions implemented.
- Uses:
  - functions,
  - references,
  - tuples,
  - Unicode-safe counting via `.chars()` where applicable.
- **≥ 3 unit tests** inside `text_utils`.
- No `unwrap` / `expect` / `panic!` in production logic.
- No I/O inside utilities.

**Stretch**
- `make_preview` cuts on char boundaries, adds `...` only when truncated.
- Extra tests: Unicode (emoji), heavy whitespace cases.

---

## Coverage Matrix

| Topic                       | Where             | How Checked                  |
|----------------------------|-------------------|------------------------------|
| Function declarations      | all utilities     | compiles, readable           |
| Params (&str / by value)   | function sigs     | no unnecessary copies        |
| Return values              | utilities         | behavior via tests           |
| Tuples                     | `summarize`       | destructuring in tests/demo  |
| Early/implicit return      | helpers           | style review                 |
| Pure functions             | `text_utils`      | no I/O, deterministic        |
| No unwrap/expect           | entire module     | quick scan                   |
| Unit tests (required)      | `#[cfg(test)]`    | `cargo test` passes          |

---

## Senior Checklist

- ❌ No `unwrap` / `expect` outside tests.
- ✅ Small, single-purpose functions.
- ✅ Clear parameter ownership (prefer `&str` for reads).
- ✅ Tuples only when a struct would be overkill.
- ✅ Thin `main`; reusable utilities.
- ✅ Comments/documentation explain Unicode choices.

---

## Decision Log (fill after coding)

Write 8–10 short lines for yourself:
- Why these normalization rules?
- What exactly belongs in `summarize`’s tuple and why?
- How did you avoid Unicode slicing pitfalls?
- What moved from `main` into utilities and why?
- What would you extract into a shared util crate later?

---

## Retrospective (3 prompts)

1) When to return a tuple vs introduce a struct?  
2) Are your functions fully pure and reusable?  
3) Which Day 3 pattern will you reuse tomorrow without thinking?

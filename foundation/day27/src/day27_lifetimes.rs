//! Day 27 - Lifetimes: traps, fixes, and tiny utilities.
//!
//! Lifetime elision summary (functions):
//! 1) Each input reference gets its own lifetime.
//! 2) If there is exactly one input reference, its lifetime is used for output.
//! 3) If there are multiple input references and one is `&self`/`&mut self`,
//!    the output uses the lifetime of `self`.
//! Otherwise, you must write explicit lifetimes.
//!
//! Cow<'a, str> escape hatch (conceptual):
//! When a function sometimes needs to return owned data because it cannot
//! prove a borrow lives long enough, `Cow<'a, str>` can carry either a borrow
//! or an owned `String`. We only mention it today; no code uses it.

// Trap A: Returning reference without proving lifetime.
// What the compiler protects: returned references must outlive the caller's use.
// What must be proven: the output borrows from an input with a known lifetime.
/// Broken example (compile_fail):
/// ```compile_fail
/// fn choose(a: &str, b: &str) -> &str {
///     if a.len() >= b.len() { a } else { b }
/// }
/// ```
///
/// Fix: tie the output to the shared lifetime of both inputs.
pub fn choose<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() {
        a
    } else {
        b
    }
}

/// Mini-utility: returns the longer of two strings using explicit lifetimes.
pub fn pick_longer<'a>(a: &'a str, b: &'a str) -> &'a str {
    choose(a, b)
}

// Trap B: Mutable + immutable borrows conflict.
// What the compiler protects: you cannot read while a mutable borrow is active.
// What must be proven: the mutable borrow ends before any shared borrow starts.
/// Broken example (compile_fail):
/// ```compile_fail
/// fn conflict(v: &mut Vec<String>) -> Option<&str> {
///     let last_mut = v.last_mut().expect("value");
///     let last = v.last();
///     last_mut.push_str(\"!\");
///     last.map(|s| s.as_str())
/// }
/// ```
///
/// Fix: end the mutable borrow before taking an immutable one.
pub fn push_suffix_then_last<'a>(v: &'a mut Vec<String>, suffix: &str) -> Option<&'a str> {
    if let Some(last) = v.last_mut() {
        last.push_str(suffix);
    }

    // Mutable borrow ended; shared borrow is now allowed.
    v.last().map(|s| s.as_str())
}

// Trap C: Self-referential temptation.
// What the compiler protects: references must not point into moved or
// reallocated data owned by the same struct.
// What must be proven: a reference outlives the data it points to.
/// Broken example (compile_fail):
/// ```compile_fail
/// struct BadCatalog<'a> {
///     items: Vec<String>,
///     first: &'a str,
/// }
///
/// impl<'a> BadCatalog<'a> {
///     fn new(items: Vec<String>) -> Self {
///         let first = items[0].as_str();
///         Self { items, first }
///     }
/// }
/// ```
///
/// Fix: store indices, or return references from methods instead of storing them.
pub struct Catalog {
    items: Vec<String>,
}

impl Catalog {
    pub fn new(items: Vec<String>) -> Self {
        Self { items }
    }

    pub fn add(&mut self, item: String) {
        self.items.push(item);
    }

    pub fn get(&self, index: usize) -> Option<&str> {
        self.items.get(index).map(|s| s.as_str())
    }

    pub fn find_prefix(&self, prefix: &str) -> Option<&str> {
        self.items
            .iter()
            .map(|s| s.as_str())
            .find(|s| s.starts_with(prefix))
    }
}

/// Mini-utility: search the catalog with explicit lifetimes.
pub fn find_in_catalog<'a>(catalog: &'a Catalog, needle: &str) -> Option<&'a str> {
    catalog
        .items
        .iter()
        .map(|s| s.as_str())
        .find(|s| *s == needle)
}

// Coverage matrix + DoD + decision log
//
// Coverage matrix
// Subtopic                      Where in day27_lifetimes.rs         Verified by
// lifetime basics               choose / pick_longer                unit tests
// local ref trap                Trap A broken -> fix                compile_fail doc test
// &mut vs & conflict            push_suffix_then_last               unit tests + Trap B doc test
// returning refs from struct    Catalog::get / find_in_catalog       unit tests
// self-referential concept      Trap C explanation                  doc section
//
// Senior DoD (Day 27)
// [x] can explain 2 borrow rules
// [x] can explain why local ref cannot escape
// [x] can write 'a in signature correctly
// [x] at least 6 tests: 3 happy, 3 negative
// [x] no unwrap/expect in core logic
//
// Decision log
// - Use compile_fail doc tests for the 3 negative cases.
// - Use indices/method returns instead of self-referential fields.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pick_longer_prefers_longest() {
        let result = pick_longer("cat", "tiger");
        assert_eq!(result, "tiger");
    }

    #[test]
    fn push_suffix_then_last_updates_and_reads() {
        let mut v = vec!["one".to_string(), "two".to_string()];
        let last = push_suffix_then_last(&mut v, "!");
        assert_eq!(last, Some("two!"));
        assert_eq!(v.last().map(|s| s.as_str()), Some("two!"));
    }

    #[test]
    fn find_in_catalog_finds_exact_match() {
        let catalog = Catalog::new(vec!["alpha".to_string(), "beta".to_string()]);
        let found = find_in_catalog(&catalog, "beta");
        assert_eq!(found, Some("beta"));
    }

    #[test]
    fn catalog_add_and_get() {
        let mut catalog = Catalog::new(vec!["alpha".to_string()]);
        catalog.add("beta".to_string());
        let item = catalog.get(1);
        assert_eq!(item, Some("beta"));
    }

    #[test]
    fn catalog_find_prefix_returns_first_match() {
        let catalog = Catalog::new(vec!["alpha".to_string(), "beta".to_string()]);
        let found = catalog.find_prefix("be");
        assert_eq!(found, Some("beta"));
    }
}

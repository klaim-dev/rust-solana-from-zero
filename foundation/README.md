### 🧱 Rust Foundation — 30 Days (vFinal)


## Live Progress

**Foundation (30 days)**  
```
[------------------------------] 0 / 30
```
- ⏳ [day01](foundation/day01) — variables, String/&str, stack vs heap  
- ⏳ [day02](Soon) — arithmetic, logic, if/else  
- ⏳ [day03](Soon) — functions, params, tuples, return  
- ⏳ [day04](Soon) — ownership, move/clone/Copy  
- ⏳ [day05](Soon) — borrowing, &T/&mut T, slices, split_at_mut  
- ⏳ [day06](Soon) — Result/Option, ?, domain error enum, checked arithmetic  
- ⏳ [day07](Soon) — parsing layer, Result/?/ok_or/map_err, thiserror + Display  
- ⏳ [day08](Soon) — refactor to ?, single &mut-pair helper, checked_* money, normalized errors  
- ⏳ [day09](Soon) — impl, methods, ergonomics  
- … up to day30

_DevLog rule: one day → one commit (plus tiny follow‑ups if needed)._

---


## Curriculum

### 🔵 Block 1 (Days 1–10): Language Basics

**Day 1.** Variables, String/&str, stack vs heap  
_Practice:_ functions, concatenation, slices.  
_Artifact:_ `day01_basics.rs`.

**Day 2.** Arithmetic/logic, `if/else`  
_Practice:_ calculator with flags.  
_Artifact:_ `day02_calc.rs`.

**Day 3.** Functions, params, tuples, `return`  
_Practice:_ string normalization utility.  
_Artifact:_ `day03_utils.rs` + 3 unit tests.

**Day 4.** Ownership: move, clone, Copy  
_Practice:_ find & remove unnecessary `clone`.  
_Artifact:_ table “where move/clone/Copy”.

**Day 5.** Borrowing: `&`, `&mut`  
_Practice:_ in-place struct updates.  
_Artifact:_ `day05_borrow.rs`.

**Day 6.** `Option`, `match`, `if let`  
_Practice:_ lookups in `HashMap<Option<T>>`.  
_Artifact:_ `day06_option.rs`.

**Day 7.** `Result`, `?`, `map_err`, `thiserror` + `Display`  
_Practice:_ parser with domain errors, human-readable messages.  
_Artifact:_ `day07_result.rs`.

**Day 8.** Review of Days 4–7 + mini‑quiz  
_Practice:_ refactor “to `?`”, clean error flow.  
_Artifact:_ diff “before/after”.

**Day 9.** `struct`, `impl`, methods  
_Practice:_ `User` model with validations.  
_Artifact:_ `day09_struct_impl.rs`.

**Day 10.** Mini‑model: `struct + Result + match`  
_Practice:_ user registration + duplicate error.  
_Artifact:_ `mini_model_v1/` + tests.

**Go/No‑Go for Block 1:**
- No `unwrap` in the production path.  
- You can clearly explain `self` / `&self` / `&mut self`.  
- 10+ green unit tests.

---

### 🟡 Block 2 (Days 11–17): Organization + Collections

**Day 11.** Modules: `mod/use/pub`, file tree  
_Practice:_ split `mini_model_v1` into modules.  
_Artifact:_ `mini_model_v2/` (+ README).

**Day 12.** `Vec<T>` & iterators (basics)  
_Practice:_ filters, maps, slices.  
_Artifact:_ `day12_vec_iter.rs`.

**Day 13.** `HashMap<K, V>`  
_Practice:_ indices by id/email, `.entry()`.  
_Artifact:_ `day13_hashmap.rs`.

**Day 14.** Loops + adapters (`map/filter/flat_map`) + `try_fold/try_for_each`  
_Practice:_ pipeline with correct early error return; `collect::<Result<Vec<_>, _>>()`.  
_Artifact:_ `day14_iter_adapters.rs`.

**Day 15.** Nested `match`, `if let` (patterns)  
_Practice:_ unpack nested `Option<Result<_>>`.  
_Artifact:_ `day15_patterns.rs`.

**Day 16.** Mini‑project “Catalog”  
_Practice:_ in‑memory CRUD, secondary index (`HashMap`), anti‑N+1 test.  
_Artifact:_ `catalog_v1/` + tests.

**Day 17.** Input/parsing: `stdin`, `parse`, `FromStr` & validation  
_Practice:_ CLI for “Catalog”, parsing with domain errors (`map_err`).  
_Artifact:_ `catalog_cli/`.

**Go/No‑Go for Block 2:**
- Modules wired correctly, collections feel solid.  
- CLI with tests, secondary index, no `unwrap`.

---

### 🟣 Block 3 (Days 18–24): Architecture

**Day 18.** `enum + match`, type‑safe domain state  
_Artifact:_ `day18_enum.rs`.

**Day 19.** `trait`, bounds (`T: Display + Clone`), `impl Trait`  
_Artifact:_ `day19_trait_bounds.rs`.

**Day 20.** `#[derive]`: `Debug`, `Clone`, `Eq`, `Hash`, `Default`, `PartialEq/Ord`  
_Artifact:_ comparison: derive vs manual impl.

**Day 21.** Files: read/write, buffering  
_Artifact:_ `day21_fs.rs`.

**Day 22.** Program args (`std::env::args`)  
_Artifact:_ CLI mode flags.

**Day 23.** Config: `serde/toml`, `dotenvy`  
_Artifact:_ `config.rs` + `.env.example`.

**Day 24.** Review: `enum + trait + Vec` (mini‑arch)  
_Practice:_ “Catalog v2” via trait interfaces.  
_Artifact:_ `catalog_v2/`.

**Go/No‑Go for Block 3:**
- Trait interfaces, enum states, and a config layer are in place.

---

### 🟢 Block 4 (Days 25–30): Consolidation + Missing Bricks

**Day 25.** Capstone CLI: “Orders v1”  
_Practice:_ modules, domain errors, tests.  
_Artifact:_ `orders_v1/`.

**Day 26.** Review of key items  
_Artifact:_ checklist of “cleanliness rules” (errors, collections, modules).

**Day 27.** Borrow checker, move pitfalls + Lifetimes 101  
_Practice:_ 3 cases with explanations (incl. why you can’t return a ref without a lifetime from a struct).  
_Artifact:_ `day27_lifetimes.rs`.

**Day 28.** Refactor “on autopilot”  
_Practice:_ PR‑style, review notes.  
_Artifact:_ diff “before/after”.

**Day 29.** Environment setup + first micro‑bench  
_Practice:_ `clippy`, `rustfmt`, `nextest`, `criterion`; bench `Vec::with_capacity` vs none.  
_Artifact:_ `Makefile/justfile`, CI sketch, `bench_report.md`.

**Day 30.** Finale: “ready for a project”  
_Practice:_ README “What I can do” + plans.  
_Artifact:_ `README.md`.

**Go/No‑Go for Stage (30 days):**
- 3–4 mini‑projects; unified `Error` type with `Display`.  
- `FromStr` / conversions (`map_err`), trait bounds, basic lifetimes covered.  
- One micro‑bench with “before/after” findings.  
- No `unwrap` in the production path anywhere.

**Consolidation rituals (do not skip):**
- Days **8, 16, 24, 30**: short decision log (8–10 lines: what broke/fixed/why kept).  
- Day **29**: save bench graphs/results — useful later in backend work.


## FAQ

**Why split by days?**  
To keep momentum and make progress auditable. Each day is a scoped, testable slice.

**Can I reuse or learn from this?**  
Yes. Fork it, adapt, or open issues/PRs if you spot improvements.

**Where are the “real” projects?**  
After Foundation, you’ll see `backend/` and `solana/` with production‑style services and on‑chain programs.

---

## Follow

- GitHub — https://github.com/klaim-dev  
- X — https://x.com/klaimdev

---

## License

MIT — see [LICENSE](/LICENSE).

# Day 14 — Iterator Pipelines: `map/filter/flat_map`, `collect::<Result<Vec<_>, _>>()`, `try_fold` / `try_for_each`
_Structure only. No solutions._

> **Foundation plan:** Day 14 — Loops + adapters (`map/filter/flat_map`) + `try_fold/try_for_each`  
> Practice: pipeline with correct early-exit on error; `collect::<Result<Vec<_>, _>>()`  
> Artifact: `day14_iter_adapters.rs`

---

## How good is your current README?
**Strong:** clear goal (strict vs tolerant), correct tool choices (`collect<Result<_>>`, `try_fold`, `filter_map`), realistic domain (logs → report), good testing intent.  
**Needs tightening:** it’s too long for GitHub scanning; repeats ideas; `flat_map` is not needed for the baseline (keep it as stretch); trimming/whitespace policy should be explicit once.

Below is a **cleaner, publish-ready** Day 14 README you can paste into `foundation/day14/README.md`.

---

## 🎯 Focus
- Read iterator chains confidently: `iter → map → filter → collect`.
- Always know what each step yields (`&T` vs `T`, `Option<T>` vs `Result<T, E>`).
- Use `collect::<Result<Vec<_>, _>>()` to turn **many Results** into **one Result** (strict mode).
- Use `try_fold` / `try_for_each` for **early exit on first error**.
- Use `filter_map` for **tolerant mode** (drop invalid lines, keep valid ones).

**Artifact:** `day14_iter_adapters.rs` with module `pipeline.rs` + tests.

---

## ✅ Super Task — Log pipeline (spec)
You receive log lines in CSV-like format:

- Format: `<user_id>,<action>,<value>`
- Examples:
  - `1,click,3`
  - `2,purchase,1999`

### Parsing rules (strict)
- `user_id`: `u64`, must be `> 0`
- `action`: only `click` or `purchase`
- `value`: integer, must be `>= 0`

### Output: `Report`
- `total_events: usize`
- `total_clicks: u64` (sum of `value` where action is `click`)
- `total_revenue_cents: u64` (sum of `value` where action is `purchase`)
- `unique_users: usize`

### Modes
- **Strict mode:** stop on the **first** invalid line and return an error.
- **Tolerant mode:** skip invalid lines, build a report from valid lines only.

### Non-negotiable invariants
- No `unwrap` / `expect` / `panic!` in production path.
- Errors are domain-friendly and Display-ready (human readable).
- Tests cover both happy path and multiple negative cases.

---

## 🧱 Public API (baseline)
- `parse_line(line: &str) -> Result<Event, CsvError>`
- `parse_all_strict(lines: &[String]) -> Result<Vec<Event>, CsvError>`
  - implemented via: `lines.iter().map(...).collect::<Result<Vec<_>, _>>()`
- `build_report_strict(lines: &[String]) -> Result<Report, CsvError>`
  - implemented via `try_fold` (parse + aggregate, early exit)
- `build_report_tolerant(lines: &[String]) -> Report`
  - implemented via `filter_map` (drop invalid lines)

**Stretch (optional):**
- `partition(lines) -> (Vec<Event>, Vec<CsvError>)`
- top-N user_ids by revenue (sorting is fine)

---

## 🧠 MVT (Minimal Viable Theory)
### 1) `iter()` vs `into_iter()`
- `iter()` yields `&T` (borrowed) → best for `&[String]`.
- `into_iter()` yields `T` (owned) → consumes the collection.

### 2) `filter_map`
- transforms `Iterator<Item = Option<T>>` into `Iterator<Item = T>`
- tolerant mode: `parse_line(...).ok()` → `filter_map` drops failures

### 3) `collect::<Result<Vec<_>, _>>()`
- converts `Iterator<Item = Result<T, E>>` into `Result<Vec<T>, E>`
- stops on the **first** `Err(E)` → strict parsing in one line

### 4) `try_fold`
- like `fold`, but the step returns `Result<Acc, E>`
- first error terminates the fold → strict parse+aggregate in one pass

### 5) `try_for_each`
- validation pass with early exit when you don’t need an accumulator

> **Rule:** strict mode = `Result` all2R; tolerant mode = drop invalid via `Option`.

---

## 🔬 Micro drills (no solutions)
1) Strict parse: `&[&str] -> Result<Vec<u64>, E>` using `map + collect<Result<Vec<_>, _>>()`  
2) Tolerant parse: `&[&str] -> Vec<u64>` using `filter_map`  
3) `try_fold`: sum `&[i32]`, fail on first negative  
4) `try_for_each`: validate `&[u64]`, fail on first zero id  
5) Reinforce Day 13: count actions using `HashMap` + `.entry().or_insert(0)`

---

## 🧪 Test checklist (minimum)
### Strict mode
- invalid `user_id = 0` → error
- invalid action (e.g. `boom`) → error
- invalid value (negative / not a number) → error
- happy path totals are correct

### Tolerant mode
- mixed input skips bad lines and counts only valid
- empty input returns zero report
- unique_users computed correctly

**Policy test (choose once):**
- Do you trim fields (`" 1 , click , 3 "`) or treat spaces as invalid?
  - pick one policy and test it.

---

## 📌 Coverage Matrix
- `map/filter/collect` → strict bulk parsing
- `collect::<Result<Vec<_>, _>>()` → strict “first error wins”
- `try_fold` → strict parse+aggregate early-exit
- `filter_map` → tolerant mode
- `Result + ?` → parsing and propagation
- No panics → safe parsing only, no `unwrap/expect`

---

## 🧾 Decision log (after implementation)
Write 8–10 lines:
- Why strict vs tolerant exists
- Where you used `collect::<Result<Vec<_>, _>>()` and what behavior you rely on
- Where you used `try_fold` and why
- Your trimming policy and why
- What you’d add in v2 (e.g., collect all errors)

---

## ✅ Submission
- `day14_iter_adapters.rs` (`pipeline` + tests)
- This README updated after coding (decision log + coverage matrix)

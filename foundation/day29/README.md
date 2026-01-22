# 📅 Day 29 — Tooling & Micro-Benchmark: Make Code Measurable

> **Day 29 is the transition point** where “just Rust code” becomes an **engineering project with tools, contracts, and measurements**.
> Today is not about features — it’s about **discipline**: formatting, linting, testing, benchmarking, and CI.

This day builds the habits required for:

* backend latency budgets
* performance regression control
* Solana / Web3 performance-critical paths

---

## 🎯 Goal of the Day

Create a **tooling-ready mini project** that demonstrates:

* strict formatting and linting (`rustfmt`, `clippy`)
* scalable testing (`cargo nextest`)
* a **real micro-benchmark** using `criterion`
* a written **bench report with conclusions**
* a minimal **CI skeleton** that enforces standards

All artifacts live in `day29_tooling/`.

---

## 📦 Deliverables

* `Makefile` (or `justfile`) with dev commands
* `criterion` benchmark:

  * `Vec::with_capacity` vs no preallocation
* `bench_report.md` with real numbers and interpretation
* `.github/workflows/ci.yml` draft
* clean repo hygiene (`.gitignore`, no `target/`)

---

## 🧠 Minimal Viable Theory (What Matters Today)

### 1. Performance without measurement is fiction

Any performance claim without a benchmark is a guess.

Today’s micro-bench trains:

* isolating a unit of work
* avoiding compiler optimizations (`black_box`)
* interpreting variance and noise
* writing **engineering conclusions**, not hype

---

### 2. Why `Vec::with_capacity` matters

Without capacity:

* `Vec` grows dynamically
* triggers reallocations and memory copies

With capacity:

* one allocation
* predictable memory behavior

This pattern generalizes to:

* `String::with_capacity`
* `HashMap::with_capacity`
* Solana account buffers

---

### 3. Criterion basics

* warm-up phase
* multiple samples → distribution, not a single number
* always run in **release mode**
* compare functions, not IO

---

### 4. Tooling as contracts

* `rustfmt` → stable style
* `clippy` → reduced accidental complexity
* `nextest` → scalable test execution
* CI → non-negotiable quality gate

---

## 🗂️ Project Structure

```text
day29_tooling/
├── Cargo.toml
├── src/
│   └── lib.rs
├── benches/
│   └── vec_capacity.rs
├── bench_report.md
├── README.md
├── Makefile
├── .gitignore
└── .github/
    └── workflows/
        └── ci.yml
```

---

## 🔧 Development Commands

All common actions are standardized:

```bash
make fmt      # cargo fmt
make clippy   # cargo clippy -D warnings
make test     # cargo test
make bench    # cargo bench
make ci       # fmt + clippy + test
```

(Exact commands documented in `Makefile`.)

---

## 🧪 Benchmark Description

### Benchmark target

Compare two functions:

* `build_vec_no_cap(n)`
* `build_vec_with_cap(n)`

Both build a `Vec<u64>` of size `n`, differing only in preallocation.

### Benchmark tool

* `criterion`
* `black_box` used to avoid dead-code elimination
* multiple input sizes (e.g. 1k / 10k / 100k)

### Why this benchmark

* simple
* deterministic
* teaches allocation costs
* mirrors real production patterns

---

## 📊 Bench Report

Results and interpretation are documented in **`bench_report.md`**, including:

* environment (OS, CPU, Rust version)
* command used
* results table
* notes on variance
* **engineering conclusion**

Example conclusion:

> `Vec::with_capacity` consistently reduces runtime by avoiding reallocations.
> The effect grows with input size and is most relevant in hot loops and repeated builds.

---

## 🧪 Tests

Even on a tooling day, correctness matters.

Included unit tests:

* both builders return vectors of correct length
* sanity checks for behavior

---

## 🛡️ CI Skeleton

A minimal GitHub Actions workflow is included.

Checks enforced:

* `cargo fmt -- --check`
* `cargo clippy -- -D warnings`
* `cargo test`

This is the **baseline CI contract** used later in backend and Solana projects.

---

## 📋 Coverage Matrix

| Area            | Where               | Verified by          |
| --------------- | ------------------- | -------------------- |
| Formatting      | Makefile + CI       | `cargo fmt --check`  |
| Linting         | Makefile + CI       | clippy `-D warnings` |
| Testing         | src/ + CI           | `cargo test`         |
| Benchmarking    | benches/            | `cargo bench`        |
| Reproducibility | bench_report.md     | documented commands  |
| CI discipline   | `.github/workflows` | GitHub Actions       |

---

## ✅ Definition of Done (Senior Level)

* ✅ `.gitignore` includes `target/`
* ✅ `edition = "2021"`
* ✅ no committed build artifacts
* ✅ fmt / clippy / tests pass cleanly
* ✅ benchmark produces stable results
* ✅ bench report contains **interpretation**, not just numbers
* ✅ CI skeleton exists and runs locally

---

## 🧾 Decision Log (Summary)

* measured allocation cost of growing `Vec`
* confirmed `with_capacity` reduces reallocations
* learned to isolate benchmarks properly
* reinforced habit: **measure before optimizing**
* next benchmarks to consider:

  * `String::with_capacity`
  * `HashMap::with_capacity`
  * iterator vs manual loops

---

## 🚀 Why This Day Matters

This day establishes habits that carry forward:

* **Backend:** latency budgets, perf regressions
* **Async Rust:** hot paths under load
* **Solana:** account allocation and compute limits

From here on, performance discussions are backed by data.

---

🕊️ **Day 29 complete.**
Code is no longer “just code” — it is measurable, reproducible, and enforceable.

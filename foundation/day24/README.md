
# Day 24 — Mini-architecture: Runtime Pipeline with Traits, Enums & Config

> **Goal of the day:** build an extensible, runtime-configured processing pipeline using `trait`, `enum`, `Vec<Box<dyn Trait>>`, and clean layer boundaries — without `if/else` spaghetti and without leaking infrastructure into the domain.

This project is part of the **Rust Foundation** track and represents a **mini-production module**, not a toy example.

---

## 🧠 Core Idea

We build a **pipeline of processing steps** that:

* is configured **at runtime** (via env / toml),
* is **order-dependent** by design,
* is **extensible** without modifying the pipeline runner,
* strictly separates:

  * **domain logic**
  * **pipeline behavior**
  * **infrastructure (config/env/file)**
  * **application wiring**

This is a reusable architectural pattern applicable to:

* ETL pipelines
* validation / normalization flows
* backend request processing
* blockchain indexing / preprocessing
* CLI / batch tools

---

## 🏗 Architecture Overview

```
config (infra) ──┐
                 │
                 ▼
        PipelineSpec (enum)
                 │
                 ▼
          Builder / Factory
                 │
                 ▼
        Pipeline { Vec<Box<dyn Step>> }
                 │
                 ▼
             Domain Record
```

### Layer boundaries

| Layer      | Responsibility                                |
| ---------- | --------------------------------------------- |
| `domain`   | Business invariants, types, errors            |
| `pipeline` | Processing logic and orchestration            |
| `config`   | Reading env / file and merging                |
| `app`      | Composition root (wiring everything together) |
| `main`     | Thin bootstrap, no logic                      |

**Important rule:**
👉 *Domain does not know about config or pipeline.*
👉 *Pipeline does not know about env or files.*
👉 *App is the only place where everything meets.*

---

## 📦 Project Structure

```
day24_catalog_v2/
├─ Cargo.toml
├─ src/
│  ├─ lib.rs
│  ├─ main.rs
│  │
│  ├─ domain/
│  │  ├─ record.rs        # Record + newtypes (Sku, Name, PriceCents)
│  │  └─ error.rs         # DomainError
│  │
│  ├─ pipeline/
│  │  ├─ step.rs          # trait Step + implementations
│  │  ├─ pipeline.rs      # Pipeline runner
│  │  ├─ spec.rs          # StepKind + PipelineSpec parsing
│  │  ├─ build.rs         # Builder / factory
│  │  └─ error.rs         # BuildError
│  │
│  ├─ config/
│  │  ├─ env.rs           # EnvConfig (raw)
│  │  ├─ file.rs          # FileConfig (toml)
│  │  ├─ models.rs        # Config + merge logic
│  │  └─ error.rs         # ConfigError
│  │
│  └─ app/
│     ├─ build.rs         # build_app
│     ├─ state.rs         # AppState
│     └─ error.rs         # AppError
│
├─ tests/
│  ├─ pipeline_build.rs
│  └─ pipeline_run.rs
│
├─ config.example.toml
├─ .env.example
└─ README.md
```

---

## 🧩 Domain Model

### `Record`

```text
Record {
  sku: Sku,
  name: Name,
  price_cents: PriceCents
}
```

### Invariants (enforced by domain types)

* `Sku` — must exist, non-empty string
* `Name` — must exist, not empty after `trim`
* `PriceCents` — must be > 0

All invariants are enforced **only in the domain**, via constructors.
No pipeline step can bypass them.

---

## 🔧 Pipeline

### Step contract

```rust
trait Step {
    fn apply(&self, record: Record) -> Result<Record, DomainError>;
}
```

Each step:

* takes ownership of a `Record`
* returns a new `Record` or a `DomainError`
* knows nothing about other steps or pipeline order

### Implemented steps (Baseline)

| Step                 | Description                                 |
| -------------------- | ------------------------------------------- |
| `TrimName`           | Trims whitespace around `name`              |
| `NormalizeSpaceName` | Collapses all whitespace into single spaces |
| `LowerSku`           | Normalizes SKU to lowercase                 |

### Why `dyn Step`

* steps are chosen **at runtime**
* order is dynamic
* generics would over-constrain the design

This is a **deliberate runtime polymorphism trade-off**.

---

## 🧠 PipelineSpec (description, not behavior)

```text
PIPELINE=trim,lower_sku,normalize_space
```

Parsed into:

```rust
PipelineSpec {
  steps: Vec<StepKind>
}
```

Where `StepKind` is an enum describing **what** should run, not **how**.

---

## 🏭 Builder / Factory

The builder is the **only place** where:

```
StepKind  ──▶  Box<dyn Step>
```

This guarantees:

* no magic
* explicit mapping
* easy extension

Adding a new step requires:

1. new `struct` implementing `Step`
2. new `StepKind` variant
3. one `match` arm in the builder

No other code changes.

---

## ⚙️ Configuration

### Sources

* Environment variables
* Optional TOML file

### Merge rule

```
env > file
```

### Required

* `PIPELINE`

### Optional

* `DATA_FILE`
* `STRICT` (reserved for future extensions)

### Example `.env`

```env
PIPELINE=trim,lower_sku,normalize_space
CONFIG_PATH=./config.toml
```

### Example `config.toml`

```toml
pipeline_raw = "trim,normalize_space"
data_file = "./data.txt"
```

---

## 🧪 Tests

### Build / Spec tests

* empty pipeline → error
* unknown step → error
* valid spec builds pipeline

### Runtime tests

* each step works correctly
* pipeline processes records sequentially
* early exit on first error

### Negative cases (≥3)

* empty pipeline
* unknown step name
* invalid domain data after step

---

## 🧾 Coverage Matrix

| Topic             | Location               | Verified by          |
| ----------------- | ---------------------- | -------------------- |
| trait objects     | `pipeline/step.rs`     | run tests            |
| enum spec         | `pipeline/spec.rs`     | build tests          |
| runtime order     | `pipeline/pipeline.rs` | run tests            |
| domain invariants | `domain/record.rs`     | negative tests       |
| config merge      | `config/models.rs`     | config tests         |
| clean boundaries  | app wiring             | compilation + review |

---

## 📌 Design Decisions (Decision Log)

1. Used `dyn Step` for runtime configurability.
2. Separated `StepKind` (description) from step behavior.
3. Pipeline is immutable and value-based (no in-place mutation).
4. Domain invariants enforced only by domain constructors.
5. Builder is the single point of step instantiation.
6. Config layer kept infra-only, no business logic.
7. App layer used as composition root.
8. No `unwrap` / `expect` in production paths.

---

## 🚀 How to Run

```bash
# set pipeline
export PIPELINE=trim,lower_sku,normalize_space

# optional config file
export CONFIG_PATH=./config.toml

# run
cargo run
```

---

## ✅ Status

**Day 24 complete.**
This project demonstrates **real architectural thinking**, not just Rust syntax.

Next days will build on this foundation with:

* validation modes
* metrics / reporting
* async pipelines
* backend integration

---

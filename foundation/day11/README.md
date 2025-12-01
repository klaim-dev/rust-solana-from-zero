
# Day 11 — Modules, `mod` / `use` / `pub`, Module Tree

public reference implementation for Day11.

Day 11 is the moment where your code stops being a single file and becomes a **module tree**.

> Plan from the Foundation:  
> *Day 11. Modules: mod/use/pub, file tree.  
> Practice: split `mini_model_v1` into modules.  
> Artifact: `mini_model_v2/` (+ README).*

---

## 🎯 Focus of the Day

- Understand how Rust sees a project: **crate → modules → submodules**.
- Control visibility with `pub`, `pub(crate)`, and private items.
- Take the Day 10 model (`User` + `UserRegistry`) and split it into modules.
- Prepare the ground for future architecture: **domain / app / infra**.

Artifact of the day: `mini_model_v2/` — the same domain as Day 10, but organized as a clean module tree.

---

## 🎯 0) Super Task (spec only)

**Goal:**  
Turn your Day 10 code into a library crate with a clear module structure and a tiny API facade:

```text
mini_model_v2/
  src/
    lib.rs
    domain/
      mod.rs
      user.rs
      registry.rs
    api/
      mod.rs
  README.md
  Cargo.toml
````

### Domain layer

* `domain::user` — `User` + `UserError` (validation, invariants).
* `domain::registry` — `UserRegistry` + `RegistrationError` (duplicates, lookup).

### API layer (thin facade)

* `api::register_user` — convenience wrapper over `UserRegistry::register`, accepting `&str` for email and calling into the domain.

### Crate public API (facade)

In `lib.rs`:

* `pub mod domain;`
* `pub mod api;`
* `pub use domain::user::{User, UserError};`
* `pub use domain::registry::{UserRegistry, RegistrationError};`
* `pub use api::register_user;`

### Invariants

* All invariants from Day 10 are preserved.
* No new `unwrap` / `expect` / `panic!` in production code.
* Dependency direction:

  * `domain` does **not** depend on `api`;
  * `api` depends on `domain`.

**Baseline acceptance:**

* `cargo build` and `cargo test` are green.
* `mini_model_v1` logic is effectively preserved as `mini_model_v2`.
* Public API: external crate can `use mini_model_v2::{UserRegistry, register_user};`.
* At least 5 tests cover both domain and the simple `register_user` facade.

---

## 🧠 1) Minimal Theory — Modules (`mod` / `use` / `pub`)

### 1. Crate and module tree

* A **crate** is a library or binary.
* For a library crate, `src/lib.rs` is the root of the module tree.
* `mod foo;` in `lib.rs` tells Rust to look for `src/foo.rs` or `src/foo/mod.rs`.

Think: **“module = logical namespace, file = where the module lives physically”**.

---

### 2. Visibility: `pub`, `pub(crate)`, private

* By default, items are **private** to their module.
* `pub` makes items visible outside the module.
* `pub(crate)` makes items visible inside the crate, but hidden from other crates.

Visibility is part of architecture: decide **what you expose** and **what you hide**.

---

### 3. Paths: `crate::` and `super::`

* Absolute paths from the crate root: `crate::domain::user::User`.
* Relative paths inside a tree:

  * from `domain::registry` to `domain::user` → `super::user::User`.

Rule of thumb:

* For public API and examples, think in `crate::...` paths or re-exports.
* Inside the domain tree, `super::...` is fine and often shorter.

---

### 4. `pub use` as a facade

* We can re-export types from inner modules:

  ```rust
  pub use domain::user::User;
  ```

* External code can then just do:

  ```rust
  use mini_model_v2::User;
  ```

Day 11 goal: make `User`, `UserRegistry`, `RegistrationError`, and `register_user` available at the crate root via `pub use`.

---

## 🔬 2) Micro Tasks (syntax and mental model)

No solutions are included here — this is just the practice plan.

### Micro 1 — Minimal module

* Create a crate with:

  * `mod math;` in `lib.rs`,
  * `math.rs` with a small `pub fn sum(a, b)` and a test using `mycrate::math::sum`.

---

### Micro 2 — Submodule with `mod.rs`

* Layout:

  ```text
  src/
    lib.rs
    domain/
      mod.rs
      user.rs
  ```

* `lib.rs` exposes `pub mod domain;`.

* `domain/mod.rs` exposes `pub mod user;`.

* `domain/user.rs` defines a tiny `User` and a test accessing it.

---

### Micro 3 — `pub use` facade

* Add `pub use domain::user::User;` in `lib.rs`.
* Verify tests can use both paths:

  * `mycrate::domain::user::User`,
  * `mycrate::User`.

---

### Micro 4 — `pub(crate)` vs private

* Create a `pub(crate) struct InternalUser { ... }` inside some module.
* Confirm it’s usable inside the crate, but not from another crate.

---

### Micro 5 — `super::` in a submodule

* In `domain/mod.rs`, declare `pub mod user; pub mod registry;`.
* In `domain/registry.rs`, import `User` with `use super::user::User;`.
* Verify with a small test.

---

## 🧩 3) Mini Task — Small Refactor (`mini_model_modplay/`)

Goal: take the **simplified** Day 10 model (User + UserRegistry) and split it into a basic module tree:

```text
src/
  lib.rs
  domain/
    mod.rs
    user.rs
    registry.rs
```

Requirements:

* `domain/user.rs` — only `User` + `UserError`.

* `domain/registry.rs` — only `UserRegistry` + `RegistrationError`.

* `domain/mod.rs`:

  ```rust
  pub mod user;
  pub mod registry;
  ```

* `lib.rs`:

  ```rust
  pub mod domain;
  pub use domain::user::{User, UserError};
  pub use domain::registry::{UserRegistry, RegistrationError};
  ```

* One integration test uses the crate directly and ensures registration still works.

Mini task = warm-up before the full `mini_model_v2`.

---

## 🚀 4) Super Task — `mini_model_v2` (90 min, “prod mode”)

Target layout:

```text
mini_model_v2/
  src/
    lib.rs
    domain/
      mod.rs
      user.rs
      registry.rs
    api/
      mod.rs
  README.md
  Cargo.toml
```

### Domain (`src/domain`)

* `user.rs`:

  * `User`, `UserError`, constructor with validation, getters, `is_adult`, `activate`/`deactivate`.
* `registry.rs`:

  * `UserRegistry`, `RegistrationError`,
  * methods: `new`, `all`, `find_by_id`, `find_by_email`, `register`,
  * **no `unwrap/expect/panic!` in production**,
  * `Result` + `Option` used as in Day 10.

### API (`src/api`)

* `api::register_user`:

  * takes `&mut UserRegistry`, `id`, `&str email`, `age`,
  * inside, converts email to `String` and calls `UserRegistry::register`.

### lib.rs

* Declare:

  ```rust
  pub mod domain;
  pub mod api;
  ```

* Re-export:

  ```rust
  pub use domain::user::{User, UserError};
  pub use domain::registry::{UserRegistry, RegistrationError};
  pub use api::register_user;
  ```

### Tests

* Keep or adapt Day 10 tests to the new module layout.
* Add at least:

  * a test for `register_user` happy path,
  * a test where `register_user` returns a duplicate error.

---

## 🧾 5) End of Day 11

### Coverage Matrix

| Subtopic                         | Where in code         | How it’s verified                 |
| -------------------------------- | --------------------- | --------------------------------- |
| Modules (`mod`)                  | `lib.rs`, `domain/*`  | crate builds, tests run           |
| Visibility (`pub`, `pub(crate)`) | `domain`, `api`       | access from tests / external code |
| Paths (`crate::`, `super::`)     | imports in domain/api | compilation, clarity              |
| `pub use` facade                 | `lib.rs`              | external-style imports work       |
| mini_model_v1 → v2 split         | `mini_model_v2/`      | behavior preserved via tests      |
| No `unwrap/expect` in prod       | `domain/*`, `api/*`   | code review / grep                |

### Senior Checklist (Day 11)

* ❌ No `unwrap` / `expect` / `panic!` in production modules.
* ✅ Module structure is meaningful, not “files for the sake of files”.
* ✅ Boundaries are clear: **domain** vs **API**.
* ✅ Visibility (`pub`, `pub(crate)`, private) is intentional.
* ✅ Negative cases (invalid data, duplicates) are still covered after refactor.

---


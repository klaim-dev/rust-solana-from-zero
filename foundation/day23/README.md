# 📅 Day 23 — Config Layer: fail-fast, typed settings, zero surprises

This day introduces a **production-grade configuration layer** for a Rust application.

The goal is to stop “passing settings by hand” and instead build a **single, explicit config entry point** that:

* loads settings from **config file + environment**
* applies **clear precedence rules**
* validates everything **once at startup**
* guarantees **no runtime surprises**

This is a **direct bridge to backend services** (Axum `Config` / `AppState` pattern).

---

## 🎯 Goals of the Day

Build a small but realistic config system that:

* loads configuration from:

  * `config.toml` (optional baseline)
  * environment variables
* applies precedence:

  ```
  env > file > defaults
  ```
* fails fast on startup if required settings are missing or invalid
* exposes a **typed `Config`**, not raw strings
* keeps **clean boundaries**:

  * domain / app logic never touches env or files
  * config logic lives in its own module
* is fully covered by **negative tests**

---

## 🧠 Key Design Principles

### 1. Fail-fast is a safety feature

If configuration is invalid, the program **must not start**.

Errors are detected:

* before any business logic runs
* before any files are read or commands executed

This avoids undefined states and late crashes.

---

### 2. Typed config, not strings

All parsing and validation happens once.

After `Config::load()`:

* no `String` parsing
* no `Option` checks
* no env/file access

The rest of the program works with **guaranteed invariants**.

---

### 3. Explicit precedence (no magic)

Precedence is part of the public contract:

```
environment variables
        ↓
config.toml
        ↓
defaults
```

This behavior is:

* documented
* tested
* predictable

---

### 4. Clean boundaries

* `config` module handles **infra concerns**
* `domain` and `app` receive values explicitly
* CLI overrides (later) are applied **outside** config

No layer leakage.

---

## 🧱 Project Structure

```text
day23/
├─ Cargo.toml
├─ src/
│  ├─ main.rs
│  ├─ env/
│  │  ├─ mod.rs        # Env trait + OsEnv/FakeEnv
│  ├─ config/
│  │  ├─ mod.rs
│  │  ├─ error.rs      # ConfigError
│  │  ├─ model.rs      # Config, FileConfig, AppEnv
│  │  ├─ file.rs       # config.toml loading
│  │  └─ env.rs        # env parsing helpers
│  └─ app/
│     └─ run.rs
└─ tests/
   └─ config.rs        # contract tests
```

---

## ⚙️ Configuration Sources

### Environment variables

| Variable    | Required | Description             |
| ----------- | -------- | ----------------------- |
| `DATA_FILE` | yes      | Path to data file       |
| `APP_ENV`   | no       | `dev` / `test` / `prod` |
| `PAGE_SIZE` | no       | Page size (usize)       |

---

### `config.toml` (optional)

Example:

```toml
app_env = "dev"
data_file = "/tmp/data.json"
page_size = 50
```

Notes:

* all fields are optional
* missing file is allowed
* syntax errors are **fatal**

---

## 🔁 Merge Algorithm (high level)

1. Try to load `config.toml`

   * ignore only `NotFound`
   * any other I/O error → fail
2. Load environment variables
3. Merge values:

   * env overrides file
   * defaults applied last
4. Validate required fields
5. Return fully typed `Config`

---

## 📦 Public API

```rust
pub struct Config;

impl Config {
    pub fn load<E: Env>(env: &E, path: &Path) -> Result<Self, ConfigError>;
}
```

After this call:

* configuration is valid
* values are typed
* no more env/file access is needed

---

## 🧪 Testing Strategy

This project uses **deterministic tests only**:

* no real environment variables
* no real filesystem dependencies
* no global state

### Covered scenarios

* missing required values
* invalid env values
* invalid TOML syntax
* config file not found
* env overrides file
* defaults applied correctly

All tests use:

* `FakeEnv`
* `tempfile`

---

## 🚫 What This Day Explicitly Avoids

* reading env variables inside business logic
* parsing strings “on demand”
* implicit defaults
* hidden precedence
* `unwrap` / `expect` in production code

---

## 🧩 Known Limitation (Intentional)

At this stage, required fields are validated **before CLI overrides**.

This means:

```
invctl print --file X
```

may still fail if `DATA_FILE` is missing from env/file.

This is a **known technical debt**, intentionally left for the next iteration
when CLI > env > file precedence is finalized.

---

## 📈 Why This Matters

This config layer can be reused directly in:

* backend services (`Axum + AppState`)
* CLI tools
* batch jobs
* Docker / CI environments

It scales naturally to:

* secrets
* structured configs
* multiple config files
* service-level settings

---

## ✅ Day 23 Status

* Architecture: ✅
* Fail-fast: ✅
* Typed config: ✅
* Precedence rules: ✅
* Tests: ✅
* Senior-level review: ✅

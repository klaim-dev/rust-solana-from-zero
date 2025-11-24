
# Day 8 — Refactor on `?` + Option/Result (Legacy Cleanup)

Public learning **skeleton** for Foundation Day 8 — **no solutions** in this repo.

Theme of the day: **review Days 4–7** and refactor legacy code to:

- remove magic values (`-1`, `""`, `0` as “no value”);
- remove `unwrap/expect/panic!` from production logic;
- introduce a clean domain model + `ConfigError` enum;
- use `Option` and `Result` consciously;
- use `thiserror` for human-readable error messages.

Artifact of the day: `day08_refactor/` with:

- `legacy/` — old code,
- `refactored/` — new code,
- `DIFF.md` — what changed and why.

---

## 🎯 0) Super Task — Refactor `legacy_config`

You receive a “legacy” module `legacy_config` that:

- parses config from lines,
- uses:
  - `bool` as a success/failure flag,
  - `String` as a generic “error container”,
  - magic values (`-1`, `""`, `0`) instead of `Option`,
  - `unwrap` / `expect` / `panic!`.

Your goal:

1. Extract a **domain model** (`Config`), a **domain error enum** (`ConfigError`), and an API that uses `Result` and `Option` correctly.
2. Rewrite functions:
   - no `unwrap/expect` in production code,
   - no magic values,
   - with `thiserror` and human-readable `Display`.
3. Fill `DIFF.md`:
   - what was “before” (errors, ownership, Option/Result),
   - what is “after”,
   - which patterns you will now apply automatically.

### Target Model (refactored)

```rust
#[derive(Debug, PartialEq)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub use_tls: bool,
    pub timeout_ms: u64,
}

#[derive(thiserror::Error, Debug)]
pub enum ConfigError {
    #[error("empty line")]
    EmptyLine,

    #[error("missing '=' separator in line: {0}")]
    MissingSeparator(String),

    #[error("unknown key: {0}")]
    UnknownKey(String),

    #[error("duplicate key: {0}")]
    DuplicateKey(String),

    #[error("invalid port: {0}")]
    InvalidPort(String),

    #[error("invalid timeout: {0}")]
    InvalidTimeout(String),

    #[error("invalid boolean: {0}")]
    InvalidBool(String),

    #[error("missing required key: {0}")]
    MissingRequiredKey(&'static str),
}
````

### Target API (refactored)

```rust
/// Parse a single "key=value" line.
pub fn parse_config_line(line: &str) -> Result<(String, String), ConfigError>;

/// Apply one key/value pair to PartialConfig (mutable, no unnecessary clones).
pub fn apply_config_kv(
    cfg: &mut PartialConfig,
    key: &str,
    value: &str,
) -> Result<(), ConfigError>;

/// Parse the whole config input into a final Config.
pub fn parse_config(input: &str) -> Result<Config, ConfigError>;
```

`PartialConfig` is an internal struct with `Option` fields:

```rust
#[derive(Default)]
struct PartialConfig {
    host: Option<String>,
    port: Option<u16>,
    use_tls: Option<bool>,
    timeout_ms: Option<u64>,
}
```

### Invariants

* `parse_config`:

  * requires at least `host` and `port`,
  * returns `MissingRequiredKey(...)` for missing required keys.
* No `unwrap/expect` in production code.
* No magic values (`-1`, `0`, `""`) used as error signals.
* Unknown keys are not silently ignored — they produce `UnknownKey` (or clearly documented behavior).

---

## 🧠 1) MVT — Review of Days 4–7

**Ownership / borrowing (Days 4–5)**

1. When to use `String` vs `&str`:

   * APIs usually **accept** `&str`,
   * structs usually **store** `String`,
   * to read from `String`, you use `&str`.
2. `&T` vs `&mut T`:

   * many readers (`&T`) or a single writer (`&mut T`).
3. `get` / `get_mut`:

   * `Vec::get(idx) -> Option<&T>`,
   * `Vec::get_mut(idx) -> Option<&mut T>` — no panic.

**Option (Day 6)**

4. `Option<T>` instead of `""`, `0`, `-1`:

   * absence of a value → `Option`, not “agreement on a magic number”.
5. `match` vs `if let`:

   * `match` when both branches matter,
   * `if let` when you care mostly about `Some`.

**Result (Day 7)**

6. `Result<T, E>`:

   * either a useful value, or an explanation of the error.
7. `?` and `map_err`:

   * `?` unwraps `Ok` and returns on `Err`,
   * `map_err` converts foreign `Err` into your domain error.
8. `thiserror` + `Display`:

   * domain enum + `#[error("...")]` → clean, human-readable messages.

---

## 🔍 Self-Quiz

1. When would you choose `Option<T>` vs `Result<T, E>`?

2. Why is `unwrap` bad in production, and how do you replace it with `?` / `match`?

3. Name 2 places in your previous days where `Option` would be wrong and `Result` is required.

4. Can you write from memory:

   ```rust
   pub fn get_item<'a>(inv: &'a Inventory, id: u32) -> Option<&'a Item>;
   ```

5. Where did you already use `#[source]` in errors and why?

---

## 🔬 2) Micro Tasks (~45 min, no solutions here)

### Micro 1 — Remove magic value for port

Legacy:

```rust
fn parse_port(raw: &str) -> u16 {
    if let Ok(p) = raw.parse::<u16>() {
        p
    } else {
        0 // 0 as "error"
    }
}
```

Target:

```rust
fn parse_port(raw: &str) -> Result<u16, ConfigError>;
```

* Use `ConfigError::InvalidPort(raw.to_string())`.

---

### Micro 2 — Replace `unwrap` with `Result`

Legacy:

```rust
fn parse_bool(raw: &str) -> bool {
    match raw {
        "true" => true,
        "false" => false,
        _ => panic!("invalid bool"),
    }
}

fn bool_flag(line: &str) -> bool {
    let parts: Vec<_> = line.split('=').collect();
    parts[1].trim().parse::<bool>().unwrap()
}
```

Target:

1. `parse_bool(raw: &str) -> Result<bool, ConfigError>` with `InvalidBool`.
2. `bool_flag(line: &str) -> Result<bool, ConfigError>` — no `unwrap`, no panic.

---

### Micro 3 — Turn Option into Result

Legacy:

```rust
fn timeout_ms(cfg: &PartialConfig) -> u64 {
    cfg.timeout_ms.unwrap_or(1000)
}
```

Target:

```rust
fn timeout_ms(cfg: &PartialConfig) -> Result<u64, ConfigError>;
```

* If present → `Ok(value)`.
* If absent → `Err(ConfigError::MissingRequiredKey("timeout_ms"))`.

---

### Micro 4 — `map_err` for numeric parsing

Skeleton:

```rust
fn parse_timeout(raw: &str) -> Result<u64, ConfigError> {
    let n = raw.trim().parse::<u64>().map_err(|_| ???)?;
    Ok(n)
}
```

Target:

* Fill in `map_err` using `ConfigError::InvalidTimeout`, without losing the original string.

---

### Micro 5 — From `bool` flag to `Result`

Legacy:

```rust
fn load_config_legacy(lines: &str, cfg: &mut Config) -> bool {
    let mut ok = true;
    for line in lines.lines() {
        if !apply_line_legacy(cfg, line) {
            ok = false;
        }
    }
    ok
}
```

Target mental shape:

```rust
fn load_config(lines: &str) -> Result<Config, ConfigError>;
```

* First error → immediately return `Err`.

---

## 🧩 3) Mini Task — `numbers_v2` (~45 min)

Legacy:

```rust
pub fn avg(nums: &[i32]) -> i32 {
    if nums.is_empty() {
        0
    } else {
        let sum: i32 = nums.iter().sum();
        sum / nums.len() as i32
    }
}

pub fn parse_and_avg(raw: &str) -> i32 {
    // "1,2,3"
    let parts: Vec<_> = raw.split(',').collect();
    let mut nums = Vec::new();

    for p in parts {
        let n = p.trim().parse::<i32>().unwrap();
        nums.push(n);
    }

    avg(&nums)
}
```

Tasks:

1. Make `avg` return `Option<f64>`:

   * empty slice → `None`,
   * non-empty → `Some(average)`.

2. Make `parse_and_avg` return `Result<f64, ParseNumbersError>`:

   ```rust
   #[derive(thiserror::Error, Debug)]
   pub enum ParseNumbersError {
       #[error("empty input")]
       EmptyInput,

       #[error("invalid number: {0}")]
       InvalidNumber(String),
   }
   ```

3. No `unwrap`.

4. Tests: empty input, `"1,2,3"`, input with `"x"`.

---

## 🚀 4) Super Task (90 min, prod-style)

Project layout:

```text
day08_refactor/
  legacy/
    legacy_config.rs
  refactored/
    config.rs
  DIFF.md
  Cargo.toml
```

### `legacy/legacy_config.rs` (given)

```rust
#[derive(Debug)]
pub struct Config {
    pub host: String,
    pub port: i32,
    pub use_tls: bool,
    pub timeout_ms: i32,
}

// Returns true if the line is "ok".
pub fn apply_line_legacy(cfg: &mut Config, line: &str) -> bool {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return true;
    }

    let parts: Vec<_> = trimmed.split('=').collect();
    if parts.len() != 2 {
        return false;
    }

    let key = parts[0].trim();
    let value = parts[1].trim();

    if key == "host" {
        cfg.host = value.to_string();
        true
    } else if key == "port" {
        // -1 == error
        let parsed = value.parse::<i32>().unwrap_or(-1);
        if parsed <= 0 {
            false
        } else {
            cfg.port = parsed;
            true
        }
    } else if key == "use_tls" {
        // any value except "true" -> false
        cfg.use_tls = value == "true";
        true
    } else if key == "timeout_ms" {
        // 0 == no timeout
        let parsed = value.parse::<i32>().unwrap_or(0);
        if parsed < 0 {
            false
        } else {
            cfg.timeout_ms = parsed;
            true
        }
    } else {
        // unknown key is ignored
        true
    }
}

/// Returns true if everything is "ok".
/// If any line is "bad" → false, but config is partially updated.
pub fn load_config_legacy(cfg: &mut Config, input: &str) -> bool {
    let mut ok = true;
    for line in input.lines() {
        if !apply_line_legacy(cfg, line) {
            ok = false;
        }
    }
    ok
}
```

Legacy issues (deliberate):

* magic values `-1`, `0` as error signals,
* `unwrap_or` hiding parse failures,
* boolean success flag,
* partial config on error,
* unknown keys silently ignored.

---

### `refactored/config.rs` (your implementation)

Target API:

```rust
#[derive(Debug, PartialEq)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub use_tls: bool,
    pub timeout_ms: u64,
}

#[derive(thiserror::Error, Debug)]
pub enum ConfigError {
    // variants as defined above
}

pub fn parse_config_line(line: &str) -> Result<(String, String), ConfigError>;

pub fn apply_config_kv(
    cfg: &mut PartialConfig,
    key: &str,
    value: &str,
) -> Result<(), ConfigError>;

pub fn parse_config(input: &str) -> Result<Config, ConfigError>;
```

Requirements:

* `parse_config`:

  * reads all lines,
  * validates known keys: `host`, `port`, `use_tls`, `timeout_ms`,
  * duplicate keys → `DuplicateKey`,
  * unknown keys → `UnknownKey`,
  * checks required fields (`host`, `port`) → `MissingRequiredKey(..)`.

* `port`:

  * parsed into `u16`,
  * `0` forbidden (or clearly documented if you allow it).

* `timeout_ms`:

  * parsed into `u64`,
  * non-negative,
  * required or optional — you pick and document in `DIFF.md`.

* `use_tls`:

  * `"true"` / `"false"` (or broader policy, but explicit).

* No `unwrap/expect` in production code.

Tests (minimum):

1. Valid config → `Ok(Config)`.
2. Empty input → `MissingRequiredKey`.
3. Invalid port → `InvalidPort`.
4. Invalid timeout → `InvalidTimeout`.
5. Invalid `use_tls` → `InvalidBool`.
6. Unknown key → `UnknownKey`.
7. Duplicate key → `DuplicateKey`.

---

### `DIFF.md`

Brief comparison:

* **Before (legacy)**:

  * magic values,
  * boolean success flags,
  * partial config on error,
  * unknown keys ignored,
  * `unwrap_or` hiding errors.

* **After (refactored)**:

  * domain enum `ConfigError`,
  * `Result<Config, ConfigError>` at the boundary,
  * strict policy for required keys,
  * `PartialConfig` with `Option` fields,
  * `thiserror` + human-readable `Display`.

---

## 🧾 5) Wrap-Up for Day 8

### Coverage Matrix

| Subtopic                | Where                               | How checked                        |
| ----------------------- | ----------------------------------- | ---------------------------------- |
| Ownership / &str/String | `Config`, `parse_config_line`       | signatures, no extra clones        |
| `&` / `&mut`            | `apply_config_kv`, `PartialConfig`  | compilation, mutable updates       |
| Option (Day 6)          | `PartialConfig`                     | `MissingRequiredKey` tests         |
| Result + `?` (Day 7)    | `parse_config`, helpers             | no unwrap, error flow via `Result` |
| thiserror + Display     | `ConfigError`                       | readable error messages            |
| Negative cases (≥ 3)    | tests                               | unknown key, bad port, empty host  |
| Refactor before/after   | `legacy` vs `refactored`, `DIFF.md` | conscious analysis                 |

### Senior Checklist

* ❌ No `unwrap/expect` in `refactored/config.rs`.
* ✅ Errors are domain enums, not raw `String`.
* ✅ `Option` used internally (`PartialConfig`), `Result` at the API boundary.
* ✅ Negative cases tested (unknown keys, duplicates, invalid values).
* ✅ Parser/config are pure functions, no `println!` / I/O inside.

### Decision Log (for yourself)

Things worth noting:

1. Which magic values existed in legacy and how you removed them.
2. Where `unwrap_or` / `panic` were turned into `Result<_, ConfigError>`.
3. Which keys you made required and why.
4. How you separated `PartialConfig` vs final `Config`.
5. Where you had to think about ownership (`String` vs `&str`) in errors.
6. Where `Option` is better than `Result`, and vice versa.
7. What you would do in a 2nd iteration (e.g., collecting all errors in `Vec<ConfigError>`).

### Retrospective

1. How many `unwrap` / magic values did you find in legacy, and how quickly do you spot them now?
2. Does “parser → Result, internals → Option/Result” feel natural and automatic?
3. Which one pattern from Days 4–7 do you now feel is truly “yours”?

```


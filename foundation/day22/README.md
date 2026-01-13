# Day 22 — `invctl`: CLI args, modes, flags, usage errors (no clap)

**Goal (Foundation Day 22):** turn “modules + domain” into a **real CLI utility** using `std::env::args()`:
- modes (`print`, `add`, …),
- flags (`--file`, `--sort`, `--help`),
- clear boundaries (**CLI parsing ≠ app orchestration ≠ domain ≠ persistence**),
- human-friendly output,
- consistent errors to `stderr`,
- correct exit codes (**0 success, 2 usage error, 1 runtime error**),
- no panics / unwraps in production code paths.

This crate is intentionally **clap-free** to deeply understand argv parsing and error contracts.

---

## What this project is

`invctl` (“inventory control”) is a small CLI tool that reads and updates an inventory stored in a text file.

### Commands (baseline)
- `print` — load inventory from file and print items sorted.
- `add` — load file, add an item, save back.

> `remove` and extra output formats are natural next steps, but the Day 22 deliverable focuses on mastering args parsing + boundaries + error contracts.

---

## Quick start

### Build
```bash
cargo build
````

### Help

```bash
cargo run -- --help
# or
cargo run -- -h
```

### Print inventory

```bash
cargo run -- print --file ./inventory.txt
cargo run -- print --file ./inventory.txt --sort name
cargo run -- print --file ./inventory.txt --sort price
```

### Add an item

```bash
cargo run -- add --file ./inventory.txt --id 3 --sku SKU3 --name "Orange" --price 150
```

---

## Data format

Inventory is stored as plain text lines (one item per line). Example:

```text
id=1 sku=SKU1 name=Apple price=100
id=2 sku=SKU2 name=Banana price=200
```

* Parsing is strict: invalid numeric fields (e.g. `price=abc`) are runtime errors.
* Save is **atomic**: write to `*.tmp`, `flush + sync`, then `rename`.

---

## Sorting

Supported `--sort` values:

* `name`  → `SortSpec::NameAsc`
* `price` → `SortSpec::PriceDescNameAsc`

Default for `print` is `name`.

---

## Error contract (important)

This project treats errors as part of the CLI API.

### Exit codes

* `0` — success
* `2` — **usage error** (invalid argv / flags / missing required args)
* `1` — **runtime error** (I/O, invalid persisted data, domain validation failures)

### Output channels

* Success / normal output → `stdout`
* Errors → `stderr` in a consistent format:

  * `invctl: error: <message>`
  * plus a hint for usage errors: `Try 'invctl --help'`

---

## Architecture (clean boundaries)

```
src/
  main.rs              # tiny: parse → run → print/exit
  cli/
    args.rs            # argv parsing, Command/Args, usage errors
    error.rs           # UsageError, CliError, exit codes, rendering
    help.rs            # help/usage text
  app/
    run.rs             # orchestration: load → domain → save → render
  domain/
    types.rs           # Item, Sku, ItemId, SortSpec
    sort.rs            # sorting logic (keys + SortSpec policy)
    index.rs           # InventoryIndex (in-memory model)
  persist/
    fs.rs              # load/save with atomic write
    format.rs          # serialize/deserialize; parse_item_line
    error.rs           # PersistError
tests/
  args_parse.rs        # argv parsing tests (usage contract)
  run_smoke.rs         # end-to-end: temp file → print/add
```

### Key design decision: `ParseOutcome`

Parsing returns either:

* `Help` (handled in `main`, never reaches `run`)
* `Args` (valid command + flags)

This prevents “help” from leaking into business logic and avoids panics like `unreachable!()`.

---

## Testing

Run all tests:

```bash
cargo test
```

What is covered:

* **CLI parsing tests**: missing flags/values, unknown flags/commands, help override, defaults, last-wins behavior.
* **Smoke tests** (end-to-end): temp file → `print`, temp file → `add`.
* **Domain tests**: sorting correctness (NameAsc vs PriceDescNameAsc).
* **Persist tests**: deserialize/serialize stability and parsing failures.

---

## Coverage matrix (Coverage Contract)

| Subtopic                        | Where in code                    | Verified by           |
| ------------------------------- | -------------------------------- | --------------------- |
| `std::env::args` parsing        | `src/cli/args.rs`                | `tests/args_parse.rs` |
| Required flags + missing values | `src/cli/args.rs` + `UsageError` | `tests/args_parse.rs` |
| Help override                   | `ParseOutcome` + `help.rs`       | `tests/args_parse.rs` |
| Error categories + exit codes   | `src/cli/error.rs`, `main.rs`    | tests + manual runs   |
| Domain sorting policy           | `src/domain/sort.rs`             | domain unit tests     |
| Persistence read/write          | `src/persist/fs.rs`, `format.rs` | persist tests + smoke |
| End-to-end command execution    | `src/app/run.rs`                 | `tests/run_smoke.rs`  |
| “No unwrap” in prod path        | whole `src/`                     | code review (DoD)     |

---

## Senior DoD checklist

* ✅ No `unwrap/expect/panic/unreachable` in production code (`src/`)
* ✅ Consistent error contract (usage vs runtime)
* ✅ Proper exit codes (0/2/1)
* ✅ Thin `main`, clean boundaries
* ✅ No unnecessary clones in print path (uses sorted references)
* ✅ Tests include negative cases and end-to-end smoke

---

## Decision log (Day 22)

* Avoided `clap` on purpose to understand argv parsing mechanics.
* Introduced `UsageError` vs `CliError` to separate user mistakes from runtime failures.
* Implemented `ParseOutcome::{Help, Args}` to keep `help` out of `run()` and prevent panics.
* Kept parsing strict and predictable: unknown flags/args fail fast with exit code 2.
* Chose atomic file saves (tmp → flush+sync → rename) to prevent partial writes.
* Sorting strategy is a domain concern (`SortSpec` + sort keys), not a CLI trick.
* Deferred “flags before command” and extra commands/features to keep the parser simple and explicit.
* Next iteration: add `remove`, `--dry-run`, output formats, and richer structured errors.

---

## Example session

```bash
$ invctl print --file inventory.txt --sort price
OK
ITEM id=2 sku=sku2 name="Banana" price_cents=200
ITEM id=1 sku=sku1 name="Apple" price_cents=100

$ invctl add --file inventory.txt --id 3 --sku SKU3 --name "Orange" --price 150
OK ADDED sku=sku3
```

---

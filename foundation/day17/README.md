
# Day 17 — Catalog CLI (stdin → parse → domain → render)

Day 17 is the bridge from an in-memory domain model (Day 16 Catalog) to real user input.
This project implements a CLI front-end that reads commands line-by-line from stdin, parses them into typed requests (`Command` via `FromStr`), executes them against an in-memory `Catalog`, and prints stable, machine-friendly output.

## Goals

* **REPL-style CLI**: read one line → parse → execute → render → print
* **Typed parsing contract** using `FromStr` (no regex, no panics)
* **Stable output** (easy to test and script)
* **Unified error contract** for user-facing errors
* **Clean boundaries**: parsing ≠ engine ≠ rendering ≠ domain

---

## Command Protocol (Baseline)

Each input line is one command:

* `create sku=<SKU> name=<NAME> category=<CAT> price=<CENTS> active=<true|false>`
* `get id=<ID>`
* `get sku=<SKU>`
* `update id=<ID> [sku=<SKU>] [name=<NAME>] [category=<CAT>] [price=<CENTS>] [active=<true|false>]`
* `delete id=<ID>`
* `list [sku=<SKU>] [category=<CAT>] [active=<true|false>] [min=<CENTS>] [max=<CENTS>] [name=<SUBSTR>]`
* `help`
* `exit`

### Key rules

* `k=v` pairs, **order doesn’t matter**
* keys are normalized to **lowercase**
* unknown keys are errors
* duplicate keys are errors
* update requires `id` and at least one change field
* list validates range: `min <= max` when both present

---

## Output Contract

### Success

* Responses start with `OK` (or `ITEM ...` for single item lines, depending on response)
* List output is stable and deterministic (one item per line)

Examples:

* `OK CREATED id=<42>`
* `OK UPDATED ...`
* `OK DELETED ...`
* `ITEM id=... sku=... name="..." category=... price_cents=... active=...`
* for list: `OK` + multiple `ITEM ...` lines (or `OK EMPTY`)

### Errors

* Every error is rendered consistently with `ERR ...`
* Parsing/contract errors are `ERR PARSE ...` (syntax, unknown fields, missing fields, bad values)
* Domain errors are mapped to the CLI contract (e.g. conflict/not found)

(Exact formatting is implemented in the renderer and verified via script tests.)

---

## Architecture (Clean Boundaries)

**Pipeline:**

1. `Command::from_str(line)` — parsing (CLI contract)
2. `execute(&mut Catalog, Command)` — application logic (domain bridging)
3. `render(Response) -> String` — formatting only
4. `main.rs` — glue + stdin loop

**Key rule:** render doesn’t know where data comes from; engine doesn’t know about stdin; parsing doesn’t touch Catalog.

---

## How to Run

From the Day 17 crate:

```bash
cargo run
```

Then type commands:

```text
create sku=book name=Alice category=books price=100 active=true
list
get sku=book
update id=1 price=150
delete id=1
exit
```

---

## Tests

### Unit tests

* command parsing (`FromStr` for `Command`)
* KV parsing (`parse_kv`, quoted/edge cases)
* execution (`execute` mapping + domain error propagation)

### Script tests (integration style)

A helper `run_script(lines: &[&str]) -> Vec<String>` simulates the main loop without real stdin/stdout, validating **full workflows** and **negative cases**.

---

## Coverage Matrix

(What is implemented where + how it’s verified.) 

| Feature              | Implementation        | Test Coverage                                                                                         |
| -------------------- | --------------------- | ----------------------------------------------------------------------------------------------------- |
| **Command Parsing**  |                       |                                                                                                       |
| Create command       | `Command::from_str()` | `from_str_create()`, `from_str_create_with_all_fields()`, `from_str_create_missing_required_field()`  |
| Get by ID            | `Command::from_str()` | `from_str_get_id()`, `from_str_get_without_arguments()`                                               |
| Get by SKU           | `Command::from_str()` | `from_str_get_sku()`, `from_str_get_sku_and_id()`                                                     |
| Update command       | `Command::from_str()` | `from_str_update_all_fields()`, `from_str_update_partial_fields()`, `from_str_update_no_changes()`    |
| Delete command       | `Command::from_str()` | `from_str_delete()`, `from_str_delete_missing_id()`                                                   |
| List command         | `Command::from_str()` | `from_str_list_no_filters()`, `from_str_list_with_filters()`, `from_str_list_invalid_range()`         |
| Help/Exit            | `Command::from_str()` | `from_str_help()`, `from_str_exit()`                                                                  |
| **KV Parsing**       |                       |                                                                                                       |
| Parse key=value      | `parse_kv()`          | `parse_kv_happy_path()`, `parse_kv_no_equal()`, `parse_kv_duplicate()`                                |
| Quoted strings       | `normalize_value()`   | `parse_kv_multiple_quoted_strings()`, `parse_kv_bad_quote()`                                          |
| Empty values         | `parse_kv()`          | `parse_kv_empty_key()`, `parse_kv_empty_value()`                                                      |
| **Execution**        |                       |                                                                                                       |
| Create item          | `execute()`           | `execute_create()`, `execute_create_duplicate_sku_conflict()`                                         |
| Get by ID            | `execute()`           | `execute_get_by_id_success()`, `execute_get_by_id_not_found()`                                        |
| Get by SKU           | `execute()`           | `execute_get_by_sku_success()`, `execute_get_by_sku_not_found()`                                      |
| Update item          | `execute()`           | `execute_update_success()`, `execute_update_missing_id_not_found()`, `execute_update_sku_collision()` |
| Delete item          | `execute()`           | `execute_delete_success()`, `execute_delete_unknown_id_not_found()`                                   |
| List items           | `execute()`           | `execute_list_empty()`, `execute_list_with_items()`                                                   |
| **Rendering**        |                       |                                                                                                       |
| Response rendering   | `render()`            | Tested via script tests                                                                               |
| Error rendering      | `render_error()`      | Tested via script tests                                                                               |
| **Integration**      |                       |                                                                                                       |
| Full workflow        | `run_script()`        | `test_script_full_workflow()`, `test_script_with_sku_get()`                                           |
| Negative cases       | `run_script()`        | `test_script_negative_cases()`                                                                        |
| **Main Loop**        |                       |                                                                                                       |
| Read-eval-print loop | `main()`              | Tested via script tests                                                                               |

---

## Definition of Done (DoD)



* [x] All commands parse correctly (create, get, update, delete, list, help, exit)
* [x] All CRUD operations work through CLI
* [x] Error handling for parsing errors (ERR PARSE)
* [x] Error handling for domain errors (ERR with domain error message)
* [x] Script test covers full workflow (create, list, get, update, delete, exit)
* [x] Script test covers negative cases (duplicate SKU, missing ID, unknown ID)
* [x] Main loop is thin glue (no business logic, only pipeline)
* [x] No unwrap/expect in production code path
* [x] All unit tests pass (83+ tests)
* [x] Integration tests pass

---

## Decision Log



1. **Thin main.rs**: Main function is a pure pipeline (read → parse → execute → render → print) with no business logic.
2. **Error handling**: All errors are rendered consistently with `ERR` prefix. Parse/contract errors use `ERR PARSE`.
3. **Script testing**: `run_script()` mimics the main loop, enabling integration testing without stdin/stdout.
4. **Response rendering**: Dedicated `render()` (and `render_error()` if present) keeps formatting isolated and stable.
5. **No unwrap in production**: Removed unwraps from runtime path; errors always propagate as `Result`.
6. **Command parsing via FromStr**: Makes parsing composable, testable, and consistent across types.
7. **Exit handling**: Exit stops the loop immediately after rendering response.
8. **Empty line handling**: Empty lines are ignored, matching typical CLI behavior.
9. **Error propagation boundary**: Domain errors are converted to CLI errors (clean boundary).
10. **Test organization**: Unit tests live near modules; script tests live in `tests/`.

---


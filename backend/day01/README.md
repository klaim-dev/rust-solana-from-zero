# Day 01 — Axum Hello World

**Routes · JSON · Healthcheck · Contract Tests**

Minimal but **production-correct** Axum service skeleton: routing, JSON responses, healthcheck, and **contract integration tests** via `oneshot` (no TCP server in tests).

> **Day 1 canon:** no request-id, no unified error contract yet.
> Those are introduced in **Day 2 (extractors + validation + error mapping)** and
> **Day 3 (middleware / request-id / tracing spans)**.

---

## 🎯 Goal of the Day

Build a **stable foundation** for the next 39 backend days:

* service boots via Tokio,
* routing is explicit and testable,
* `GET /healthz` returns typed JSON,
* behavior is fixed with **contract tests**, not “it works in browser”.

This project is the **anchor point** for the entire backend track.

---

## ✅ API Contract (Day 1)

### `GET /healthz`

* **200 OK**
* `Content-Type: application/json…`
* body: `{ "ok": true }`

### Unknown path (e.g. `GET /nope`)

* **404 Not Found** (Axum default)
* unified JSON error contract starts in **Day 2**

---

## 🧱 Invariants (held for all 40 days)

* ❌ No `unwrap` / `expect` in **production paths**
* ✅ Handlers are thin and return **typed values**, not manual HTTP responses
* ✅ Router is built in `build_router()` (single source of truth)
* ✅ `main.rs` is lifecycle glue only
* ✅ Project structure is **production-ready**, not “everything in main”

---

## 📁 Project Structure

Representative structure (minor variations are fine):

```
src/
  main.rs            # bootstrap only (bind + serve)
  lib.rs             # exports for integration tests
  app.rs             # build_router()
  routes/
    mod.rs
    healthz.rs       # handler + response type

tests/
  http_contract.rs   # contract tests via oneshot
```

---

## 🚀 Run the Service

### Start locally

```bash
cargo run
```

Default address: `127.0.0.1:3000`
(configuration will be introduced on Day 4).

### Check health

```bash
curl -v http://127.0.0.1:3000/healthz
```

Expected:

* `HTTP/1.1 200 OK`
* `content-type: application/json`
* body: `{"ok":true}`

---

## 🧪 Tests

We test the **Router as a Service**, not a real TCP server.

```bash
cargo test
```

### What is covered

* `/healthz`

  * `StatusCode::OK`
  * `Content-Type` starts with `application/json`
  * JSON **shape** contains `"ok": true`
* `/nope`

  * `StatusCode::NOT_FOUND`

### Why `oneshot`

* no ports
* no race conditions
* deterministic, fast tests
* verifies **HTTP behavior**, not environment

This is **contract testing**, not manual verification.

---

## 🧠 Technical Decisions (Decision Log)

* **Why `build_router()`?**
  The router *is* the API contract.
  It must be reusable by `main` and tests without spinning up TCP or duplicating setup.

* **Why `oneshot` instead of running a server in tests?**
  We test the Axum/Tower pipeline (`Request → Service → Response`) directly as a future.
  Faster, cleaner, and more precise.

* **Why check `Content-Type` with `starts_with("application/json")`?**
  Real responses may include `charset=utf-8`.
  The contract is “this is JSON”, not an exact header string.

* **Why JSON shape via `serde_json::Value` instead of deserializing into a struct?**
  Tests should validate **external API behavior**, not internal Rust models.
  This keeps tests stable as implementation evolves.

---

## 🧾 Coverage Contract

| Area          | Location             | Verification             |
| ------------- | -------------------- | ------------------------ |
| Routing       | `app.rs`, `routes/*` | integration tests        |
| JSON response | `routes/healthz.rs`  | status + header + shape  |
| Bootstrap     | `main.rs`            | `cargo run` + `curl`     |
| 404 behavior  | Axum default routing | `/nope` integration test |

---

## ✅ Definition of Done (Senior Level)

* ✅ `GET /healthz` → `200` + `{ "ok": true }`
* ✅ `build_router()` used by both `main` and tests
* ✅ contract integration test for `/healthz`
* ✅ negative test for unknown route (`404`)
* ✅ no `unwrap/expect` in production code
* ✅ non-monolithic project structure

---

## ➡️ What’s Next (Day 2)

Day 2 moves from “alive” to **correct API behavior**:

* request extractors (`Path`, `Query`, `Json`)
* input validation
* unified error type
* error → HTTP mapping
* contract tests for failures and edge cases

---

## 🛠️ Useful Commands

```bash
cargo fmt
cargo clippy -D warnings
cargo test
cargo run
```

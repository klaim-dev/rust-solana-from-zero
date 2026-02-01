# Backend Day 2 — API Contracts: Extractors, Validation, Error Mapping (Axum)

**Day 2 focus:**  
Build a **strict, explicit API contract** on top of Axum using typed extractors, domain validation, and a single unified error model mapped to HTTP.

This day turns a “working service” into a **predictable API**:
- typed input only (Path / Query / Json),
- clear separation between parsing errors and domain validation,
- **one JSON error format for all failures**,
- behavior fixed by contract tests, not by manual checks.

This foundation is designed to scale for the next 38 days without rewriting core logic.

---

## 🎯 Goal of the Day

Extend the Day 1 Axum skeleton so that:

- All inputs are accepted via **typed extractors**
- Extractor failures are **captured and mapped**, not leaked
- Validation errors return **422**, not generic 400
- All errors go through a **single AppError → HTTP mapping**
- Even `404` is returned as **JSON**
- Handlers remain thin and predictable
- API behavior is fixed by **contract tests**

---

## 🧱 Architecture Overview

Clean separation of responsibilities:

```

src/
main.rs
infra/
http.rs          # Router wiring, fallback
app/
error.rs         # AppError, ErrorCode, IntoResponse
handlers/
users.rs       # /users/:id
search.rs      # /search
echo.rs        # /echo
domain/
error.rs         # DomainError (no HTTP knowledge)
validation.rs    # Business rules
tests/
http_contract.rs   # API contract tests

````

### Layer responsibilities

| Layer   | Responsibility |
|--------|----------------|
| domain | Business rules and domain errors |
| app    | Mapping domain errors to HTTP (presentation) |
| infra  | Router wiring and transport concerns |

---

## ❗ Error Contract (Core of Day 2)

### Unified JSON error format

Every error response has **exactly one shape**:

```json
{
  "code": "not_found",
  "message": "user not found"
}
````

### Error codes table

| HTTP | code              | Meaning                     |
| ---- | ----------------- | --------------------------- |
| 400  | bad_request       | Input could not be parsed   |
| 401  | unauthorized      | Reserved                    |
| 403  | forbidden         | Reserved                    |
| 404  | not_found         | Resource not found          |
| 409  | conflict          | Conflict with current state |
| 422  | unprocessable     | Validation failed           |
| 429  | too_many_requests | Reserved                    |
| 500  | internal          | Internal server error       |

> **Important rule:**
>
> * **400** → extractor / parsing failure
> * **422** → parsed correctly, but domain rules violated

---

## 🔑 Key Concepts Implemented

### 1. Typed Extractors

* `Path<u64>` for route parameters
* `Query<T>` for query strings
* `Json<T>` for request bodies

Handlers never deal with raw strings.

---

### 2. Extractor Rejection Handling

Handlers accept extractors as:

```rust
Result<Extractor, Rejection>
```

This allows converting extractor failures into **our own AppError**, instead of leaking Axum defaults.

---

### 3. Domain Validation

Business rules live in `domain/` and return `DomainError`:

* `Validation`
* `NotFound`
* `Conflict`

The domain layer has **no knowledge of HTTP**.

---

### 4. Error Mapping

`impl From<DomainError> for AppError` defines how domain failures are presented over HTTP:

| DomainError | AppError      | HTTP |
| ----------- | ------------- | ---- |
| Validation  | Unprocessable | 422  |
| NotFound    | NotFound      | 404  |
| Conflict    | Conflict      | 409  |

---

### 5. Thin Handlers

Handlers return:

```rust
Result<Json<T>, AppError>
```

They:

* extract typed input,
* call domain logic,
* return values or errors.

No manual response building.

---

## 📡 Implemented Endpoints

### `GET /healthz`

Health check.

**Response**

```json
{ "ok": true }
```

---

### `GET /users/:id`

| Case       | Response          |
| ---------- | ----------------- |
| `/users/1` | `200 { "id": 1 }` |
| `/users/0` | `404 not_found`   |

---

### `GET /search?limit=&q=`

| Case            | Response                       |
| --------------- | ------------------------------ |
| `limit=10&q=hi` | `200 { "limit":10, "q":"hi" }` |
| `limit=ABC`     | `400 bad_request`              |
| `limit=0`       | `422 unprocessable`            |

---

### `POST /echo`

| Case                      | Response          |
| ------------------------- | ----------------- |
| valid JSON                | `200 echo`        |
| invalid JSON              | `400 bad_request` |
| `email=taken@example.com` | `409 conflict`    |
| `email=fail@example.com`  | `500 internal`    |

---

## 🧪 Testing Strategy

All behavior is fixed using **contract tests** with `oneshot`:

* no real TCP
* no mocking HTTP manually
* router tested as a pure service

### Covered cases

* Happy paths
* Extractor parse errors (400)
* Validation errors (422)
* Conflict (409)
* Internal error (500)
* Fallback 404 JSON

Tests define **what the API guarantees**, not how it is implemented.

---

## ✅ Day 2 Definition of Done

* ❌ No `unwrap` / `expect` in production paths
* ✅ Typed extractors everywhere
* ✅ 400 vs 422 clearly separated
* ✅ Single `AppError` + `IntoResponse`
* ✅ JSON fallback for 404
* ✅ ≥3 negative test cases (actual: more)
* ✅ Clean domain / app / infra boundaries

---

## 🧠 Why This Matters

This day locks in:

* predictable error handling,
* stable client contracts,
* freedom to refactor internals without breaking APIs.

Everything built later (auth, middleware, DB, async flows) will **stand on this foundation**.

---

## 🔜 Next Day

**Day 3 — Middleware, request-id, tracing spans**

From here on, every request will be:

* traceable,
* observable,
* correlated across logs.
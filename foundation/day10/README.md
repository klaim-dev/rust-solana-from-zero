# Day 10 — Mini User Registration Model

**Solution for Foundation Day 10**

This project implements a small, consistent domain around `User` and an in-memory `UserRegistry`, demonstrating:
- `struct` + `impl` + methods
- `Result` + `thiserror` for domain errors
- `Option` for search results
- Honest handling of duplicates (id/email)

---

## 📂 Project Structure

The project is organized into a production-ready module structure:

- **`src/user.rs`**: Defines the `User` entity and `UserError`.
- **`src/registry.rs`**: Implements the `UserRegistry` and `RegistrationError`.
- **`src/lib.rs`**: Exports the modules.
- **`src/main.rs`**: Entry point demonstrating usage.

## 🛠 Features

### User Domain (`User`)
- **Invariants**:
  - `id` must be positive.
  - `email` must be valid (non-empty, contains '@', no spaces).
  - `age` must be between 0 and 120.
- **Behavior**:
  - `is_adult()` check.
  - `activate()` / `deactivate()` state management.

### Registry (`UserRegistry`)
- **Storage**: In-memory `Vec<User>`.
- **Registration**:
  - Validates user data.
  - **Prevents duplicates**: Checks for existing `id` or `email` before insertion.
  - Returns `Result<&User, RegistrationError>`.
- **Search**:
  - `find_by_id` -> `Option<&User>`
  - `find_by_email` -> `Option<&User>`

## 🚀 Usage

Run the main program:
```bash
cargo run
```

Run the tests:
```bash
cargo test
```

---

## 🧠 Theory & Concepts

### 1. `struct + impl` = Domain Capsule
The `User` struct protects its state through private fields and a constructor `new()` that enforces invariants. If you hold a `User`, it is guaranteed to be valid.

### 2. `Result` vs `Option`
- **`Result`**: Used for operations that can fail where the reason matters (e.g., registration failed because of a duplicate ID).
- **`Option`**: Used for search operations where a value might simply be absent (e.g., user not found).

### 3. Error Handling
We use `thiserror` to define clear, typed errors:
- `UserError`: Validation failures (InvalidId, InvalidEmail, etc.).
- `RegistrationError`: Registry failures (DuplicateId, DuplicateEmail) and wraps `UserError`.

---

## ✅ Task Checklist (Completed)

- [x] **User Domain**: `struct` + `impl` with validation.
- [x] **Registry**: `new`, `register`, `find_by_id`, `find_by_email`.
- [x] **Error Handling**: Custom enums with `thiserror`.
- [x] **Tests**: Comprehensive unit tests for happy paths and edge cases.
- [x] **No Panics**: Production code uses `Result`/`Option` instead of `unwrap`.

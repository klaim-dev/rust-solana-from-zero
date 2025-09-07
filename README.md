<div align="center">
  
# 🦀⚡ Rust + Solana: From Zero  
**A public, day‑by‑day engineering log.**  
From “hello, Rust” → production‑grade backends → on‑chain programs on Solana.

[![GitHub Stars](https://img.shields.io/github/stars/klaim-dev/rust-solana-from-zero?style=flat&color=ffd166)](https://github.com/klaim-dev/rust-solana-from-zero/stargazers)
[![Last Commit](https://img.shields.io/github/last-commit/klaim-dev/rust-solana-from-zero?color=06d6a0)](https://github.com/klaim-dev/rust-solana-from-zero/commits/main)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](/LICENSE)
[![Rust](https://img.shields.io/badge/rust-stable-orange)](https://www.rust-lang.org/)
[![Built with grit](https://img.shields.io/badge/built%20with-grit-8a2be2)](#)

</div>

---

## TL;DR
This repo is a **transparent, daily DevLog**. Each day lives in `rust/foundation/dayXX` as a **self‑contained Cargo project** with tests.  
Commits read like a journal: `day05: borrowing, &T/&mut T, slices, split_at_mut`.

The aim: **strong Rust fundamentals → pragmatic backend → real Solana programs**. No fluff, just reps.

---

## Repo Map

```
rust-solana-from-zero/
├── LICENSE
├── README.md
└── rust/
    └── foundation/
        ├── day01/  # variables, String/&str, stack vs heap
        ├── day02/  # arithmetic, logic, if/else
        ├── day03/  # functions, params, tuples, return
        ├── day04/  # ownership: move/clone/Copy
        └── day05/  # borrowing: &T/&mut T, slices, split_at_mut
```

> Every `dayXX` is a tiny **binary** crate with `Cargo.toml`, `src/`, and tests. No nested git repos — one clean monorepo.

## Live Progress

**Foundation (30 days)**  
```
[######------------------------] 6 / 30
```
- ✅ [day01](rust/foundation/day01) — variables, String/&str, stack vs heap  
- ✅ [day02](rust/foundation/day02) — arithmetic, logic, if/else  
- ✅ [day03](rust/foundation/day03) — functions, params, tuples, return  
- ✅ [day04](rust/foundation/day04) — ownership, move/clone/Copy  
- ✅ [day05](rust/foundation/day05) — borrowing, &T/&mut T, slices, split_at_mut  
- ✅ [day06](rust/foundation/day06) — Result/Option, ?, domain error enum, checked arithmetic  
- ✅ [day07](rust/foundation/day06) — parsing layer, Result/?/ok_or/map_err, thiserror + Display  
- ⏳ day08 — enums & pattern matching (API design)  
- ⏳ day09 — impl, methods, ergonomics & autoderef  
- … up to day30

_DevLog rule: one day → one commit (plus tiny follow‑ups if needed)._

---

## Design Principles

- **Small scopes, real reps.** Every day ships something runnable + testable.  
- **Honest code.** Clear invariants, explicit ownership/borrows, tiny helpers over “magic”.  
- **Library‑last.** Standard library first; crates only when they clarify the learning.  
- **No panic in engines.** Starting day06–07: model errors with `Result` instead of `unwrap/expect`.  
- **Safety is a feature.** Borrow checker, tests, and invariants are guardrails, not friction.

---

## Daily Playbook

1. Create a new folder: `rust/foundation/dayXX/`  
2. Implement the day’s topic + a few **unit tests**  
3. Commit with a clear message:  
   ```
   day06: Option, match, if let
   ```
4. Push to `main`

Example:

```bash
git add rust/foundation/day06
git commit -m "day06: Option, match, if let"
git push
```

---

## Roadmap (condensed)

1) **Rust Foundation (30 days)** — memory, ownership, modules, testing  
2) **Rust Backend (40 days)** — Axum, SQLx/PostgreSQL, layered architecture, CI/CD  
3) **Advanced Rust (50 days)** — lifetimes, async, concurrency, safe abstractions  
4) **Solana (60–75 days)** — Anchor, PDAs, CPIs, program security, backend integration

> Milestones will appear as new top‑level folders (e.g. `rust/backend`, `solana/anchor-apps`).

---

## Quality & House Rules

- Keep `Cargo.lock` committed (these are app‑like crates).  
- `.gitignore` is tuned for Rust workspaces, wasm, JS tooling, IDEs, and coverage junk.  
- No secrets in the repo — use `.env.example` & local env vars.  
- One monorepo. No nested `.git` in day folders.

---

## FAQ

**Why split by days?**  
To keep momentum and make progress auditable. Each day is a scoped, testable slice.

**Can I reuse or learn from this?**  
Yes. Fork it, adapt, or open issues/PRs if you spot improvements.

**Where are the “real” projects?**  
After Foundation, you’ll see `backend/` and `solana/` with production‑style services and on‑chain programs.

---

## Follow

- GitHub — https://github.com/klaim-dev  
- X — https://x.com/klaimdev

---

## License

MIT — see [LICENSE](/LICENSE).

<div align="center">

# 🦀⚡ Rust + Solana: From Zero  
**A public, day-by-day engineering log.**  
From “hello, Rust” → production-grade backends → on-chain programs on Solana.

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](/LICENSE)
[![Rust](https://img.shields.io/badge/rust-stable-orange)](https://www.rust-lang.org/)
[![Code > Comfort](https://img.shields.io/badge/code%20%3E%20comfort-8a2be2)](#)

</div>

---

## TL;DR

One monorepo. Four phases.  
Every day is a small, verifiable unit of work:

1. 🦀 **Rust Foundation** — 30 days  
2. 🖧 **Rust Backend** — 40 days  
3. ⚙️ **Advanced Rust** — 50 days  
4. 🪙 **Blockchain / Solana** — 60–75 days  

Each `dayXX`:

- lives in its own folder,
- is a self-contained Cargo project,
- has its own `README.md` as a daily DevLog:
  focus → tasks → invariants → tests → decision log.

**Rule:** if there’s a folder — there’s real code.

---

## Phase Specs

This repo is driven by explicit specs for each phase:

| Phase | Spec |
|-------|------|
| 🦀 Rust Foundation | [`foundation/README.md`](./foundation/README.md) |
| 🖧 Rust Backend | [`backend/README.md`](./backend/README.md) |
| ⚙️ Advanced Rust | [`deep-dive/README.md`](./deep-dive/README.md) |
| 🪙 Blockchain / Solana | [`blockchain/README.md`](./blockchain/README.md) |

Each spec is a contract: blocks, days, Go/No-Go criteria, required artifacts.

Active phases will also have matching branches:
`foundation`, `backend`, `deep-dive`, `blockchain`.

---

## DevLog Rules

- One day → one meaningful commit (plus tiny fixes).
- No fake progress. No empty day folders “for looks”.
- No `unwrap` / `expect` on production paths.
- Negative and edge-case tests are mandatory.
- Clean boundaries: domain / app / infra don’t leak into each other.
- English-first docs so any engineer can follow.
- Everything must be reproducible: clone → build → test.

This repo is for:
- future me,
- reviewers / leads,
- anyone who wants to see a real Rust → Solana path without marketing noise.

---

## License

MIT — see [LICENSE](/LICENSE).


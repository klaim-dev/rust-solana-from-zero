# 🖧 Rust Backend — 40 Days (vFinal)

**Live Progress:** `Day 0 / 40`  
`[----------]` (0%)

_Updated manually. No fake numbers._

## Stack & Principles

**Default stack:** Axum + SQLx (async, Postgres) + tracing + OpenTelemetry.  
Diesel is allowed only via `spawn_blocking + r2d2`. The primary target: **SQLx**.

**Weekly ritual (every Friday):**
- Review: debt, metrics, performance.
- 10-line decision log (what changed, why, next).
- Fix p50 / p95 / p99 graphs and keep them in the repo.

---

## 🔧 Block 1 (Days 1–10): Architecture & Axum

**Day 1.** Axum hello-world, routes, JSON  
_Artifact:_ `backend/axum_hello/` (healthcheck `/healthz`).

**Day 2.** Extractors, validation, error mapping → HTTP  
_Artifact:_ `AppError` + status map (400/401/403/404/409/422/429/500).

**Day 3.** Middleware/Tower, request-id, tracing spans  
_Artifact:_ request-id based log correlation, sane log levels.

**Day 4.** State sharing: `Arc`, config, pools  
_Artifact:_ `AppState` + `Config` (`dotenvy`, required vars).

**Day 5.** Layers: domain / app / infra  
_Artifact:_ module diagram, folder skeleton, service interfaces.

**Day 6.** CRUD #1 (User): DTO, service, handlers  
_Artifact:_ endpoints + unit tests for service.

**Day 7.** Errors: `thiserror`/`anyhow`, readable `Display`  
_Artifact:_ unified error type, HTTP mapping.

**Day 8.** Integration tests (in-memory, no DB)  
_Artifact:_ `cargo nextest` setup, isolated tests.

**Day 9.** Architecture refactor  
_Artifact:_ PR-style cleanup (boundaries, names, contracts).

**Day 10.** Baseline bench (k6/hey), latency report  
_Target:_ p50 < 15 ms (local), logs correlate via request-id.

**Go/No-Go Block 1:**  
Skeleton API is stable, healthcheck works, AppError + status map ready, tracing wired, first latency numbers recorded.

---

## 🐘 Block 2 (Days 11–20): Database (SQLx) & Business Logic

**Day 11.** SQLx, migrations, docker-postgres  
_Artifact:_ `docker-compose.yml`, `sqlx migrate` (up/down), `DATABASE_URL` in config.

**Day 12.** Query patterns: filters, N+1 guard, `EXPLAIN/ANALYZE`  
_Artifact:_ EXPLAIN report, indexes plan, N+1 eliminated (JOIN/CTE).

**Day 13.** Models, `From`/`Into`/`TryFrom`  
_Artifact:_ clear DTO ↔ DB models, validated conversions.

**Day 14.** Transactions, isolation, idempotency  
_Artifact:_ idempotency-key pattern + rollback tests.

**Day 15.** Axum + SQLx integration: pool & timeouts  
_Artifact:_ tuned pool config, per-query timeouts, safe retries.

**Day 16.** Business logic (Users + Posts), service tests  
_Artifact:_ happy/edge cases, 409 conflict, 404 not found.

**Day 17.** Relations & pagination  
_Artifact:_ shared pagination (limit/offset/next-cursor), sorting.

**Day 18.** Indexes: B-tree, partial, composite, covering  
_Artifact:_ index migrations + before/after metrics.

**Day 19.** Integration tests with real DB  
_Artifact:_ fixtures, transactional tests.

**Day 20.** Review + latency p50/p95  
_Target:_ p50 < 20 ms, p95 < 50 ms (dev), charts committed.

**Go/No-Go Block 2:**  
Migrations & rollbacks safe, critical queries indexed, idempotency in place, N+1 removed, latency targets met.

---

## 🔐 Block 3 (Days 21–28): Security & Auth

**Day 21.** Passwords: argon2 + pepper  
_Artifact:_ `auth::password`, safety & perf tests.

**Day 22.** JWT: access/refresh, rotation, `aud/iss/exp`, skew  
_Artifact:_ JWT middleware, refresh flow, blacklist/allowlist strategy.

**Day 23.** RBAC: roles + user context  
_Artifact:_ protected routes, tests for roles.

**Day 24.** Input validation, XSS/SQLi defense  
_Artifact:_ central validator, negative tests.

**Day 25.** Rate limiting (IP + account), backoff  
_Artifact:_ token/leaky bucket middleware, 429 metrics.

**Day 26.** CORS/CSRF, cookies/sessions  
_Artifact:_ CORS/CSRF policy, secure cookies (if used).

**Day 27.** Security logging & alerts  
_Artifact:_ audit log for auth, alert on spikes.

**Day 28.** Security checklist (12 pts) + tests  
_Artifact:_ `security.md`, all green.

**Go/No-Go Block 3:**  
Auth stable, rate limiting on, security checklist implemented & enforced by tests.

---

## ⚙️ Block 4 (Days 29–36): Async, Observability, DevOps

**Day 29.** Timeouts, cancellation, backpressure  
_Artifact:_ timeout map, bounded queues, retry policy.

**Day 30.** Redis: cache/sessions/pubsub (optional)  
_Artifact:_ Redis module with timeouts & retries.

**Day 31.** Dockerfile (multi-stage), `docker-compose`  
_Artifact:_ lean images, one-command local stack.

**Day 32.** `Makefile` / `justfile`  
_Artifact:_ `dev/test/bench/deploy` commands, `.env.example`.

**Day 33.** GitHub Actions CI  
_Artifact:_ `.github/workflows/ci.yml`, cached deps.

**Day 34.** Deploy target (Fly.io/Render/DO/…)  
_Artifact:_ deploy manifests, secrets management.

**Day 35.** Observability: OTel traces + metrics  
_Artifact:_ OTLP exporter, dashboards: p50/p95/p99, error-rate.

**Day 36.** Production hardening  
_Artifact:_ non-root, health probes, CPU/RAM limits, graceful shutdown.

**Go/No-Go Block 4:**  
CI green, one-command deploy exists, traces+metrics online, health checks solid.

---

## 📦 Block 5 (Days 37–40): Finalization

**Day 37.** Domain/app/infra boundaries final  
_Artifact:_ final diagram, explicit contracts.

**Day 38.** Webhooks & external APIs, retries + idempotency  
_Artifact:_ webhook handler template, retry matrix.

**Day 39.** OpenAPI/Redoc, docs, examples  
_Artifact:_ OpenAPI spec, usage README.

**Day 40.** Production rehearsal: release, rollback, runbook  
_Artifact:_ release checklist, rollback plan, SLA/SLO targets.

---

## Exit Criteria (Before Advanced & Solana)

- ✅ Starter backend repo template:
  Axum, SQLx, AppError, AppState, config, tests, CI, Docker, observability.
- ✅ Latency budget defined & respected for core endpoints.
- ✅ Idempotency for external calls & DB transactions.
- ✅ Timeouts, retries, backpressure, health checks, graceful shutdown.
- ✅ Security checklist passed; audit logs & alerts in place.
- ✅ Docs: OpenAPI, README, runbook, migrations (up/down) are clear.



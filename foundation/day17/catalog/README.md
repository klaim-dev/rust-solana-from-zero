# Day 16 — In‑Memory Catalog (Repository Layer)

This mini‑project implements an **in‑memory catalog repository** with full CRUD support, a **secondary index**, and an explicit **anti‑N+1 contract**.
The goal of Day 16 is not raw algorithmic complexity, but **layering, ownership, invariants, and repository semantics** — exactly how a real backend data layer is designed.

---

## 🎯 Goal of the Day

Build a production‑style **repository layer** (still pure Rust + std) that:

* Owns and manages domain entities (`Item`)
* Enforces domain invariants (unique SKU, non‑empty name, etc.)
* Maintains **primary and secondary indexes** consistently
* Provides **atomic updates** (no partial state on error)
* Guarantees **anti‑N+1 behavior by design**

This repository is intentionally designed to be **1‑to‑1 portable** to a SQLx/Postgres implementation later.

---

## 🧱 Architecture Overview

```
domain/
  item.rs        // Item, ItemId, Sku, Category, CreateItem, UpdateItem, Filter
  errors.rs      // CatalogError (domain error contract)

store/
  catalog.rs     // Catalog repository (CRUD + indexes)
```

**Layering rules:**

* `domain` contains **pure domain types and invariants**
* `store::Catalog` is the **single owner of data**
* Domain types have no knowledge of storage or indexes
* All consistency rules live in the repository layer

---

## 🗂 Storage Model

* **Primary storage**: `HashMap<ItemId, Item>`
* **Secondary index**: `HashMap<Sku, ItemId>`

This enables:

* O(1) lookup by ID
* O(1) lookup by SKU
* Clean separation between entity storage and lookup strategy

---

## 📋 Coverage Matrix

| Feature                         | Implementation             | Test Coverage                          |
| ------------------------------- | -------------------------- | -------------------------------------- |
| **CRUD Operations**             |                            |                                        |
| Create item                     | `Catalog::create_item()`   | `test_create_and_get_by_id_and_sku`    |
| Read by ID                      | `Catalog::get_by_id()`     | `test_create_and_get_by_id_and_sku`    |
| Read by SKU                     | `Catalog::get_by_sku()`    | `test_create_and_get_by_id_and_sku`    |
| Update item                     | `Catalog::update_item()`   | `test_update_item_*`                   |
| Delete item                     | `Catalog::delete_item()`   | `test_delete_removes_secondary_index`  |
| **Update Semantics**            |                            |                                        |
| Partial updates via `Option<T>` | `UpdateItem`               | `test_update_item_all_fields`          |
| SKU rebinding                   | `update_item` (sku branch) | `test_update_item_sku_rebinding`       |
| SKU collision handling          | `update_item`              | `test_update_item_sku_collision_error` |
| Empty name validation           | `update_item`              | `test_update_item_empty_name_error`    |
| **List / Query**                |                            |                                        |
| Category filter                 | `list_items()`             | `test_list_items_filter_category`      |
| Active‑only filter              | `list_items()`             | `test_list_items_filter_active_only`   |
| Price range filter              | `list_items()`             | `test_list_items_filter_price_range`   |
| Name substring filter           | `list_items()`             | `test_list_items_filter_name_contains` |
| Sorting (price + id)            | `list_items()`             | `test_list_items_sorting_stable`       |
| Anti‑N+1 guarantee              | `list_items()`             | `test_list_items_anti_n1_architecture` |
| **Secondary Index**             |                            |                                        |
| SKU → ID mapping                | `Catalog::id_by_sku`       | `test_update_item_sku_rebinding`       |
| Index consistency               | atomic update/delete       | `test_update_item_sku_collision_error` |

---

## 🔐 Invariants Enforced

* `ItemId` is generated internally by the repository
* `Sku` is normalized and unique
* `name` must be non‑empty after trim
* Updates are **atomic**: on error, state is unchanged
* Primary and secondary indexes are always consistent

---

## 🚫 Anti‑N+1 Contract

`list_items()`:

* Iterates **once** over `items_by_id.values()`
* Applies filters in memory
* Sorts the collected slice
* **Never calls `get_by_id()` in a loop**

This is verified by an explicit test using a call counter.

> This mirrors a real backend guarantee: *"this endpoint performs exactly one data access"*.

---

## 🧪 Testing Strategy

* Happy‑path CRUD tests
* Negative tests (SKU collision, empty name, missing item)
* State‑invariance tests (failed update leaves storage unchanged)
* Architectural test for anti‑N+1 behavior

Total: **comprehensive repository‑level coverage**, similar to real backend unit tests.

---

## 🧠 Decision Log

**Why in‑memory?**
Allows focusing on **architecture and invariants** without IO noise. The API is intentionally shaped to match a future SQLx repository.

**Why a secondary index?**
Fast SKU lookup without scanning. Mirrors a `UNIQUE INDEX` in SQL.

**Why atomic rebinding on SKU update?**
Prevents stale or split‑brain state between primary storage and secondary index.

**Why `Option<T>` for updates?**
Models partial updates explicitly and avoids invalid “default overwrite” behavior.

**Why anti‑N+1 by construction?**
This is not an optimization — it is a **contract**. The API shape makes N+1 impossible.

---

## 🔜 Portability to SQLx

This repository maps directly to SQL:

* `create_item` → `INSERT`
* `get_by_id / get_by_sku` → `SELECT ... WHERE`
* `update_item` → `UPDATE` (transactional for SKU changes)
* `list_items` → `SELECT ... WHERE ... ORDER BY`

No domain or API changes required.

---

## ✅ Day 16 Result

A **production‑style repository layer** with:

* Clear ownership boundaries
* Explicit invariants
* Secondary index management
* Anti‑N+1 guarantees
* Strong test coverage

This is the foundation for the upcoming CLI and backend stages.

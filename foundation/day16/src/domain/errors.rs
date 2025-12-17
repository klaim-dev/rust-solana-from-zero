use thiserror::Error;

use super::item::{ItemId, Sku};

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum CatalogError {
    // -------------------------
    // Input / validation
    // -------------------------
    #[error("invalid item id (must be non-zero)")]
    InvalidItemId,

    #[error("sku is empty (after trim)")]
    EmptySku,

    #[error("name is empty (after trim)")]
    EmptyName,

    /// Optional but recommended (if Filter has min/max)
    #[error("invalid price range: min ({min}) must be <= max ({max})")]
    InvalidPriceRange { min: u64, max: u64 },

    // -------------------------
    // CRUD / consistency
    // -------------------------
    #[error("duplicate sku: {sku}")]
    DuplicateSku { sku: Sku },

    #[error("duplicate id: {id}")]
    DuplicateId { id: ItemId },

    #[error("item not found: id={id}")]
    ItemNotFound { id: ItemId },

    /// Optional: only if absence should be an error
    #[error("item not found: sku={sku}")]
    SkuNotFound { sku: Sku },

    // -------------------------
    // Update-specific
    // -------------------------
    #[error("sku collision: sku={sku} is already used by another item")]
    SkuCollision { sku: Sku },

    // -------------------------
    // Internal consistency
    // -------------------------
    #[error("internal invariant violation: duplicate sku in index during update")]
    InvariantViolation,
}

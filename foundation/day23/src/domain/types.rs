use crate::domain::error::{ItemErr, SkuErr};
use std::fmt::{self};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortSpec {
    PriceDescNameAsc,
    NameAsc,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ItemId(u64);

impl fmt::Display for ItemId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ItemId {
    pub fn new(id: u64) -> Self {
        Self(id)
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Sku(String);
impl Sku {
    pub fn try_new(input: &str) -> Result<Sku, SkuErr> {
        let normalized = input.trim().to_lowercase();
        if normalized.is_empty() {
            return Err(SkuErr::InvalidSku);
        }
        Ok(Sku(normalized))
    }
}
impl fmt::Display for Sku {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Item {
    id: ItemId,
    sku: Sku,
    name: String,
    price_cents: u64,
}

impl Item {
    pub fn try_new(id: ItemId, sku: Sku, name: &str, price_cents: u64) -> Result<Self, ItemErr> {
        let norm_name = name.trim();
        if norm_name.is_empty() {
            return Err(ItemErr::InvalidName);
        }
        Ok(Self {
            id,
            sku,
            name: norm_name.to_string(),
            price_cents,
        })
    }

    pub fn get_price_cents(&self) -> u64 {
        self.price_cents
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_id(&self) -> ItemId {
        self.id
    }

    pub fn get_sku(&self) -> &Sku {
        &self.sku
    }
}

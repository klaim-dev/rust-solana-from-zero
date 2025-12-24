use crate::domain::errors::CatalogError;
use std::{fmt, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ItemId(u64);

impl fmt::Display for ItemId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for ItemId {
    type Err = CatalogError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let raw = s.trim();
        let parsed = raw
            .parse::<u64>()
            .map_err(|_| CatalogError::InvalidItemId)?;
        ItemId::new(parsed)
    }
}

impl ItemId {
    pub fn new(raw: u64) -> Result<ItemId, CatalogError> {
        if raw == 0 {
            return Err(CatalogError::InvalidItemId);
        }
        Ok(ItemId(raw))
    }

    pub fn as_u64(&self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Sku(String);

impl fmt::Display for Sku {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Sku {
    pub fn new(raw: String) -> Result<Sku, CatalogError> {
        let sku = raw.trim().to_ascii_lowercase();
        if sku.is_empty() {
            return Err(CatalogError::EmptySku);
        }
        Ok(Sku(sku))
    }
}

impl FromStr for Sku {
    type Err = CatalogError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Sku::new(s.to_string())
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Category {
    Books,
    Electronics,
    Grocery,
    Other,
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Category::Books => "books",
            Category::Electronics => "electronics",
            Category::Grocery => "grocery",
            Category::Other => "other",
        };
        f.write_str(s)
    }
}

impl FromStr for Category {
    type Err = CatalogError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let raw = s.trim().to_ascii_lowercase();
        match raw.as_str() {
            "books" | "book" => Ok(Category::Books),
            "electronics" | "electronic" => Ok(Category::Electronics),
            "food" | "grocery" => Ok(Category::Grocery),
            "other" => Ok(Category::Other),
            _ => Err(CatalogError::InvalidCategory),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Item {
    id: ItemId,
    sku: Sku,
    name: String,
    category: Category,
    price_cents: u64,
    is_active: bool,
}

impl Item {
    pub(crate) fn new(
        id: ItemId,
        sku: Sku,
        name: String,
        category: Category,
        price_cents: u64,
        is_active: bool,
    ) -> Self {
        Self {
            id,
            sku,
            name,
            category,
            price_cents,
            is_active,
        }
    }
    pub fn id(&self) -> ItemId {
        self.id
    }
    pub fn sku(&self) -> &Sku {
        &self.sku
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn category(&self) -> &Category {
        &self.category
    }
    pub fn price_cents(&self) -> u64 {
        self.price_cents
    }
    pub fn is_active(&self) -> bool {
        self.is_active
    }

    pub(crate) fn set_sku(&mut self, sku: Sku) {
        self.sku = sku;
    }

    pub(crate) fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub(crate) fn set_category(&mut self, category: Category) {
        self.category = category;
    }

    pub(crate) fn set_price_cents(&mut self, price_cents: u64) {
        self.price_cents = price_cents;
    }

    pub(crate) fn set_is_active(&mut self, is_active: bool) {
        self.is_active = is_active;
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateItem {
    pub sku: String,
    pub name: String,
    pub category: Category,
    pub price_cents: u64,
    pub is_active: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateItem {
    pub sku: Option<String>,
    pub name: Option<String>,
    pub category: Option<Category>,
    pub price_cents: Option<u64>,
    pub is_active: Option<bool>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Filter {
    pub category: Option<Category>,
    pub active_only: bool,
    pub price_min: Option<u64>,
    pub price_max: Option<u64>,
    pub name_contains: Option<String>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_str_item_id_happy_path() {
        assert_eq!(ItemId::from_str("1"), Ok(ItemId(1)));
        assert_eq!(ItemId::from_str(" 42 "), Ok(ItemId(42)));
    }

    #[test]
    fn from_str_item_id_zero() {
        assert_eq!(ItemId::from_str("0"), Err(CatalogError::InvalidItemId));
    }

    #[test]
    fn from_str_item_id_negative() {
        assert_eq!(ItemId::from_str("-1"), Err(CatalogError::InvalidItemId));
    }

    #[test]
    fn from_str_item_id_letters() {
        assert_eq!(ItemId::from_str("abc"), Err(CatalogError::InvalidItemId));
    }
}

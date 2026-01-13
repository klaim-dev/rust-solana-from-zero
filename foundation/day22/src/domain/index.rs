use crate::domain::error::SkuErr;
use crate::domain::sort::{NameKey, SortKey};
use crate::domain::types::{Item, ItemId, Sku, SortSpec};
use std::collections::HashMap;

#[derive(Debug, thiserror::Error)]
pub enum IndexError {
    #[error("duplicate id")]
    DuplicateId,
    #[error("duplicate sku: {input_sku}")]
    DuplicateSku { input_sku: Sku },
    #[error(transparent)]
    SkuErr(#[from] SkuErr),
}

pub struct InventoryIndex {
    by_id: HashMap<ItemId, Item>,
    by_sku: HashMap<Sku, ItemId>,
}

impl InventoryIndex {
    pub fn new() -> Self {
        Self {
            by_id: HashMap::new(),
            by_sku: HashMap::new(),
        }
    }
    pub fn insert(&mut self, item: Item) -> Result<(), IndexError> {
        let id = item.get_id();
        let sku = item.get_sku().clone();
        if self.by_id.contains_key(&id) {
            return Err(IndexError::DuplicateId);
        }

        if self.by_sku.contains_key(&sku) {
            return Err(IndexError::DuplicateSku {
                input_sku: sku.clone(),
            });
        }
        self.by_sku.insert(sku, id);
        self.by_id.insert(id, item);
        Ok(())
    }

    pub fn get_by_id(&self, id: ItemId) -> Option<&Item> {
        self.by_id.get(&id)
    }

    pub fn get_by_sku(&self, sku: &str) -> Option<&Item> {
        Sku::try_new(sku)
            .ok()
            .and_then(|sku| self.by_sku.get(&sku).and_then(|id| self.by_id.get(id)))
    }

    pub fn get_all_item(&self) -> Vec<Item> {
        self.by_id.values().cloned().collect::<Vec<_>>()
    }

    pub fn ids(&self) -> impl Iterator<Item = ItemId> + '_ {
        self.by_id.keys().copied()
    }

    pub fn list_sorted(&self, spec: SortSpec) -> Vec<&Item> {
        let mut items = self.by_id.values().collect::<Vec<_>>();
        match spec {
            SortSpec::PriceDescNameAsc => {
                items.sort_by_cached_key(|i| SortKey::from_item(i));
            }

            SortSpec::NameAsc => {
                items.sort_by_cached_key(|i| NameKey::from_item(i));
            }
        }
        items
    }
}

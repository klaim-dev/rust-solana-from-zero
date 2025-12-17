use std::collections::HashMap;

#[cfg(test)]
use std::cell::Cell;

use crate::domain::errors::CatalogError;
use crate::domain::item::{CreateItem, Filter, UpdateItem};
use crate::domain::item::{Item, ItemId, Sku};

pub struct Catalog {
    next_id: u64,
    items_by_id: HashMap<ItemId, Item>,
    id_by_sku: HashMap<Sku, ItemId>,
    #[cfg(test)]
    get_by_id_call_count: Cell<u32>,
}

impl Catalog {
    pub fn new() -> Self {
        Self {
            next_id: 1,
            items_by_id: HashMap::new(),
            id_by_sku: HashMap::new(),
            #[cfg(test)]
            get_by_id_call_count: Cell::new(0),
        }
    }

    // Test-only methods for verifying anti-N+1 guarantee
    pub fn get_by_id_call_count(&self) -> u32 {
        #[cfg(test)]
        {
            self.get_by_id_call_count.get()
        }
        #[cfg(not(test))]
        {
            0
        }
    }

    pub fn reset_get_by_id_call_count(&self) {
        #[cfg(test)]
        {
            self.get_by_id_call_count.set(0);
        }
    }

    pub fn create_item(&mut self, input: CreateItem) -> Result<ItemId, CatalogError> {
        let sku = Sku::new(input.sku)?;

        let name = input.name.trim();
        if name.is_empty() {
            return Err(CatalogError::EmptyName);
        }

        if self.id_by_sku.contains_key(&sku) {
            return Err(CatalogError::DuplicateSku { sku: sku.clone() });
        }

        let id = ItemId::new(self.next_id)?;
        self.next_id += 1;

        let item = Item::new(
            id,
            sku.clone(),
            name.to_string(),
            input.category,
            input.price_cents,
            input.is_active,
        );

        if self.items_by_id.insert(id, item).is_some() {
            return Err(CatalogError::DuplicateId { id });
        }

        if self.id_by_sku.insert(sku.clone(), id).is_some() {
            self.items_by_id.remove(&id);
            return Err(CatalogError::DuplicateSku { sku });
        }

        Ok(id)
    }

    pub fn get_by_id(&self, id: &ItemId) -> Option<&Item> {
        #[cfg(test)]
        {
            let count = self.get_by_id_call_count.get();
            self.get_by_id_call_count.set(count + 1);
        }
        self.items_by_id.get(id)
    }

    pub fn get_by_sku(&self, sku: &Sku) -> Option<&Item> {
        self.id_by_sku.get(sku).and_then(|id| self.get_by_id(id))
    }

    pub fn delete_item(&mut self, id: ItemId) -> Result<Item, CatalogError> {
        let item = self
            .items_by_id
            .remove(&id)
            .ok_or(CatalogError::ItemNotFound { id })?;
        self.id_by_sku.remove(item.sku());
        Ok(item)
    }

    pub fn update_item(&mut self, id: ItemId, patch: UpdateItem) -> Result<Item, CatalogError> {
        let item = self.items_by_id.get_mut(&id).ok_or(CatalogError::ItemNotFound { id })?;

        // Handle SKU update with index rebinding
        if let Some(raw) = patch.sku {
            let new_sku = Sku::new(raw)?;

            if &new_sku != item.sku() {
                match self.id_by_sku.get(&new_sku) {
                    Some(other_id) if *other_id != id => {
                        return Err(CatalogError::SkuCollision { sku: new_sku });
                    }
                    _ => {}
                }

                let old_sku = item.sku().clone();

                self.id_by_sku.remove(&old_sku);
                if self.id_by_sku.insert(new_sku.clone(), id).is_some() {
                    // This should never happen after remove, but check for invariant violation
                    return Err(CatalogError::InvariantViolation);
                }

                item.set_sku(new_sku);
            }
        }

        // Handle name update
        if let Some(raw_name) = patch.name {
            let name = raw_name.trim();
            if name.is_empty() {
                return Err(CatalogError::EmptyName);
            }
            item.set_name(name.to_string());
        }

        // Handle category update
        if let Some(category) = patch.category {
            item.set_category(category);
        }

        // Handle price_cents update
        if let Some(price_cents) = patch.price_cents {
            item.set_price_cents(price_cents);
        }

        // Handle is_active update
        if let Some(is_active) = patch.is_active {
            item.set_is_active(is_active);
        }

        Ok(item.clone())
    }

    pub fn list_items(&self, filter: Filter) -> Vec<&Item> {
        let mut items: Vec<&Item> = self.items_by_id.values().collect();

        // Normalize name_contains once before filtering
        let normalized_name_contains = filter.name_contains.as_ref().map(|s| s.to_lowercase());

        // Apply filters
        items.retain(|item| {
            // Filter by category
            if let Some(ref filter_category) = filter.category {
                if item.category() != filter_category {
                    return false;
                }
            }

            // Filter by active_only
            if filter.active_only && !item.is_active() {
                return false;
            }

            // Filter by price_min
            if let Some(price_min) = filter.price_min {
                if item.price_cents() < price_min {
                    return false;
                }
            }

            // Filter by price_max
            if let Some(price_max) = filter.price_max {
                if item.price_cents() > price_max {
                    return false;
                }
            }

            // Filter by name_contains (normalized once outside the loop)
            if let Some(ref normalized) = normalized_name_contains {
                if !item.name().to_lowercase().contains(normalized) {
                    return false;
                }
            }

            true
        });

        // Sort by price_cents (ascending), tie-breaker by id (ascending)
        items.sort_by(|a, b| {
            match a.price_cents().cmp(&b.price_cents()) {
                std::cmp::Ordering::Equal => a.id().as_u64().cmp(&b.id().as_u64()),
                other => other,
            }
        });

        items
    }
}

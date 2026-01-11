use crate::domain::types::{Item, ItemId};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) struct SortKey<'a> {
    price_cents: u64,
    name: &'a str,
    id: ItemId,
}

impl<'a> Ord for SortKey<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .price_cents
            .cmp(&self.price_cents)
            .then_with(|| self.name.cmp(&other.name))
            .then_with(|| self.id.cmp(&other.id))
    }
}

impl<'a> PartialOrd for SortKey<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> SortKey<'a> {
    pub(crate) fn from_item(item: &'a Item) -> Self {
        Self {
            price_cents: item.get_price_cents(),
            name: item.get_name(),
            id: item.get_id(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) struct NameKey<'a> {
    name: &'a str,
    id: ItemId,
}

impl<'a> Ord for NameKey<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name
            .cmp(&other.name)
            .then_with(|| self.id.cmp(&other.id))
    }
}

impl<'a> PartialOrd for NameKey<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl<'a> NameKey<'a> {
    pub(crate) fn from_item(item: &'a Item) -> Self {
        Self {
            name: item.get_name(),
            id: item.get_id(),
        }
    }
}

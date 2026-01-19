use crate::domain::types::{OrderId, Qty, Sku};
use std::collections::HashMap;

use crate::domain::error::DomainError;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderStatus {
    Draft,
    Confirmed,
    Cancelled,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Order {
    id: OrderId,
    customer: String,
    status: OrderStatus,
    items: HashMap<Sku, Qty>,
}

impl Order {
    pub fn new(id: OrderId, customer: String, status: OrderStatus) -> Result<Self, DomainError> {
        if customer.trim().is_empty() {
            return Err(DomainError::EmptyCustomer);
        }
        if customer.contains('"') {
            return Err(DomainError::CustomerQuote);
        }
        if customer.contains('\n') || customer.contains('\r') {
            return Err(DomainError::CustomerNewline);
        }
        Ok(Self {
            id,
            customer,
            status,
            items: HashMap::new(),
        })
    }

    pub fn add_item(&mut self, sku: Sku, qty: Qty) -> Result<(), DomainError> {
        if self.status == OrderStatus::Cancelled {
            return Err(DomainError::OrderCancelled);
        }
        if self.items.contains_key(&sku) {
            return Err(DomainError::SkuAlreadyExists);
        }

        self.items.insert(sku, qty);
        Ok(())
    }

    pub fn remove_item(&mut self, sku: &Sku) -> Result<(), DomainError> {
        if self.status == OrderStatus::Cancelled {
            return Err(DomainError::OrderCancelled);
        }

        self.items
            .remove(sku)
            .map(|_| ())
            .ok_or(DomainError::ItemNotFound)
    }

    pub fn total_qty(&self) -> u32 {
        self.items.values().map(|qty| qty.get()).sum()
    }

    pub fn get_id(&self) -> OrderId {
        self.id
    }
    pub fn get_customer(&self) -> &str {
        &self.customer
    }
    pub fn get_status(&self) -> OrderStatus {
        self.status
    }
    pub fn get_items(&self) -> impl Iterator<Item = (&Sku, &Qty)> {
        self.items.iter()
    }
}

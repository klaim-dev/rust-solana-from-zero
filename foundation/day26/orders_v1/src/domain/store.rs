use crate::domain::error::DomainError;
use crate::domain::order::Order;
use crate::domain::types::{OrderId, Qty, Sku};
use std::collections::BTreeSet;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Store {
    orders_by_id: HashMap<OrderId, Order>,
    ids_by_customer: HashMap<CustomerKey, BTreeSet<OrderId>>,
}
impl Store {
    pub fn new() -> Self {
        Self {
            orders_by_id: HashMap::new(),
            ids_by_customer: HashMap::new(),
        }
    }
    pub fn add_order(&mut self, order: Order) -> Result<(), DomainError> {
        if self.orders_by_id.contains_key(&order.get_id()) {
            return Err(DomainError::DuplicateOrderId {
                order_id: order.get_id(),
            });
        }
        let customer_key = CustomerKey::new(order.get_customer());
        let order_id = order.get_id();
        self.orders_by_id.insert(order.get_id(), order);
        self.ids_by_customer
            .entry(customer_key)
            .or_default()
            .insert(order_id);
        Ok(())
    }
    pub fn add_item(&mut self, order_id: OrderId, sku: Sku, qty: Qty) -> Result<(), DomainError> {
        let order = self.get_order_mut(order_id)?;
        order.add_item(sku, qty)
    }
    pub fn remove_item(&mut self, order_id: OrderId, sku: &Sku) -> Result<(), DomainError> {
        let order = self.get_order_mut(order_id)?;
        order.remove_item(sku)
    }

    pub fn total_qty(&self, order_id: OrderId) -> Result<u32, DomainError> {
        let order = self.get_order(order_id)?;
        Ok(order.total_qty())
    }

    pub fn list_all(&self) -> Vec<&Order> {
        self.orders_by_id.values().collect()
    }

    pub fn list_by_customer(&self, customer: &str) -> Vec<&Order> {
        let customer_key = CustomerKey::new(customer);
        let mut vec_orders = Vec::new();
        if let Some(ids) = self.ids_by_customer.get(&customer_key) {
            for id in ids {
                if let Some(order) = self.orders_by_id.get(id) {
                    vec_orders.push(order);
                }
            }
        }
        vec_orders
    }

    pub fn show(&self, order_id: OrderId) -> Result<&Order, DomainError> {
        self.get_order(order_id)
    }

    fn get_order(&self, order_id: OrderId) -> Result<&Order, DomainError> {
        self.orders_by_id
            .get(&order_id)
            .ok_or(DomainError::OrderNotFound { order_id })
    }

    fn get_order_mut(&mut self, order_id: OrderId) -> Result<&mut Order, DomainError> {
        self.orders_by_id
            .get_mut(&order_id)
            .ok_or(DomainError::OrderNotFound { order_id })
    }
}

impl Default for Store {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Eq, Hash, PartialEq)]
pub struct CustomerKey(String);
impl CustomerKey {
    pub fn new(input: &str) -> Self {
        Self(input.trim().to_ascii_lowercase())
    }
}

use crate::domain::error::DomainError;
use crate::domain::order::Order;
use crate::domain::types::{OrderId, Qty, Sku};
use std::collections::BTreeSet;
use std::collections::HashMap;

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
        let order = self.orders_by_id.get_mut(&order_id);
        match order {
            None => Err(DomainError::OrderNotFound { order_id }),
            Some(order) => {
                Order::add_item(order, sku, qty)?;
                Ok(())
            }
        }
    }
    pub fn remove_item(&mut self, order_id: OrderId, sku: &Sku) -> Result<(), DomainError> {
        if let Some(order) = self.orders_by_id.get_mut(&order_id) {
            let _ = order.remove_item(sku)?;
        } else {
            return Err(DomainError::OrderNotFound { order_id });
        }
        Ok(())
    }

    pub fn total_item(&self, order_id: OrderId) -> Result<u32, DomainError> {
        if let Some(order) = self.orders_by_id.get(&order_id) {
            return Ok(order.total_qty());
        } else {
            return Err(DomainError::OrderNotFound { order_id });
        }
    }

    pub fn get(&self, order_id: OrderId) -> Result<&Order, DomainError> {
        if let Some(order) = self.orders_by_id.get(&order_id) {
            return Ok(order);
        } else {
            return Err(DomainError::OrderNotFound { order_id });
        }
    }
    pub fn list_all<'a>(&'a self) -> Vec<&'a Order> {
        let mut orders = self.orders_by_id.values().collect::<Vec<_>>();
        orders.sort_by_key(|order| order.get_id());
        orders
    }

    pub fn list_by_customer<'a>(&'a self, customer: &str) -> Vec<&'a Order> {
        let customer_key = CustomerKey::new(customer);
        let mut vec_orders = Vec::new();
        if let Some(ids) = self.ids_by_customer.get(&customer_key) {
            for id in ids {
                if let Some(order) = self.orders_by_id.get(id) {
                    let _ = vec_orders.push(order);
                }
            }
        }
        vec_orders
    }

    pub fn show<'a>(&'a self, order_id: OrderId) -> Result<&'a Order, DomainError> {
        if let Some(order) = self.orders_by_id.get(&order_id) {
            return Ok(order);
        } else {
            return Err(DomainError::OrderNotFound { order_id });
        }
    }
}
#[derive(Eq, Hash, PartialEq)]
pub struct CustomerKey(String);
impl CustomerKey {
    pub fn new(input: &str) -> Self {
        Self(input.trim().to_ascii_lowercase())
    }

    pub fn get(&self) -> &str {
        &self.0
    }
}

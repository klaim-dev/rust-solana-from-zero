use crate::domain::types::OrderId;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("sku is empty")]
    EmptySku,
    #[error("order id must be non-zero")]
    ZeroOrderId,
    #[error("qty must be non-zero")]
    ZeroQty,
    #[error("money amount must be non-zero")]
    ZeroMoneyCents,
    #[error("customer is empty")]
    EmptyCustomer,
    #[error("customer contains a double quote")]
    CustomerQuote,
    #[error("sku contains whitespace")]
    SkuWhitespace,
    #[error("sku contains a double quote")]
    SkuQuote,
    #[error("order is canceled")]
    OrderCanceled,
    #[error("sku already exists in order")]
    SkuAllreadyExsist,
    #[error("item not found")]
    ItemNotFound,
    #[error("order is already canceled")]
    OrderJustCanceled,
    #[error("order id {order_id} already exists")]
    DuplicateOrderId { order_id: OrderId },
    #[error("order id {order_id} not found")]
    OrderNotFound { order_id: OrderId },
}

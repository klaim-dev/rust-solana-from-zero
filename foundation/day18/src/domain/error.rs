use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum OrderError {
    // =========================
    // FSM / transaction
    // =========================
    #[error("invalid transition: from={from} to={to}")]
    InvalidTransition {
        from: &'static str,
        to: &'static str,
    },

    // =========================
    // Submit rules
    // =========================
    #[error("cannot submit an empty order")]
    EmptyOrder,

    // =========================
    // Editing rules (order locked)
    // =========================
    #[error("order is not editable in state {state}")]
    OrderNotEditable { state: String },

    // =========================
    // Items validation / operations
    // =========================
    #[error("item not found: {sku}")]
    ItemNotFound { sku: String },
       #[error("empty sku")]
    EmptySku,

    #[error("duplicate item: {sku}")]
    DuplicateItem { sku: String },

    #[error("zero quantity")]
    ZeroQuantity,

    #[error("invalid price for item {sku}: {price}")]
    InvalidPrice { sku: String, price: u64 },

    // =========================
    // Payment rules
    // =========================
    #[error("tx_id is empty")]
    EmptyTxId,

    #[error("payment mismatch: expected_total={expected_total} paid_total={paid_total}")]
    PaymentMismatch {
        expected_total: u64,
        paid_total: u64,
    },

    // =========================
    // Cancel rules
    // =========================
    #[error("reason is empty")]
    EmptyCancelReason,

    // =========================
    // Invariant protection (should not happen, but handled)
    // =========================
    #[error("invariant violated: {msg}")]
    InvariantViolation { msg: &'static str },
}

pub fn invalid_transition(from: &'static str, to: &'static str) -> OrderError {
    OrderError::InvalidTransition { from, to }
}
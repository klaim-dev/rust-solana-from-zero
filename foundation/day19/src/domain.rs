use std::fmt;
pub enum AuditEvent {
    OrderCreated {
        id: u64,
    },
    OrderPaid {
        id: u64,
        tx: String,
    },
    OrderCancelled {
        id: u64,
        reason: String,
    },
}

impl fmt::Display for AuditEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AuditEvent::OrderCreated { id } => {
                write!(f, "OrderCreated(id={})", id)
            }
            AuditEvent::OrderPaid { id, tx } => {
                write!(f, "OrderPaid(id={}, tx={})", id, tx)
            }
            AuditEvent::OrderCancelled { id, reason } => {
                write!(f, "OrderCancelled(id={}, reason={})", id, reason)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order_created_display() {
        let event = AuditEvent::OrderCreated { id: 123 };
        assert_eq!(event.to_string(), "OrderCreated(id=123)");
    }

    #[test]
    fn test_order_paid_display() {
        let event = AuditEvent::OrderPaid {
            id: 456,
            tx: "tx_abc123".to_string(),
        };
        assert_eq!(event.to_string(), "OrderPaid(id=456, tx=tx_abc123)");
    }

    #[test]
    fn test_order_cancelled_display() {
        let event = AuditEvent::OrderCancelled {
            id: 789,
            reason: "out_of_stock".to_string(),
        };
        assert_eq!(
            event.to_string(),
            "OrderCancelled(id=789, reason=out_of_stock)"
        );
    }

    #[test]
    fn test_order_created_with_zero_id() {
        let event = AuditEvent::OrderCreated { id: 0 };
        assert_eq!(event.to_string(), "OrderCreated(id=0)");
    }

    #[test]
    fn test_order_paid_with_empty_tx() {
        let event = AuditEvent::OrderPaid {
            id: 1,
            tx: String::new(),
        };
        assert_eq!(event.to_string(), "OrderPaid(id=1, tx=)");
    }

    #[test]
    fn test_order_cancelled_with_long_reason() {
        let event = AuditEvent::OrderCancelled {
            id: 999,
            reason: "Customer requested cancellation due to shipping delay".to_string(),
        };
        assert_eq!(
            event.to_string(),
            "OrderCancelled(id=999, reason=Customer requested cancellation due to shipping delay)"
        );
    }
}
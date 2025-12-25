use crate::domain::error::OrderError;
use crate::domain::error::invalid_transition;
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum OrderState {
    //TAG    | DATA
    Submitted { at: u64 },
    Paid { at: u64, tx_id: String },
    Cancelled { at: u64, reason: String },
    Draft,
}
impl fmt::Display for OrderState{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            OrderState::Draft => "Draft",
            OrderState::Cancelled {..} => "Cancelled",
            OrderState::Paid {..} => "Paid",
            OrderState::Submitted { .. } => "Submitted",
        };
        write!(f, "{s}")
    }
}

impl OrderState {
    pub(crate) fn submit(&mut self, now: u64) -> Result<(), OrderError> {
        match self {
            Self::Draft => {
                *self = OrderState::Submitted { at: now };
                Ok(())
            }
            Self::Cancelled { .. } => Err(invalid_transition("Cancelled", "Submitted")),
            Self::Paid { .. } => Err(invalid_transition("Paid", "Submitted")),
            Self::Submitted { .. } => {
                let to_op = "submit";
                Err(invalid_transition("Submitted", to_op))
            }
        }
    }

    pub(crate) fn pay(&mut self, now: u64, tx_id: String) -> Result<(), OrderError> {
        let normalized = tx_id.trim();
        if normalized.is_empty() {
            return Err(OrderError::EmptyTxId);
        }

        match self {
            Self::Submitted { .. } => {
                *self = OrderState::Paid {
                    at: now,
                    tx_id: normalized.to_string(),
                };
                Ok(())
            }
            Self::Draft => Err(invalid_transition("Draft", "Paid")),
            Self::Cancelled { .. } => Err(invalid_transition("Cancelled", "Paid")),
            Self::Paid { .. } => Err(invalid_transition("Paid", "Paid")),
        }
    }

    pub(crate) fn cancel(&mut self, now: u64, reason: String) -> Result<(), OrderError> {
        let normalized = reason.trim();
        if normalized.is_empty() {
            return Err(OrderError::EmptyCancelReason);
        }

        match self {
            Self::Draft => {
                *self = OrderState::Cancelled {
                    at: now,
                    reason: normalized.to_string(),
                };
                Ok(())
            }
            Self::Submitted { .. } => {
                *self = OrderState::Cancelled {
                    at: now,
                    reason: normalized.to_string(),
                };
                Ok(())
            }
            Self::Paid { .. } => Err(invalid_transition("Paid", "Cancelled")),
            Self::Cancelled { .. } => Err(invalid_transition("Cancelled", "Cancelled")),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn submit_happy_path() {
        let mut order_state = OrderState::Draft;
        let res = order_state.submit(100).unwrap();
        assert_eq!(order_state, OrderState::Submitted { at: 100 });
    }

    #[test]
    fn submit_submitted_is_error() {
        let mut order_state = OrderState::Submitted { at: 1 };
        let err = order_state.submit(100).err().unwrap();
        assert_eq!(
            err,
            OrderError::InvalidTransition {
                from: "Submitted",
                to: "submit"
            }
        );

        assert!(matches!(order_state, OrderState::Submitted { at: 1 }))
    }

    #[test]
    fn submit_paid_is_error() {
        let mut order_state = OrderState::Paid {
            at: 100,
            tx_id: "Test".to_string(),
        };
        let err = order_state.submit(100).err().unwrap();
        assert_eq!(
            err,
            OrderError::InvalidTransition {
                from: "Paid",
                to: "Submitted"
            }
        );

        assert!(matches!(order_state, OrderState::Paid { at: 100, .. }))
    }

    #[test]
    fn submit_cancelled_is_error() {
        let mut order_state = OrderState::Cancelled {
            at: 100,
            reason: "test".to_string(),
        };
        let err = order_state.submit(100).err().unwrap();
        assert_eq!(
            err,
            OrderError::InvalidTransition {
                from: "Cancelled",
                to: "Submitted"
            }
        );
        assert!(matches!(order_state, OrderState::Cancelled { at: 100, .. }))
    }

    #[test]
    fn pay_happy_path() {
        let mut order_state = OrderState::Submitted { at: 100 };
        let res = order_state.pay(100, "test".to_string()).unwrap();
        assert_eq!(
            order_state,
            OrderState::Paid {
                at: 100,
                tx_id: "test".to_string()
            }
        );
    }

    #[test]
    fn pay_paid_is_error() {
        let mut order_state = OrderState::Paid {
            at: 100,
            tx_id: "Test".to_string(),
        };
        let err = order_state.pay(100, "Test".to_string()).err().unwrap();
        assert_eq!(
            err,
            OrderError::InvalidTransition {
                from: "Paid",
                to: "Paid"
            }
        );

        assert!(matches!(order_state, OrderState::Paid { at: 100, .. }))
    }

    #[test]
    fn pay_draft_is_error() {
        let mut order_state = OrderState::Draft;
        let err = order_state.pay(100, "Test".to_string()).err().unwrap();
        assert_eq!(
            err,
            OrderError::InvalidTransition {
                from: "Draft",
                to: "Paid"
            }
        );

        assert!(matches!(order_state, OrderState::Draft))
    }

    #[test]
    fn pay_cancelled_is_error() {
        let mut order_state = OrderState::Cancelled {
            at: 100,
            reason: "Test".to_string(),
        };
        let err = order_state.pay(100, "Test".to_string()).err().unwrap();
        assert_eq!(
            err,
            OrderError::InvalidTransition {
                from: "Cancelled",
                to: "Paid"
            }
        );

        assert!(matches!(order_state, OrderState::Cancelled { at: 100, .. }))
    }

    #[test]
    fn pay__tx_id_is_empty() {
        let mut order_state = OrderState::Paid {
            at: 100,
            tx_id: "Test".to_string(),
        };
        let err = order_state.pay(100, " ".to_string()).err().unwrap();
        assert_eq!(err, OrderError::EmptyTxId);

        assert!(matches!(order_state, OrderState::Paid { at: 100, .. }))
    }

    #[test]
    fn cancel_draft_happy_path() {
        let mut order_state = OrderState::Draft;
        let res = order_state.cancel(100, "Test".to_string()).unwrap();
        assert_eq!(
            order_state,
            OrderState::Cancelled {
                at: 100,
                reason: "Test".to_string()
            }
        );
    }
    #[test]
    fn cancel_submitted_happy_path() {
        let mut order_state = OrderState::Submitted { at: 100 };
        let res = order_state.cancel(100, "Test".to_string()).unwrap();
        assert_eq!(
            order_state,
            OrderState::Cancelled {
                at: 100,
                reason: "Test".to_string()
            }
        );
    }

    #[test]
    fn cancel__reason_is_empty() {
        let mut order_state = OrderState::Submitted { at: 100 };
        let err = order_state.cancel(100, " ".to_string()).err().unwrap();
        assert_eq!(err, OrderError::EmptyCancelReason);

        assert!(matches!(order_state, OrderState::Submitted { at: 100 }))
    }

    #[test]
    fn cancel_pay_is_error() {
        let mut order_state = OrderState::Paid {
            at: 100,
            tx_id: "Test".to_string(),
        };
        let err = order_state.cancel(100, "Test".to_string()).err().unwrap();
        assert_eq!(
            err,
            OrderError::InvalidTransition {
                from: "Paid",
                to: "Cancelled"
            }
        );

        assert!(matches!(order_state, OrderState::Paid { at: 100, .. }))
    }

    #[test]
    fn cancel_cancelled_is_error() {
        let mut order_state = OrderState::Cancelled {
            at: 100,
            reason: "Test".to_string(),
        };
        let err = order_state.cancel(100, "Test".to_string()).err().unwrap();
        assert_eq!(
            err,
            OrderError::InvalidTransition {
                from: "Cancelled",
                to: "Cancelled"
            }
        );

        assert!(matches!(order_state, OrderState::Cancelled { at: 100, .. }))
    }
}

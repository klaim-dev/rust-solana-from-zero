use crate::domain::AuditEvent;

pub trait Formatter {
    fn format(&self, e: &AuditEvent) -> String;
}
pub struct PlainFormatter;
impl Formatter for PlainFormatter {
    fn format(&self, e: &AuditEvent) -> String {
        e.to_string()
    }

}
pub struct CompactFormatter;
impl Formatter for CompactFormatter {
    fn format(&self, e: &AuditEvent) -> String {
        match e {
            AuditEvent::OrderCreated { id } => {
                format!("CREATED#{}", id)
            }
            AuditEvent::OrderPaid { id, .. } => {
                format!("PAID#{}", id)
            }
            AuditEvent::OrderCancelled { id, .. } => {
                format!("CANCELLED#{}", id)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::AuditEvent;

    #[test]
    fn test_plain_formatter_order_created() {
        let formatter = PlainFormatter;
        let event = AuditEvent::OrderCreated { id: 123 };
        assert_eq!(formatter.format(&event), "OrderCreated(id=123)");
    }

    #[test]
    fn test_plain_formatter_order_paid() {
        let formatter = PlainFormatter;
        let event = AuditEvent::OrderPaid {
            id: 456,
            tx: "tx_xyz789".to_string(),
        };
        assert_eq!(formatter.format(&event), "OrderPaid(id=456, tx=tx_xyz789)");
    }

    #[test]
    fn test_plain_formatter_order_cancelled() {
        let formatter = PlainFormatter;
        let event = AuditEvent::OrderCancelled {
            id: 789,
            reason: "customer_request".to_string(),
        };
        assert_eq!(
            formatter.format(&event),
            "OrderCancelled(id=789, reason=customer_request)"
        );
    }

    #[test]
    fn test_compact_formatter_order_created() {
        let formatter = CompactFormatter;
        let event = AuditEvent::OrderCreated { id: 123 };
        assert_eq!(formatter.format(&event), "CREATED#123");
    }

    #[test]
    fn test_compact_formatter_order_paid() {
        let formatter = CompactFormatter;
        let event = AuditEvent::OrderPaid {
            id: 456,
            tx: "tx_abc123".to_string(),
        };
        assert_eq!(formatter.format(&event), "PAID#456");
    }

    #[test]
    fn test_compact_formatter_order_cancelled() {
        let formatter = CompactFormatter;
        let event = AuditEvent::OrderCancelled {
            id: 789,
            reason: "out_of_stock".to_string(),
        };
        assert_eq!(formatter.format(&event), "CANCELLED#789");
    }

    #[test]
    fn test_compact_formatter_ignores_additional_fields() {
        let formatter = CompactFormatter;
        let paid_event = AuditEvent::OrderPaid {
            id: 999,
            tx: "very_long_transaction_id_12345".to_string(),
        };
        // Компактный формат должен игнорировать tx
        assert_eq!(formatter.format(&paid_event), "PAID#999");

        let cancelled_event = AuditEvent::OrderCancelled {
            id: 888,
            reason: "very long reason that should be ignored".to_string(),
        };

        assert_eq!(formatter.format(&cancelled_event), "CANCELLED#888");
    }

    #[test]
    fn test_formatters_with_zero_id() {
        let plain = PlainFormatter;
        let compact = CompactFormatter;
        let event = AuditEvent::OrderCreated { id: 0 };

        assert_eq!(plain.format(&event), "OrderCreated(id=0)");
        assert_eq!(compact.format(&event), "CREATED#0");
    }
}
use crate::domain::AuditEvent;

use crate::format::Formatter;

use crate::sink::Sink;

pub fn render(
    events: &[AuditEvent],
    formatter: &impl Formatter,
    filter: impl Fn(&AuditEvent) -> bool,
) -> Vec<String> {
    let mut out = Vec::new();

    for e in events {
        if filter(e) {
            let s = formatter.format(e);
            out.push(s);
        }
    }

    out
}

pub fn emit_events<F, S, P>(
    events: &[AuditEvent],
    formatter: &F,
    sink: &mut S,
    predicate: P,
)
where
    F: Formatter,
    S: Sink,
    P: Fn(&AuditEvent) -> bool,
{
    for e in events{
        if predicate(e) {
            let s = formatter.format(e);
            sink.write(s);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::AuditEvent;
    use crate::format::{PlainFormatter, CompactFormatter};
    use crate::sink::VecSink;

    #[test]
    fn test_render_empty_events() {
        let events = vec![];
        let formatter = PlainFormatter;
        let result = render(&events, &formatter, |_| true);
        assert!(result.is_empty());
    }

    #[test]
    fn test_render_all_events() {
        let events = vec![
            AuditEvent::OrderCreated { id: 1 },
            AuditEvent::OrderPaid {
                id: 2,
                tx: "tx1".to_string(),
            },
            AuditEvent::OrderCancelled {
                id: 3,
                reason: "test".to_string(),
            },
        ];
        let formatter = PlainFormatter;
        let result = render(&events, &formatter, |_| true);

        assert_eq!(result.len(), 3);
        assert_eq!(result[0], "OrderCreated(id=1)");
        assert_eq!(result[1], "OrderPaid(id=2, tx=tx1)");
        assert_eq!(result[2], "OrderCancelled(id=3, reason=test)");
    }

    #[test]
    fn test_render_with_filter() {
        let events = vec![
            AuditEvent::OrderCreated { id: 1 },
            AuditEvent::OrderCreated { id: 2 },
            AuditEvent::OrderPaid {
                id: 3,
                tx: "tx1".to_string(),
            },
        ];
        let formatter = PlainFormatter;
        let result = render(&events, &formatter, |e| matches!(e, AuditEvent::OrderCreated { .. }));

        assert_eq!(result.len(), 2);
        assert_eq!(result[0], "OrderCreated(id=1)");
        assert_eq!(result[1], "OrderCreated(id=2)");
    }

    #[test]
    fn test_render_filters_all_events() {
        let events = vec![
            AuditEvent::OrderCreated { id: 1 },
            AuditEvent::OrderPaid {
                id: 2,
                tx: "tx1".to_string(),
            },
        ];
        let formatter = PlainFormatter;
        let result = render(&events, &formatter, |_| false);
        assert!(result.is_empty());
    }

    #[test]
    fn test_render_with_compact_formatter() {
        let events = vec![
            AuditEvent::OrderCreated { id: 1 },
            AuditEvent::OrderPaid {
                id: 2,
                tx: "tx1".to_string(),
            },
        ];
        let formatter = CompactFormatter;
        let result = render(&events, &formatter, |_| true);

        assert_eq!(result.len(), 2);
        assert_eq!(result[0], "CREATED#1");
        assert_eq!(result[1], "PAID#2");
    }

    #[test]
    fn test_emit_events_empty() {
        let events = vec![];
        let formatter = PlainFormatter;
        let mut sink = VecSink::new();

        emit_events(&events, &formatter, &mut sink, |_| true);

        assert_eq!(sink.lines().len(), 0);
    }

    #[test]
    fn test_emit_events_all_events() {
        let events = vec![
            AuditEvent::OrderCreated { id: 1 },
            AuditEvent::OrderPaid {
                id: 2,
                tx: "tx1".to_string(),
            },
            AuditEvent::OrderCancelled {
                id: 3,
                reason: "test".to_string(),
            },
        ];
        let formatter = PlainFormatter;
        let mut sink = VecSink::new();

        emit_events(&events, &formatter, &mut sink, |_| true);

        assert_eq!(sink.lines().len(), 3);
        assert_eq!(sink.lines()[0], "OrderCreated(id=1)");
        assert_eq!(sink.lines()[1], "OrderPaid(id=2, tx=tx1)");
        assert_eq!(sink.lines()[2], "OrderCancelled(id=3, reason=test)");
    }

    #[test]
    fn test_emit_events_with_predicate() {
        let events = vec![
            AuditEvent::OrderCreated { id: 1 },
            AuditEvent::OrderCreated { id: 2 },
            AuditEvent::OrderPaid {
                id: 3,
                tx: "tx1".to_string(),
            },
        ];
        let formatter = PlainFormatter;
        let mut sink = VecSink::new();

        emit_events(&events, &formatter, &mut sink, |e| matches!(e, AuditEvent::OrderCreated { .. }));

        assert_eq!(sink.lines().len(), 2);
        assert_eq!(sink.lines()[0], "OrderCreated(id=1)");
        assert_eq!(sink.lines()[1], "OrderCreated(id=2)");
    }

    #[test]
    fn test_emit_events_with_compact_formatter() {
        let events = vec![
            AuditEvent::OrderCreated { id: 1 },
            AuditEvent::OrderPaid {
                id: 2,
                tx: "tx1".to_string(),
            },
        ];
        let formatter = CompactFormatter;
        let mut sink = VecSink::new();

        emit_events(&events, &formatter, &mut sink, |_| true);

        assert_eq!(sink.lines().len(), 2);
        assert_eq!(sink.lines()[0], "CREATED#1");
        assert_eq!(sink.lines()[1], "PAID#2");
    }

    #[test]
    fn test_emit_events_filters_by_id() {
        let events = vec![
            AuditEvent::OrderCreated { id: 1 },
            AuditEvent::OrderCreated { id: 2 },
            AuditEvent::OrderCreated { id: 3 },
        ];
        let formatter = PlainFormatter;
        let mut sink = VecSink::new();

        emit_events(&events, &formatter, &mut sink, |e| {
            match e {
                AuditEvent::OrderCreated { id } => *id > 1,
                _ => false,
            }
        });

        assert_eq!(sink.lines().len(), 2);
        assert_eq!(sink.lines()[0], "OrderCreated(id=2)");
        assert_eq!(sink.lines()[1], "OrderCreated(id=3)");
    }

    #[test]
    fn test_emit_events_filters_all() {
        let events = vec![
            AuditEvent::OrderCreated { id: 1 },
            AuditEvent::OrderPaid {
                id: 2,
                tx: "tx1".to_string(),
            },
        ];
        let formatter = PlainFormatter;
        let mut sink = VecSink::new();

        emit_events(&events, &formatter, &mut sink, |_| false);

        assert_eq!(sink.lines().len(), 0);
    }
}


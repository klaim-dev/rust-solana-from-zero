use day19::{
    emit_events, render,
    AuditEvent,
    CompactFormatter, PlainFormatter,
    VecSink,
};

#[test]
fn test_integration_plain_formatting_pipeline() {
    let events = vec![
        AuditEvent::OrderCreated { id: 100 },
        AuditEvent::OrderPaid {
            id: 100,
            tx: "0xabc123".to_string(),
        },
        AuditEvent::OrderCancelled {
            id: 101,
            reason: "customer_request".to_string(),
        },
    ];

    let formatter = PlainFormatter;
    let result = render(&events, &formatter, |_| true);

    assert_eq!(result.len(), 3);
    assert_eq!(result[0], "OrderCreated(id=100)");
    assert_eq!(result[1], "OrderPaid(id=100, tx=0xabc123)");
    assert_eq!(result[2], "OrderCancelled(id=101, reason=customer_request)");
}

#[test]
fn test_integration_compact_formatting_pipeline() {
    let events = vec![
        AuditEvent::OrderCreated { id: 100 },
        AuditEvent::OrderPaid {
            id: 100,
            tx: "0xabc123".to_string(),
        },
        AuditEvent::OrderCancelled {
            id: 101,
            reason: "customer_request".to_string(),
        },
    ];

    let formatter = CompactFormatter;
    let result = render(&events, &formatter, |_| true);

    assert_eq!(result.len(), 3);
    assert_eq!(result[0], "CREATED#100");
    assert_eq!(result[1], "PAID#100");
    assert_eq!(result[2], "CANCELLED#101");
}

#[test]
fn test_integration_emit_with_sink() {
    let events = vec![
        AuditEvent::OrderCreated { id: 1 },
        AuditEvent::OrderPaid {
            id: 1,
            tx: "tx1".to_string(),
        },
        AuditEvent::OrderCancelled {
            id: 2,
            reason: "test".to_string(),
        },
    ];

    let formatter = PlainFormatter;
    let mut sink = VecSink::new();

    emit_events(&events, &formatter, &mut sink, |_| true);

    let lines = sink.lines();
    assert_eq!(lines.len(), 3);
    assert_eq!(lines[0], "OrderCreated(id=1)");
    assert_eq!(lines[1], "OrderPaid(id=1, tx=tx1)");
    assert_eq!(lines[2], "OrderCancelled(id=2, reason=test)");
}

#[test]
fn test_integration_filter_paid_events_only() {
    let events = vec![
        AuditEvent::OrderCreated { id: 1 },
        AuditEvent::OrderPaid {
            id: 1,
            tx: "tx1".to_string(),
        },
        AuditEvent::OrderCreated { id: 2 },
        AuditEvent::OrderPaid {
            id: 2,
            tx: "tx2".to_string(),
        },
        AuditEvent::OrderCancelled {
            id: 3,
            reason: "test".to_string(),
        },
    ];

    let formatter = CompactFormatter;
    let result = render(&events, &formatter, |e| matches!(e, AuditEvent::OrderPaid { .. }));

    assert_eq!(result.len(), 2);
    assert_eq!(result[0], "PAID#1");
    assert_eq!(result[1], "PAID#2");
}

#[test]
fn test_integration_filter_by_id_range() {
    let events = vec![
        AuditEvent::OrderCreated { id: 1 },
        AuditEvent::OrderCreated { id: 5 },
        AuditEvent::OrderCreated { id: 10 },
        AuditEvent::OrderCreated { id: 15 },
    ];

    let formatter = PlainFormatter;
    let mut sink = VecSink::new();

    emit_events(&events, &formatter, &mut sink, |e| {
        match e {
            AuditEvent::OrderCreated { id } => *id >= 5 && *id <= 10,
            _ => false,
        }
    });

    let lines = sink.lines();
    assert_eq!(lines.len(), 2);
    assert_eq!(lines[0], "OrderCreated(id=5)");
    assert_eq!(lines[1], "OrderCreated(id=10)");
}

#[test]
fn test_integration_full_workflow_with_multiple_formats() {
    let events = vec![
        AuditEvent::OrderCreated { id: 1 },
        AuditEvent::OrderPaid {
            id: 1,
            tx: "0x123".to_string(),
        },
        AuditEvent::OrderCreated { id: 2 },
        AuditEvent::OrderCancelled {
            id: 2,
            reason: "out_of_stock".to_string(),
        },
    ];


    let plain_formatter = PlainFormatter;
    let plain_result = render(&events, &plain_formatter, |_| true);
    assert_eq!(plain_result.len(), 4);

    let compact_formatter = CompactFormatter;
    let mut sink = VecSink::new();
    emit_events(&events, &compact_formatter, &mut sink, |_| true);

    let compact_lines = sink.lines();
    assert_eq!(compact_lines.len(), 4);
    assert_eq!(compact_lines[0], "CREATED#1");
    assert_eq!(compact_lines[1], "PAID#1");
    assert_eq!(compact_lines[2], "CREATED#2");
    assert_eq!(compact_lines[3], "CANCELLED#2");
}

#[test]
fn test_integration_empty_events_workflow() {
    let events = vec![];

    let formatter = PlainFormatter;
    let result = render(&events, &formatter, |_| true);
    assert!(result.is_empty());

    let mut sink = VecSink::new();
    emit_events(&events, &formatter, &mut sink, |_| true);
    assert!(sink.lines().is_empty());
}

#[test]
fn test_integration_complex_filtering_scenario() {
    let events = vec![
        AuditEvent::OrderCreated { id: 1 },
        AuditEvent::OrderPaid {
            id: 1,
            tx: "tx1".to_string(),
        },
        AuditEvent::OrderCreated { id: 2 },
        AuditEvent::OrderPaid {
            id: 2,
            tx: "tx2".to_string(),
        },
        AuditEvent::OrderCancelled {
            id: 3,
            reason: "test".to_string(),
        },
        AuditEvent::OrderCreated { id: 4 },
    ];

    let formatter = CompactFormatter;
    let result = render(&events, &formatter, |e| {
        match e {
            AuditEvent::OrderCreated { id } => *id % 2 == 0,
            _ => false,
        }
    });

    assert_eq!(result.len(), 2);
    assert_eq!(result[0], "CREATED#2");
    assert_eq!(result[1], "CREATED#4");
}

#[test]
fn test_integration_sink_persistence() {
    let events1 = vec![
        AuditEvent::OrderCreated { id: 1 },
        AuditEvent::OrderCreated { id: 2 },
    ];

    let events2 = vec![
        AuditEvent::OrderPaid {
            id: 1,
            tx: "tx1".to_string(),
        },
        AuditEvent::OrderPaid {
            id: 2,
            tx: "tx2".to_string(),
        },
    ];

    let formatter = CompactFormatter;
    let mut sink = VecSink::new();

    emit_events(&events1, &formatter, &mut sink, |_| true);
    assert_eq!(sink.lines().len(), 2);

    emit_events(&events2, &formatter, &mut sink, |_| true);
    assert_eq!(sink.lines().len(), 4);

    assert_eq!(sink.lines()[0], "CREATED#1");
    assert_eq!(sink.lines()[1], "CREATED#2");
    assert_eq!(sink.lines()[2], "PAID#1");
    assert_eq!(sink.lines()[3], "PAID#2");
}

#[test]
fn test_integration_multiple_filters() {
    let events = vec![
        AuditEvent::OrderCreated { id: 1 },
        AuditEvent::OrderPaid {
            id: 1,
            tx: "tx1".to_string(),
        },
        AuditEvent::OrderCancelled {
            id: 2,
            reason: "test1".to_string(),
        },
        AuditEvent::OrderCreated { id: 3 },
        AuditEvent::OrderCancelled {
            id: 4,
            reason: "test2".to_string(),
        },
    ];

    let formatter = PlainFormatter;
    let cancelled = render(&events, &formatter, |e| matches!(e, AuditEvent::OrderCancelled { .. }));
    assert_eq!(cancelled.len(), 2);

    let mut sink = VecSink::new();
    emit_events(&events, &formatter, &mut sink, |e| {
        matches!(e, AuditEvent::OrderCreated { .. } | AuditEvent::OrderPaid { .. })
    });
    assert_eq!(sink.lines().len(), 3);
}


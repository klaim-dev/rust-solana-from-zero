use day18::domain::{Order, OrderId, LineItem, OrderError};

#[test]
fn happy_path_add_submit() {
    let id = OrderId::new(1).unwrap();
    let mut order = Order::new(id);
    
    let item = LineItem::new("SKU1".to_string(), 2, 1000).unwrap();
    order.add_item(item).unwrap();
    
    order.submit(100).unwrap();
}

#[test]
fn happy_path_submit_pay() {
    let id = OrderId::new(1).unwrap();
    let mut order = Order::new(id);
    
    let item = LineItem::new("SKU1".to_string(), 2, 1000).unwrap();
    order.add_item(item).unwrap();
    
    order.submit(100).unwrap();
    order.pay(200, "tx123".to_string()).unwrap();
}

#[test]
fn happy_path_draft_cancel() {
    let id = OrderId::new(1).unwrap();
    let mut order = Order::new(id);
    
    let item = LineItem::new("SKU1".to_string(), 2, 1000).unwrap();
    order.add_item(item).unwrap();
    
    order.cancel(100, "Customer request".to_string()).unwrap();
}

#[test]
fn submit_empty_order() {
    let id = OrderId::new(1).unwrap();
    let mut order = Order::new(id);
    
    let err = order.submit(100).err().unwrap();
    assert_eq!(err, OrderError::EmptyOrder);
}

#[test]
fn mutate_after_submit() {
    let id = OrderId::new(1).unwrap();
    let mut order = Order::new(id);
    
    let item = LineItem::new("SKU1".to_string(), 2, 1000).unwrap();
    order.add_item(item).unwrap();
    
    order.submit(100).unwrap();
    

    let item2 = LineItem::new("SKU2".to_string(), 1, 500).unwrap();
    let err = order.add_item(item2).err().unwrap();
    assert!(matches!(err, OrderError::OrderNotEditable { .. }));
    

    let err = order.set_qty("SKU1", 3).err().unwrap();
    assert!(matches!(err, OrderError::OrderNotEditable { .. }));
    

    let err = order.remove_item_by_sku("SKU1").err().unwrap();
    assert!(matches!(err, OrderError::OrderNotEditable { .. }));
}

#[test]
fn invalid_transition_draft_to_pay() {
    let id = OrderId::new(1).unwrap();
    let mut order = Order::new(id);
    
    let item = LineItem::new("SKU1".to_string(), 2, 1000).unwrap();
    order.add_item(item).unwrap();
    
    let err = order.pay(100, "tx123".to_string()).err().unwrap();
    assert!(matches!(err, OrderError::InvalidTransition { from: "Draft", to: "Paid" }));
}

#[test]
fn invalid_transition_paid_to_submit() {
    let id = OrderId::new(1).unwrap();
    let mut order = Order::new(id);
    
    let item = LineItem::new("SKU1".to_string(), 2, 1000).unwrap();
    order.add_item(item).unwrap();
    
    order.submit(100).unwrap();
    order.pay(200, "tx123".to_string()).unwrap();
    
    let err = order.submit(300).err().unwrap();
    assert!(matches!(err, OrderError::InvalidTransition { from: "Paid", to: "Submitted" }));
}

#[test]
fn invalid_transition_paid_to_cancel() {
    let id = OrderId::new(1).unwrap();
    let mut order = Order::new(id);
    
    let item = LineItem::new("SKU1".to_string(), 2, 1000).unwrap();
    order.add_item(item).unwrap();
    
    order.submit(100).unwrap();
    order.pay(200, "tx123".to_string()).unwrap();
    
    let err = order.cancel(300, "Reason".to_string()).err().unwrap();
    assert!(matches!(err, OrderError::InvalidTransition { from: "Paid", to: "Cancelled" }));
}

#[test]
fn set_qty_zero() {
    let id = OrderId::new(1).unwrap();
    let mut order = Order::new(id);
    
    let item = LineItem::new("SKU1".to_string(), 2, 1000).unwrap();
    order.add_item(item).unwrap();
    
    let err = order.set_qty("SKU1", 0).err().unwrap();
    assert_eq!(err, OrderError::ZeroQuantity);
}

#[test]
fn set_qty_happy_path() {
    let id = OrderId::new(1).unwrap();
    let mut order = Order::new(id);
    
    let item = LineItem::new("SKU1".to_string(), 2, 1000).unwrap();
    order.add_item(item).unwrap();
    
    order.set_qty("SKU1", 5).unwrap();
    
    let total = order.total_cents().unwrap();
    assert_eq!(total, 5000); // 5 * 1000
}

#[test]
fn total_cents_calculation() {
    let id = OrderId::new(1).unwrap();
    let mut order = Order::new(id);
    
    let item1 = LineItem::new("SKU1".to_string(), 2, 1000).unwrap();
    let item2 = LineItem::new("SKU2".to_string(), 3, 500).unwrap();
    
    order.add_item(item1).unwrap();
    order.add_item(item2).unwrap();
    
    let total = order.total_cents().unwrap();
    assert_eq!(total, 3500); // 2*1000 + 3*500 = 2000 + 1500
}

#[test]
fn submit_submitted_is_error() {
    let id = OrderId::new(1).unwrap();
    let mut order = Order::new(id);
    
    let item = LineItem::new("SKU1".to_string(), 2, 1000).unwrap();
    order.add_item(item).unwrap();
    
    order.submit(100).unwrap();
    
    let err = order.submit(200).err().unwrap();
    assert!(matches!(err, OrderError::InvalidTransition { from: "Submitted", to: "submit" }));
}

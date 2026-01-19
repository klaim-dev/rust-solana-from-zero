use day25::domain::order::{Order, OrderStatus};
use day25::domain::store::Store;
use day25::domain::types::{OrderId, Qty, Sku};
use day25::persist::format::serialize;

fn sample_store() -> Store {
    let order1_id = OrderId::new(1).expect("order id");
    let order2_id = OrderId::new(2).expect("order id");

    let order1 = Order::new(order1_id, "alice".to_string(), OrderStatus::Draft).expect("order1");
    let order2 = Order::new(order2_id, "bob".to_string(), OrderStatus::Confirmed).expect("order2");

    let mut store = Store::new();
    store.add_order(order2).expect("add order2");
    store.add_order(order1).expect("add order1");

    store
        .add_item(
            order2_id,
            Sku::new("sku2".to_string()).expect("sku2"),
            Qty::new(5).expect("qty"),
        )
        .expect("add item");
    store
        .add_item(
            order2_id,
            Sku::new("sku1".to_string()).expect("sku1"),
            Qty::new(2).expect("qty"),
        )
        .expect("add item");
    store
        .add_item(
            order1_id,
            Sku::new("sku1".to_string()).expect("sku1"),
            Qty::new(1).expect("qty"),
        )
        .expect("add item");

    store
}

#[test]
fn serialize_matches_golden_file() {
    let store = sample_store();
    let output = serialize(&store);
    let expected = include_str!("fixtures/orders_golden.txt");
    assert_eq!(output, expected);
}

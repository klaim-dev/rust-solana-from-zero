use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use day25::domain::order::{Order, OrderStatus};
use day25::domain::store::Store;
use day25::domain::types::{OrderId, Qty, Sku};
use day25::persist::format::serialize;
use day25::persist::fs::{load, save_atomic};

fn unique_temp_dir() -> PathBuf {
    let mut dir = std::env::temp_dir();
    let pid = std::process::id();
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time went backwards")
        .as_nanos();
    dir.push(format!("day25-test-{}-{}", pid, nanos));
    fs::create_dir_all(&dir).expect("create temp dir");
    dir
}

fn sample_store() -> Store {
    let order_id = OrderId::new(1).expect("order id");
    let order = Order::new(order_id, "alice".to_string(), OrderStatus::Confirmed).expect("order");
    let mut store = Store::new();
    store.add_order(order).expect("add order");
    store
        .add_item(
            order_id,
            Sku::new("sku1".to_string()).expect("sku"),
            Qty::new(2).expect("qty"),
        )
        .expect("add item");
    store
}

#[test]
fn load_not_found_returns_empty() {
    let dir = unique_temp_dir();
    let path = dir.join("orders.txt");

    let store = load(&path).expect("load missing file");
    assert!(store.list_all().is_empty());

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn save_then_load_roundtrip() {
    let dir = unique_temp_dir();
    let path = dir.join("orders.txt");
    let store = sample_store();

    save_atomic(&path, &store).expect("save");
    let store2 = load(&path).expect("load");

    assert_eq!(serialize(&store), serialize(&store2));

    let _ = fs::remove_dir_all(&dir);
}

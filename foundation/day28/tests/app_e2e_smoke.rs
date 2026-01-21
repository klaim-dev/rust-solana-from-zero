use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use day28::app::output::AppOutput;
use day28::app::service::{AppCommand, OrderService};
use day28::domain::types::{OrderId, Qty, Sku};
use day28::persist::repo::FileRepo;

fn unique_temp_dir() -> PathBuf {
    let mut dir = std::env::temp_dir();
    let pid = std::process::id();
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time went backwards")
        .as_nanos();
    dir.push(format!("day28-e2e-{}-{}", pid, nanos));
    fs::create_dir_all(&dir).expect("create temp dir");
    dir
}

#[test]
fn file_repo_roundtrip_smoke() {
    let dir = unique_temp_dir();
    let path = dir.join("orders.txt");
    let repo = FileRepo::new(path.clone());
    let service = OrderService::new(repo);

    service
        .run(AppCommand::AddOrder {
            id: OrderId::new(1).expect("id"),
            customer: "alice".to_string(),
        })
        .expect("add order");
    service
        .run(AppCommand::AddItem {
            id: OrderId::new(1).expect("id"),
            sku: Sku::new("sku1".to_string()).expect("sku"),
            qty: Qty::new(2).expect("qty"),
        })
        .expect("add item");

    let out = service
        .run(AppCommand::Show {
            id: OrderId::new(1).expect("id"),
        })
        .expect("show");

    match out {
        AppOutput::Text(text) => assert!(text.contains("ORDER id=1")),
    }

    let content = fs::read_to_string(&path).expect("read file");
    assert!(content.contains("ORDER id=1"));
    assert!(content.contains("ITEM order_id=1"));

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn file_repo_list_and_total_from_existing_file() {
    let dir = unique_temp_dir();
    let path = dir.join("orders.txt");
    let content = concat!(
        "ORDER id=1 customer=\"alice\" status=draft\n",
        "ITEM order_id=1 sku=sku1 qty=2\n",
        "ORDER id=2 customer=\"bob\" status=confirmed\n",
        "ITEM order_id=2 sku=sku2 qty=5\n"
    );
    fs::write(&path, content).expect("write file");

    let repo = FileRepo::new(path);
    let service = OrderService::new(repo);

    let out = service
        .run(AppCommand::List { customer: None })
        .expect("list");
    match out {
        AppOutput::Text(text) => {
            assert!(text.contains("ORDER id=1"));
            assert!(text.contains("ORDER id=2"));
        }
    }

    let total = service
        .run(AppCommand::Total {
            id: OrderId::new(2).expect("id"),
        })
        .expect("total");
    match total {
        AppOutput::Text(text) => assert!(text.contains("total_qty=5")),
    }

    let _ = fs::remove_dir_all(&dir);
}

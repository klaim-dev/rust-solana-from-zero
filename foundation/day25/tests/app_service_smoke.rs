use std::mem::replace;
use std::sync::Mutex;

use day25::app::error::AppError;
use day25::app::output::AppOutput;
use day25::app::repo::StoreRepo;
use day25::app::service::{AppCommand, OrderService};
use day25::domain::error::DomainError;
use day25::domain::store::Store;
use day25::domain::types::{OrderId, Qty, Sku};

struct InMemoryRepo {
    store: Mutex<Store>,
}

impl InMemoryRepo {
    fn new() -> Self {
        Self {
            store: Mutex::new(Store::new()),
        }
    }
}

impl StoreRepo for InMemoryRepo {
    fn load(&self) -> Result<Store, AppError> {
        let mut guard = self.store.lock().expect("store lock");
        Ok(replace(&mut *guard, Store::new()))
    }

    fn save(&self, store: Store) -> Result<(), AppError> {
        let mut guard = self.store.lock().expect("store lock");
        *guard = store;
        Ok(())
    }
}

fn service() -> OrderService<InMemoryRepo> {
    OrderService::new(InMemoryRepo::new())
}

#[test]
fn add_order_ok() {
    let service = service();
    let out = service
        .run(AppCommand::AddOrder {
            id: OrderId::new(1).expect("id"),
            customer: "alice".to_string(),
        })
        .expect("add order");
    assert_eq!(out, AppOutput::Text("OK".into()));
}

#[test]
fn add_item_ok() {
    let service = service();
    service
        .run(AppCommand::AddOrder {
            id: OrderId::new(1).expect("id"),
            customer: "alice".to_string(),
        })
        .expect("add order");

    let out = service
        .run(AppCommand::AddItem {
            id: OrderId::new(1).expect("id"),
            sku: Sku::new("sku1".to_string()).expect("sku"),
            qty: Qty::new(2).expect("qty"),
        })
        .expect("add item");

    assert_eq!(out, AppOutput::Text("OK".into()));
}

#[test]
fn show_contains_order_header() {
    let service = service();
    service
        .run(AppCommand::AddOrder {
            id: OrderId::new(1).expect("id"),
            customer: "alice".to_string(),
        })
        .expect("add order");

    let out = service
        .run(AppCommand::Show {
            id: OrderId::new(1).expect("id"),
        })
        .expect("show");

    match out {
        AppOutput::Text(text) => assert!(text.contains("ORDER id=1")),
    }
}

#[test]
fn list_all_two_orders_has_two_order_lines() {
    let service = service();
    service
        .run(AppCommand::AddOrder {
            id: OrderId::new(1).expect("id"),
            customer: "alice".to_string(),
        })
        .expect("add order");
    service
        .run(AppCommand::AddOrder {
            id: OrderId::new(2).expect("id"),
            customer: "bob".to_string(),
        })
        .expect("add order");

    let out = service
        .run(AppCommand::List { customer: None })
        .expect("list");

    match out {
        AppOutput::Text(text) => {
            let count = text
                .lines()
                .filter(|line| line.starts_with("ORDER "))
                .count();
            assert_eq!(count, 2);
        }
    }
}

#[test]
fn list_customer_filters() {
    let service = service();
    service
        .run(AppCommand::AddOrder {
            id: OrderId::new(1).expect("id"),
            customer: "alice".to_string(),
        })
        .expect("add order");
    service
        .run(AppCommand::AddOrder {
            id: OrderId::new(2).expect("id"),
            customer: "bob".to_string(),
        })
        .expect("add order");

    let out = service
        .run(AppCommand::List {
            customer: Some("alice".to_string()),
        })
        .expect("list");

    match out {
        AppOutput::Text(text) => {
            assert!(text.contains("ORDER id=1"));
            assert!(!text.contains("ORDER id=2"));
        }
    }
}

#[test]
fn negative_add_item_missing_order() {
    let service = service();
    let err = service
        .run(AppCommand::AddItem {
            id: OrderId::new(1).expect("id"),
            sku: Sku::new("sku1".to_string()).expect("sku"),
            qty: Qty::new(1).expect("qty"),
        })
        .expect_err("missing order");

    match err {
        AppError::Domain(DomainError::OrderNotFound { .. }) => {}
        _ => panic!("unexpected error: {err:?}"),
    }
}

#[test]
fn negative_duplicate_order_id() {
    let service = service();
    service
        .run(AppCommand::AddOrder {
            id: OrderId::new(1).expect("id"),
            customer: "alice".to_string(),
        })
        .expect("add order");

    let err = service
        .run(AppCommand::AddOrder {
            id: OrderId::new(1).expect("id"),
            customer: "bob".to_string(),
        })
        .expect_err("duplicate order");

    match err {
        AppError::Domain(DomainError::DuplicateOrderId { .. }) => {}
        _ => panic!("unexpected error: {err:?}"),
    }
}

use std::path::PathBuf;
use std::sync::Mutex;

use day28::config::Config;

static ENV_LOCK: Mutex<()> = Mutex::new(());

fn with_orders_env(value: Option<&str>, f: impl FnOnce()) {
    let _guard = ENV_LOCK.lock().expect("env lock");
    let original = std::env::var("ORDERS_FILE").ok();

    match value {
        Some(value) => std::env::set_var("ORDERS_FILE", value),
        None => std::env::remove_var("ORDERS_FILE"),
    }

    f();

    match original {
        Some(value) => std::env::set_var("ORDERS_FILE", value),
        None => std::env::remove_var("ORDERS_FILE"),
    }
}

#[test]
fn config_defaults_to_orders_txt() {
    with_orders_env(None, || {
        let config = Config::load(None);
        assert_eq!(config.storage_path, PathBuf::from("orders.txt"));
    });
}

#[test]
fn config_uses_env_when_set() {
    with_orders_env(Some("/tmp/env-orders.txt"), || {
        let config = Config::load(None);
        assert_eq!(config.storage_path, PathBuf::from("/tmp/env-orders.txt"));
    });
}

#[test]
fn config_override_beats_env() {
    with_orders_env(Some("/tmp/env-orders.txt"), || {
        let config = Config::load(Some(PathBuf::from("/tmp/cli-orders.txt")));
        assert_eq!(config.storage_path, PathBuf::from("/tmp/cli-orders.txt"));
    });
}

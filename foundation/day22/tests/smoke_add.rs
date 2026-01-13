use std::{fs, path::PathBuf};

use day22::app::run::run;
use day22::cli::args::{Args, Command};
use day22::domain::types::SortSpec;

#[test]
fn smoke_add_single_item() {
    // 1) Create temp directory
    let dir = std::env::temp_dir().join("day22_smoke_add_single");
    let _ = fs::create_dir_all(&dir);
    let path: PathBuf = dir.join("inventory.txt");
    
    // Clean up from any previous run
    let _ = fs::remove_file(&path);

    // 2) Args for adding an item
    let args = Args {
        cmd: Command::Add {
            id: "1".to_string(),
            sku: "SKU1".to_string(),
            name: "Apple".to_string(),
            price: "100".to_string(),
        },
        file: Some(path.clone()),
        sort: SortSpec::NameAsc,
    };

    // 3) Call run
    let out = run(args).expect("run should succeed");

    // 4) Assert baseline
    assert!(out.starts_with("OK\n"), "out={out:?}");
    assert!(out.contains("Added item"), "out={out:?}");
    assert!(out.contains("id=1"), "out={out:?}");
    assert!(out.contains("sku=SKU1"), "out={out:?}"); // Original SKU before normalization
    assert!(out.contains("Apple"), "out={out:?}");

    // 5) Verify file was created and contains the item
    let content = fs::read_to_string(&path).expect("file should exist");
    assert!(content.contains("id=1"), "content={content:?}");
    assert!(content.contains("sku=sku1"), "content={content:?}");
    assert!(content.contains("name=Apple"), "content={content:?}");
    assert!(content.contains("price=100"), "content={content:?}");

    // Cleanup
    let _ = fs::remove_file(&path);
    let _ = fs::remove_dir(&dir);
}

#[test]
fn smoke_add_multiple_items() {
    // 1) Create temp directory
    let dir = std::env::temp_dir().join("day22_smoke_add_multiple");
    let _ = fs::create_dir_all(&dir);
    let path: PathBuf = dir.join("inventory.txt");
    
    // Clean up from any previous run
    let _ = fs::remove_file(&path);

    // 2) Add first item
    let args1 = Args {
        cmd: Command::Add {
            id: "1".to_string(),
            sku: "SKU1".to_string(),
            name: "Apple".to_string(),
            price: "100".to_string(),
        },
        file: Some(path.clone()),
        sort: SortSpec::NameAsc,
    };
    run(args1).expect("first add should succeed");

    // 3) Add second item
    let args2 = Args {
        cmd: Command::Add {
            id: "2".to_string(),
            sku: "SKU2".to_string(),
            name: "Banana".to_string(),
            price: "200".to_string(),
        },
        file: Some(path.clone()),
        sort: SortSpec::NameAsc,
    };
    let out2 = run(args2).expect("second add should succeed");
    assert!(out2.contains("id=2"), "out2={out2:?}");

    // 4) Verify file contains both items
    let content = fs::read_to_string(&path).expect("file should exist");
    assert!(content.contains("id=1"), "content={content:?}");
    assert!(content.contains("id=2"), "content={content:?}");
    assert!(content.contains("Apple"), "content={content:?}");
    assert!(content.contains("Banana"), "content={content:?}");

    // Cleanup
    let _ = fs::remove_file(&path);
    let _ = fs::remove_dir(&dir);
}

#[test]
fn smoke_add_then_print() {
    // 1) Create temp directory
    let dir = std::env::temp_dir().join("day22_smoke_add_print");
    let _ = fs::create_dir_all(&dir);
    let path: PathBuf = dir.join("inventory.txt");
    
    // Clean up from any previous run
    let _ = fs::remove_file(&path);

    // 2) Add item
    let args_add = Args {
        cmd: Command::Add {
            id: "42".to_string(),
            sku: "TEST123".to_string(),
            name: "Test Item".to_string(),
            price: "999".to_string(),
        },
        file: Some(path.clone()),
        sort: SortSpec::NameAsc,
    };
    run(args_add).expect("add should succeed");

    // 3) Print items
    let args_print = Args {
        cmd: Command::Print,
        file: Some(path.clone()),
        sort: SortSpec::NameAsc,
    };
    let out = run(args_print).expect("print should succeed");

    // 4) Verify output
    assert!(out.contains("ITEM id=42"), "out={out:?}");
    assert!(out.contains("sku=test123"), "out={out:?}"); // normalized
    assert!(out.contains("name=\"Test Item\""), "out={out:?}");
    assert!(out.contains("price_cents=999"), "out={out:?}");

    // Cleanup
    let _ = fs::remove_file(&path);
    let _ = fs::remove_dir(&dir);
}

#[test]
fn smoke_add_duplicate_id() {
    // 1) Create temp directory
    let dir = std::env::temp_dir().join("day22_smoke_add_dup_id");
    let _ = fs::create_dir_all(&dir);
    let path: PathBuf = dir.join("inventory.txt");
    
    // Clean up from any previous run
    let _ = fs::remove_file(&path);

    // 2) Add first item
    let args1 = Args {
        cmd: Command::Add {
            id: "1".to_string(),
            sku: "SKU1".to_string(),
            name: "Apple".to_string(),
            price: "100".to_string(),
        },
        file: Some(path.clone()),
        sort: SortSpec::NameAsc,
    };
    run(args1).expect("first add should succeed");

    // 3) Try to add item with duplicate ID
    let args2 = Args {
        cmd: Command::Add {
            id: "1".to_string(),
            sku: "SKU2".to_string(),
            name: "Banana".to_string(),
            price: "200".to_string(),
        },
        file: Some(path.clone()),
        sort: SortSpec::NameAsc,
    };
    let result = run(args2);
    assert!(result.is_err(), "duplicate id should fail");
    let err = result.unwrap_err().to_string();
    assert!(err.contains("duplicate"), "err={err:?}");

    // Cleanup
    let _ = fs::remove_file(&path);
    let _ = fs::remove_dir(&dir);
}

#[test]
fn smoke_add_duplicate_sku() {
    // 1) Create temp directory
    let dir = std::env::temp_dir().join("day22_smoke_add_dup_sku");
    let _ = fs::create_dir_all(&dir);
    let path: PathBuf = dir.join("inventory.txt");
    
    // Clean up from any previous run
    let _ = fs::remove_file(&path);

    // 2) Add first item
    let args1 = Args {
        cmd: Command::Add {
            id: "1".to_string(),
            sku: "SKU1".to_string(),
            name: "Apple".to_string(),
            price: "100".to_string(),
        },
        file: Some(path.clone()),
        sort: SortSpec::NameAsc,
    };
    run(args1).expect("first add should succeed");

    // 3) Try to add item with duplicate SKU (case insensitive)
    let args2 = Args {
        cmd: Command::Add {
            id: "2".to_string(),
            sku: "sku1".to_string(), // Same SKU, different case
            name: "Banana".to_string(),
            price: "200".to_string(),
        },
        file: Some(path.clone()),
        sort: SortSpec::NameAsc,
    };
    let result = run(args2);
    assert!(result.is_err(), "duplicate sku should fail");
    let err = result.unwrap_err().to_string();
    assert!(err.contains("duplicate sku"), "err={err:?}");

    // Cleanup
    let _ = fs::remove_file(&path);
    let _ = fs::remove_dir(&dir);
}

#[test]
fn smoke_add_invalid_price() {
    // 1) Create temp directory
    let dir = std::env::temp_dir().join("day22_smoke_add_invalid_price");
    let _ = fs::create_dir_all(&dir);
    let path: PathBuf = dir.join("inventory.txt");
    
    // Clean up from any previous run
    let _ = fs::remove_file(&path);

    // 2) Try to add item with invalid price
    let args = Args {
        cmd: Command::Add {
            id: "1".to_string(),
            sku: "SKU1".to_string(),
            name: "Apple".to_string(),
            price: "not_a_number".to_string(),
        },
        file: Some(path.clone()),
        sort: SortSpec::NameAsc,
    };
    let result = run(args);
    assert!(result.is_err(), "invalid price should fail");
    let err = result.unwrap_err().to_string();
    assert!(err.contains("invalid price") || err.contains("Invalid price"), "err={err:?}");

    // Cleanup
    let _ = fs::remove_file(&path);
    let _ = fs::remove_dir(&dir);
}

#[test]
fn smoke_add_invalid_id() {
    // 1) Create temp directory
    let dir = std::env::temp_dir().join("day22_smoke_add_invalid_id");
    let _ = fs::create_dir_all(&dir);
    let path: PathBuf = dir.join("inventory.txt");
    
    // Clean up from any previous run
    let _ = fs::remove_file(&path);

    // 2) Try to add item with invalid ID
    let args = Args {
        cmd: Command::Add {
            id: "not_a_number".to_string(),
            sku: "SKU1".to_string(),
            name: "Apple".to_string(),
            price: "100".to_string(),
        },
        file: Some(path.clone()),
        sort: SortSpec::NameAsc,
    };
    let result = run(args);
    assert!(result.is_err(), "invalid id should fail");
    let err = result.unwrap_err().to_string();
    assert!(err.contains("invalid") || err.contains("Invalid"), "err={err:?}");

    // Cleanup
    let _ = fs::remove_file(&path);
    let _ = fs::remove_dir(&dir);
}

#[test]
fn smoke_add_empty_name() {
    // 1) Create temp directory
    let dir = std::env::temp_dir().join("day22_smoke_add_empty_name");
    let _ = fs::create_dir_all(&dir);
    let path: PathBuf = dir.join("inventory.txt");
    
    // Clean up from any previous run
    let _ = fs::remove_file(&path);

    // 2) Try to add item with empty name
    let args = Args {
        cmd: Command::Add {
            id: "1".to_string(),
            sku: "SKU1".to_string(),
            name: "   ".to_string(), // Only whitespace
            price: "100".to_string(),
        },
        file: Some(path.clone()),
        sort: SortSpec::NameAsc,
    };
    let result = run(args);
    assert!(result.is_err(), "empty name should fail");
    let err = result.unwrap_err().to_string();
    assert!(err.contains("invalid") || err.contains("name"), "err={err:?}");

    // Cleanup
    let _ = fs::remove_file(&path);
    let _ = fs::remove_dir(&dir);
}

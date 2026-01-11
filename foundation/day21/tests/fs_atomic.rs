use day21::domain::types::{Item, ItemId, Sku};
use day21::index::InventoryIndex;
use day21::persist::fs::save_to_file;
use std::fs;

#[test]
fn test_atomic_save_creates_file() {
    let dir = tempfile::tempdir().unwrap();
    let file_path = dir.path().join("inventory.txt");
    
    let mut idx = InventoryIndex::new();
    let item = Item::try_new(
        ItemId::new(2),
        Sku::try_new("XYZ-999").unwrap(),
        "Widget",
        500
    ).unwrap();
    idx.insert(item).unwrap();

    // Ensure file doesn't exist yet
    assert!(!file_path.exists());

    save_to_file(&idx, &file_path).unwrap();

    // Ensure file exists after save
    assert!(file_path.exists());
    let content = fs::read_to_string(&file_path).unwrap();
    assert!(content.contains("sku=xyz-999"));
}

#[test]
fn test_save_invalid_path() {
    let idx = InventoryIndex::new();
    
    // Use an empty path or a path that is a directory, which should fail.
    // However, the most robust way to trigger the "InvalidPath" error from the code 
    // (which checks .file_name()) is to use a path that terminates in `..` or is empty?
    // Path::new("subdir/..").file_name() is None.
    
    let bad_path = std::path::Path::new("subdir/.."); 
    let res = save_to_file(&idx, &bad_path);
    assert!(res.is_err());
    
    let err_msg = res.unwrap_err().to_string();
    assert!(err_msg.contains("subdir/.."));
}

#[test]
fn test_atomic_replace_existing_file() {
    let dir = tempfile::tempdir().unwrap();
    let file_path = dir.path().join("atomic_inventory.txt");
    
    // 1. Create file with old content
    fs::write(&file_path, "OLD CONTENT").unwrap();
    
    // 2. Save new index
    let mut idx = InventoryIndex::new();
    idx.insert(Item::try_new(
        ItemId::new(99), 
        Sku::try_new("NEW-99").unwrap(), 
        "NewItem", 
        999
    ).unwrap()).unwrap();
    
    save_to_file(&idx, &file_path).unwrap();
    
    // 3. Verify content
    let content = fs::read_to_string(&file_path).unwrap();
    assert!(!content.contains("OLD CONTENT"));
    assert!(content.contains("sku=new-99"));
}

#[test]
fn test_save_persists_data() {
    let mut idx = InventoryIndex::new();
    idx.insert(Item::try_new(ItemId::new(10), Sku::try_new("TEST-1").unwrap(), "TestItem", 123).unwrap()).unwrap();

    let dir = tempfile::tempdir().unwrap();
    let file_path = dir.path().join("saved_inventory.txt");
    
    save_to_file(&idx, &file_path).unwrap();

    let content = fs::read_to_string(&file_path).unwrap();
    assert!(content.contains("id=10"));
    assert!(content.contains("sku=test-1"));
    assert!(content.contains("name=TestItem"));
    assert!(content.contains("price=123"));
}

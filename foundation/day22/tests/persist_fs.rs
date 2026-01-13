use day22::domain::index::InventoryIndex;
use day22::domain::types::{Item, ItemId, Sku};
use day22::persist::fs::{load_from_file, save_to_file};
use std::fs;
use std::path::PathBuf;

#[test]
fn save_to_file_creates_new_file() {
    let dir = std::env::temp_dir().join("day22_persist_fs_create");
    let _ = fs::create_dir_all(&dir);
    let path = dir.join("new_file.txt");
    
    // Ensure file doesn't exist
    let _ = fs::remove_file(&path);
    
    let mut idx = InventoryIndex::new();
    let item = Item::try_new(ItemId::new(1), Sku::try_new("ABC").unwrap(), "Test", 100).unwrap();
    idx.insert(item).unwrap();
    
    save_to_file(&idx, &path).expect("save should succeed");
    
    assert!(path.exists(), "file should be created");
    
    let content = fs::read_to_string(&path).expect("should read file");
    assert!(content.contains("id=1"));
    
    // Cleanup
    let _ = fs::remove_file(&path);
    let _ = fs::remove_dir(&dir);
}

#[test]
fn save_to_file_overwrites_existing() {
    let dir = std::env::temp_dir().join("day22_persist_fs_overwrite");
    let _ = fs::create_dir_all(&dir);
    let path = dir.join("overwrite.txt");
    
    // Create initial file with different content
    fs::write(&path, "old content").expect("write initial");
    
    let mut idx = InventoryIndex::new();
    let item = Item::try_new(ItemId::new(1), Sku::try_new("NEW").unwrap(), "New", 200).unwrap();
    idx.insert(item).unwrap();
    
    save_to_file(&idx, &path).expect("save should succeed");
    
    let content = fs::read_to_string(&path).expect("should read file");
    assert!(content.contains("id=1"));
    assert!(content.contains("sku=new"));
    assert!(!content.contains("old content"));
    
    // Cleanup
    let _ = fs::remove_file(&path);
    let _ = fs::remove_dir(&dir);
}

#[test]
fn save_to_file_atomic_replace() {
    let dir = std::env::temp_dir().join("day22_persist_fs_atomic");
    let _ = fs::create_dir_all(&dir);
    let path = dir.join("atomic.txt");
    
    // Create initial file
    let mut idx1 = InventoryIndex::new();
    let item1 = Item::try_new(ItemId::new(1), Sku::try_new("A").unwrap(), "A", 1).unwrap();
    idx1.insert(item1).unwrap();
    save_to_file(&idx1, &path).expect("first save");
    
    // Update file
    let mut idx2 = InventoryIndex::new();
    let item2 = Item::try_new(ItemId::new(2), Sku::try_new("B").unwrap(), "B", 2).unwrap();
    idx2.insert(item2).unwrap();
    save_to_file(&idx2, &path).expect("second save");
    
    // Verify file contains only new content
    let content = fs::read_to_string(&path).expect("should read file");
    assert!(content.contains("id=2"));
    assert!(!content.contains("id=1"));
    
    // Cleanup
    let _ = fs::remove_file(&path);
    let _ = fs::remove_dir(&dir);
}

#[test]
fn load_from_file_success() {
    let dir = std::env::temp_dir().join("day22_persist_fs_load");
    let _ = fs::create_dir_all(&dir);
    let path = dir.join("load.txt");
    
    // Create file with known content
    let content = "\
id=1 sku=ABC name=Apple price=100
id=2 sku=DEF name=Banana price=200
";
    fs::write(&path, content).expect("write file");
    
    let idx = load_from_file(&path).expect("load should succeed");
    
    let items = idx.get_all_item();
    assert_eq!(items.len(), 2);
    
    let item1 = idx.get_by_id(ItemId::new(1));
    assert!(item1.is_some());
    assert_eq!(item1.unwrap().get_name(), "Apple");
    
    // Cleanup
    let _ = fs::remove_file(&path);
    let _ = fs::remove_dir(&dir);
}

#[test]
fn load_from_file_missing() {
    let path = PathBuf::from("/tmp/nonexistent_file_day22_test.txt");
    
    let result = load_from_file(&path);
    assert!(result.is_err());
}

#[test]
fn save_load_roundtrip() {
    let dir = std::env::temp_dir().join("day22_persist_fs_roundtrip");
    let _ = fs::create_dir_all(&dir);
    let path = dir.join("roundtrip.txt");
    
    // Clean up from previous run
    let _ = fs::remove_file(&path);
    
    // Create index with items
    let mut idx1 = InventoryIndex::new();
    let item1 = Item::try_new(ItemId::new(1), Sku::try_new("ABC").unwrap(), "Apple", 100).unwrap();
    let item2 = Item::try_new(ItemId::new(2), Sku::try_new("DEF").unwrap(), "Banana", 200).unwrap();
    let item3 = Item::try_new(ItemId::new(3), Sku::try_new("GHI").unwrap(), "Cherry", 300).unwrap();
    idx1.insert(item1).unwrap();
    idx1.insert(item2).unwrap();
    idx1.insert(item3).unwrap();
    
    // Save
    save_to_file(&idx1, &path).expect("save should succeed");
    
    // Load
    let idx2 = load_from_file(&path).expect("load should succeed");
    
    // Verify
    let items1 = idx1.get_all_item();
    let items2 = idx2.get_all_item();
    
    assert_eq!(items1.len(), items2.len());
    
    for item in &items1 {
        let found = idx2.get_by_id(item.get_id());
        assert!(found.is_some());
        assert_eq!(found.unwrap(), item);
    }
    
    // Cleanup
    let _ = fs::remove_file(&path);
    let _ = fs::remove_dir(&dir);
}

#[test]
fn save_to_file_empty_index() {
    let dir = std::env::temp_dir().join("day22_persist_fs_empty");
    let _ = fs::create_dir_all(&dir);
    let path = dir.join("empty.txt");
    
    let _ = fs::remove_file(&path);
    
    let idx = InventoryIndex::new();
    save_to_file(&idx, &path).expect("save should succeed");
    
    assert!(path.exists());
    
    let content = fs::read_to_string(&path).expect("should read file");
    assert_eq!(content, "");
    
    // Cleanup
    let _ = fs::remove_file(&path);
    let _ = fs::remove_dir(&dir);
}

#[test]
fn load_from_file_empty() {
    let dir = std::env::temp_dir().join("day22_persist_fs_load_empty");
    let _ = fs::create_dir_all(&dir);
    let path = dir.join("empty_load.txt");
    
    fs::write(&path, "").expect("write empty file");
    
    let idx = load_from_file(&path).expect("load should succeed");
    
    let items = idx.get_all_item();
    assert_eq!(items.len(), 0);
    
    // Cleanup
    let _ = fs::remove_file(&path);
    let _ = fs::remove_dir(&dir);
}

#[test]
fn save_to_file_invalid_path() {
    let path = PathBuf::from("/invalid/path/that/does/not/exist/test.txt");
    
    let idx = InventoryIndex::new();
    let result = save_to_file(&idx, &path);
    
    // Should fail because parent directory doesn't exist
    assert!(result.is_err());
}

#[test]
fn load_from_file_with_comments() {
    let dir = std::env::temp_dir().join("day22_persist_fs_comments");
    let _ = fs::create_dir_all(&dir);
    let path = dir.join("comments.txt");
    
    let content = "\
# This is a comment
id=1 sku=ABC name=Apple price=100
# Another comment

id=2 sku=DEF name=Banana price=200
";
    fs::write(&path, content).expect("write file");
    
    let idx = load_from_file(&path).expect("load should succeed");
    
    let items = idx.get_all_item();
    assert_eq!(items.len(), 2);
    
    // Cleanup
    let _ = fs::remove_file(&path);
    let _ = fs::remove_dir(&dir);
}

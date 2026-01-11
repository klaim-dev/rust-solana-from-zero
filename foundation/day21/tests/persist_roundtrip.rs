use day21::domain::types::{Item, ItemId, Sku};
use day21::index::InventoryIndex;
use day21::persist::fs::{load_from_file, save_to_file};
use tempfile::NamedTempFile;

#[test]
fn test_save_load_roundtrip() {
    let mut idx = InventoryIndex::new();
    let item = Item::try_new(
        ItemId::new(1),
        Sku::try_new("ABC-123").unwrap(),
        "Gadget",
        100
    ).unwrap();
    idx.insert(item).unwrap();

    let file = NamedTempFile::new().unwrap();
    let path = file.path();

    save_to_file(&idx, path).unwrap();

    let loaded = load_from_file(path).unwrap();
    
    let loaded_item = loaded.get_by_id(ItemId::new(1)).unwrap();
    assert_eq!(loaded_item.get_sku().to_string(), "abc-123");
    assert_eq!(loaded_item.get_name(), "Gadget");
    assert_eq!(loaded_item.get_price_cents(), 100);
}

#[test]
fn test_roundtrip_multiple_items() {
    let mut idx = InventoryIndex::new();
    idx.insert(Item::try_new(ItemId::new(1), Sku::try_new("ABC-101").unwrap(), "Item1", 100).unwrap()).unwrap();
    idx.insert(Item::try_new(ItemId::new(2), Sku::try_new("ABC-102").unwrap(), "Item2", 200).unwrap()).unwrap();
    idx.insert(Item::try_new(ItemId::new(3), Sku::try_new("ABC-103").unwrap(), "Item3", 300).unwrap()).unwrap();

    let file = NamedTempFile::new().unwrap();
    save_to_file(&idx, file.path()).unwrap();

    let loaded = load_from_file(file.path()).unwrap();
    let ids: Vec<_> = loaded.ids().collect();
    assert_eq!(ids.len(), 3);
    
    let item2 = loaded.get_by_id(ItemId::new(2)).unwrap();
    assert_eq!(item2.get_name(), "Item2");
}

#[test]
fn test_roundtrip_empty() {
    let idx = InventoryIndex::new();
    let file = NamedTempFile::new().unwrap();
    save_to_file(&idx, file.path()).unwrap();

    let loaded = load_from_file(file.path()).unwrap();
    assert_eq!(loaded.ids().count(), 0);
}

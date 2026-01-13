use day22::domain::index::{IndexError, InventoryIndex};
use day22::domain::types::{Item, ItemId, Sku, SortSpec};

#[test]
fn index_new() {
    let idx = InventoryIndex::new();
    let items = idx.get_all_item();
    assert_eq!(items.len(), 0);
}

#[test]
fn index_insert_single() {
    let mut idx = InventoryIndex::new();
    
    let id = ItemId::new(1);
    let sku = Sku::try_new("ABC").expect("valid");
    let item = Item::try_new(id, sku, "Apple", 100).expect("valid");
    
    idx.insert(item).expect("insert should succeed");
    
    let items = idx.get_all_item();
    assert_eq!(items.len(), 1);
}

#[test]
fn index_insert_multiple() {
    let mut idx = InventoryIndex::new();
    
    let item1 = Item::try_new(
        ItemId::new(1),
        Sku::try_new("ABC").unwrap(),
        "Apple",
        100,
    ).unwrap();
    
    let item2 = Item::try_new(
        ItemId::new(2),
        Sku::try_new("DEF").unwrap(),
        "Banana",
        200,
    ).unwrap();
    
    idx.insert(item1).expect("insert 1 should succeed");
    idx.insert(item2).expect("insert 2 should succeed");
    
    let items = idx.get_all_item();
    assert_eq!(items.len(), 2);
}

#[test]
fn index_get_by_id() {
    let mut idx = InventoryIndex::new();
    
    let id = ItemId::new(42);
    let sku = Sku::try_new("TEST").expect("valid");
    let item = Item::try_new(id, sku, "Test", 100).expect("valid");
    
    idx.insert(item).expect("insert should succeed");
    
    let found = idx.get_by_id(ItemId::new(42));
    assert!(found.is_some());
    assert_eq!(found.unwrap().get_name(), "Test");
    
    let not_found = idx.get_by_id(ItemId::new(999));
    assert!(not_found.is_none());
}

#[test]
fn index_get_by_sku() {
    let mut idx = InventoryIndex::new();
    
    let id = ItemId::new(1);
    let sku = Sku::try_new("FIND_ME").expect("valid");
    let item = Item::try_new(id, sku, "Item", 100).expect("valid");
    
    idx.insert(item).expect("insert should succeed");
    
    let found = idx.get_by_sku("find_me");
    assert!(found.is_some());
    assert_eq!(found.unwrap().get_id(), ItemId::new(1));
    
    let found_upper = idx.get_by_sku("FIND_ME");
    assert!(found_upper.is_some());
    
    let not_found = idx.get_by_sku("NOT_THERE");
    assert!(not_found.is_none());
}

#[test]
fn index_duplicate_id() {
    let mut idx = InventoryIndex::new();
    
    let item1 = Item::try_new(
        ItemId::new(1),
        Sku::try_new("ABC").unwrap(),
        "Apple",
        100,
    ).unwrap();
    
    let item2 = Item::try_new(
        ItemId::new(1), // Duplicate ID
        Sku::try_new("DEF").unwrap(),
        "Banana",
        200,
    ).unwrap();
    
    idx.insert(item1).expect("first insert should succeed");
    
    let result = idx.insert(item2);
    assert!(result.is_err());
    
    match result.unwrap_err() {
        IndexError::DuplicateId => {}, // Expected
        other => panic!("expected DuplicateId, got {:?}", other),
    }
}

#[test]
fn index_duplicate_sku() {
    let mut idx = InventoryIndex::new();
    
    let item1 = Item::try_new(
        ItemId::new(1),
        Sku::try_new("ABC").unwrap(),
        "Apple",
        100,
    ).unwrap();
    
    let item2 = Item::try_new(
        ItemId::new(2),
        Sku::try_new("abc").unwrap(), // Duplicate SKU (case insensitive)
        "Banana",
        200,
    ).unwrap();
    
    idx.insert(item1).expect("first insert should succeed");
    
    let result = idx.insert(item2);
    assert!(result.is_err());
    
    match result.unwrap_err() {
        IndexError::DuplicateSku { .. } => {}, // Expected
        other => panic!("expected DuplicateSku, got {:?}", other),
    }
}

#[test]
fn index_ids() {
    let mut idx = InventoryIndex::new();
    
    let item1 = Item::try_new(ItemId::new(1), Sku::try_new("A").unwrap(), "A", 1).unwrap();
    let item2 = Item::try_new(ItemId::new(2), Sku::try_new("B").unwrap(), "B", 2).unwrap();
    let item3 = Item::try_new(ItemId::new(3), Sku::try_new("C").unwrap(), "C", 3).unwrap();
    
    idx.insert(item1).unwrap();
    idx.insert(item2).unwrap();
    idx.insert(item3).unwrap();
    
    let ids: Vec<ItemId> = idx.ids().collect();
    assert_eq!(ids.len(), 3);
    assert!(ids.contains(&ItemId::new(1)));
    assert!(ids.contains(&ItemId::new(2)));
    assert!(ids.contains(&ItemId::new(3)));
}

#[test]
fn index_list_sorted_by_name() {
    let mut idx = InventoryIndex::new();
    
    let item1 = Item::try_new(ItemId::new(1), Sku::try_new("A").unwrap(), "Zebra", 100).unwrap();
    let item2 = Item::try_new(ItemId::new(2), Sku::try_new("B").unwrap(), "Apple", 200).unwrap();
    let item3 = Item::try_new(ItemId::new(3), Sku::try_new("C").unwrap(), "Mango", 150).unwrap();
    
    idx.insert(item1).unwrap();
    idx.insert(item2).unwrap();
    idx.insert(item3).unwrap();
    
    let sorted = idx.list_sorted(SortSpec::NameAsc);
    assert_eq!(sorted.len(), 3);
    assert_eq!(sorted[0].get_name(), "Apple");
    assert_eq!(sorted[1].get_name(), "Mango");
    assert_eq!(sorted[2].get_name(), "Zebra");
}

#[test]
fn index_list_sorted_by_price() {
    let mut idx = InventoryIndex::new();
    
    let item1 = Item::try_new(ItemId::new(1), Sku::try_new("A").unwrap(), "Zebra", 100).unwrap();
    let item2 = Item::try_new(ItemId::new(2), Sku::try_new("B").unwrap(), "Apple", 300).unwrap();
    let item3 = Item::try_new(ItemId::new(3), Sku::try_new("C").unwrap(), "Mango", 200).unwrap();
    
    idx.insert(item1).unwrap();
    idx.insert(item2).unwrap();
    idx.insert(item3).unwrap();
    
    let sorted = idx.list_sorted(SortSpec::PriceDescNameAsc);
    assert_eq!(sorted.len(), 3);
    // Price descending
    assert_eq!(sorted[0].get_price_cents(), 300);
    assert_eq!(sorted[1].get_price_cents(), 200);
    assert_eq!(sorted[2].get_price_cents(), 100);
}

#[test]
fn index_list_sorted_by_price_same_price() {
    let mut idx = InventoryIndex::new();
    
    // Same price, should sort by name
    let item1 = Item::try_new(ItemId::new(1), Sku::try_new("A").unwrap(), "Zebra", 100).unwrap();
    let item2 = Item::try_new(ItemId::new(2), Sku::try_new("B").unwrap(), "Apple", 100).unwrap();
    let item3 = Item::try_new(ItemId::new(3), Sku::try_new("C").unwrap(), "Mango", 100).unwrap();
    
    idx.insert(item1).unwrap();
    idx.insert(item2).unwrap();
    idx.insert(item3).unwrap();
    
    let sorted = idx.list_sorted(SortSpec::PriceDescNameAsc);
    assert_eq!(sorted.len(), 3);
    // Same price, sorted by name ascending
    assert_eq!(sorted[0].get_name(), "Apple");
    assert_eq!(sorted[1].get_name(), "Mango");
    assert_eq!(sorted[2].get_name(), "Zebra");
}

#[test]
fn index_get_all_item_clones() {
    let mut idx = InventoryIndex::new();
    
    let item = Item::try_new(ItemId::new(1), Sku::try_new("A").unwrap(), "Item", 100).unwrap();
    idx.insert(item).unwrap();
    
    let items1 = idx.get_all_item();
    let items2 = idx.get_all_item();
    
    // Should get independent clones
    assert_eq!(items1.len(), 1);
    assert_eq!(items2.len(), 1);
}

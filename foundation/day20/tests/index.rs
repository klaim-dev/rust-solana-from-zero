use day20::domain::types::{Item, ItemId, Sku, SortSpec};
use day20::domain::error::{ItemErr, SkuErr};
use day20::index::{IndexError, InventoryIndex};
use std::collections::HashMap;

#[test]
fn item_id_hashmap_key_works() {
    let mut map = HashMap::new();
    let id = ItemId::new(7);
    map.insert(id, 7u64);
    
    assert_eq!(map.get(&ItemId::new(7)), Some(&7));
}

#[test]
fn sku_normalizes_and_compares() {
    let sku1 = Sku::try_new(" AbC ").expect("valid sku");
    let sku2 = Sku::try_new("abc").expect("valid sku");
    
    assert_eq!(sku1, sku2);
    assert_eq!(format!("{}", sku1), "abc");
}

#[test]
fn insert_and_get_by_id() {
    let mut index = InventoryIndex::new();
    let id = ItemId::new(100);
    let sku = Sku::try_new("sku100").expect("valid");
    let item = Item::try_new(id, sku, "Item 100", 1000).expect("valid item");
    
    // Insert succeeds
    index.insert(item).unwrap();
    
    // Get by id returns Some
    let found = index.get_by_id(id);
    assert!(found.is_some());
    assert_eq!(found.unwrap().get_id(), id);
}

#[test]
fn get_by_sku_invalid_returns_none() {
    let index = InventoryIndex::new();
    // Even if index was not empty, invalid sku string should return None
    assert!(index.get_by_sku(" ").is_none());
    
    let mut index_populated = InventoryIndex::new();
    let item = Item::try_new(ItemId::new(1), Sku::try_new("abc").unwrap(), "name", 10).unwrap();
    index_populated.insert(item).unwrap();
    
    assert!(index_populated.get_by_sku(" ").is_none());
}

#[test]
fn duplicate_id_is_error() {
    let mut index = InventoryIndex::new();
    let id = ItemId::new(1);
    
    let item1 = Item::try_new(id, Sku::try_new("abc").unwrap(), "Name 1", 100).unwrap();
    index.insert(item1).unwrap();
    
    // Same ID, different SKU
    let item2 = Item::try_new(id, Sku::try_new("xyz").unwrap(), "Name 2", 200).unwrap();
    let res = index.insert(item2);
    
    assert!(matches!(res, Err(IndexError::DuplicateId)));
}

#[test]
fn duplicate_sku_is_error() {
    let mut index = InventoryIndex::new();
    
    let item1 = Item::try_new(ItemId::new(1), Sku::try_new("abc").unwrap(), "Name 1", 100).unwrap();
    index.insert(item1).unwrap();
    
    // Different ID, same SKU (normalized)
    let item2 = Item::try_new(ItemId::new(2), Sku::try_new(" AbC ").unwrap(), "Name 2", 200).unwrap();
    let res = index.insert(item2);
    
    assert!(matches!(res, Err(IndexError::DuplicateSku { .. })));
}

#[test]
fn get_by_sku_finds_inserted_item() {
    let mut index = InventoryIndex::new();
    let item = Item::try_new(ItemId::new(1), Sku::try_new(" AbC ").unwrap(), "name", 10).unwrap();
    index.insert(item).unwrap();
    
    let found = index.get_by_sku("abc");
    assert!(found.is_some());
    assert_eq!(found.unwrap().get_id(), ItemId::new(1));
}

#[test]
fn list_sorted_price_desc_name_asc_id_asc() {
    let mut index = InventoryIndex::new();
    
    let id1 = ItemId::new(1);
    let id2 = ItemId::new(2);
    let id3 = ItemId::new(3);
    let id4 = ItemId::new(4);
    
    // 1. High price (should be first)
    index.insert(Item::try_new(id1, Sku::try_new("a").unwrap(), "Z Name", 200).unwrap()).unwrap();
    
    // 2. Low price (should be last group)
    // Same price, different names
    // "A Name" < "B Name", so "A Name" comes before "B Name"
    index.insert(Item::try_new(id2, Sku::try_new("b").unwrap(), "B Name", 100).unwrap()).unwrap();
    index.insert(Item::try_new(id3, Sku::try_new("c").unwrap(), "A Name", 100).unwrap()).unwrap();
    
    // 3. Same price (100), same name ("B Name"), different ID
    // id2 < id4? id2=2, id4=4. So 2 comes before 4.
    // Wait, let's make it explicitly overlap with id2's properties except ID.
    // Use "B Name" and price 100.
    index.insert(Item::try_new(id4, Sku::try_new("d").unwrap(), "B Name", 100).unwrap()).unwrap();
    
    let sorted = index.list_sorted(SortSpec::PriceDescNameAsc);
    
    // Expected order:
    // 1. id1 (Price 200)
    // 2. id3 (Price 100, Name "A Name")
    // 3. id2 (Price 100, Name "B Name", ID 2)
    // 4. id4 (Price 100, Name "B Name", ID 4)
    
    assert_eq!(sorted.len(), 4);
    assert_eq!(sorted[0].get_id(), id1);
    assert_eq!(sorted[1].get_id(), id3);
    assert_eq!(sorted[2].get_id(), id2);
    assert_eq!(sorted[3].get_id(), id4);
}

#[test]
fn list_sorted_name_asc_id_asc() {
    let mut index = InventoryIndex::new();
    
    let id10 = ItemId::new(10);
    let id20 = ItemId::new(20);
    let id30 = ItemId::new(30);
    
    // "b", "a", "a"
    index.insert(Item::try_new(id10, Sku::try_new("1").unwrap(), "b", 10).unwrap()).unwrap();
    index.insert(Item::try_new(id30, Sku::try_new("2").unwrap(), "a", 10).unwrap()).unwrap();
    index.insert(Item::try_new(id20, Sku::try_new("3").unwrap(), "a", 10).unwrap()).unwrap();
    
    let sorted = index.list_sorted(SortSpec::NameAsc);
    
    // Expected order:
    // 1. "a" with lowest ID (id20)
    // 2. "a" with highest ID (id30)
    // 3. "b" (id10)
    
    assert_eq!(sorted.len(), 3);
    assert_eq!(sorted[0].get_id(), id20);
    assert_eq!(sorted[1].get_id(), id30);
    assert_eq!(sorted[2].get_id(), id10);
}

#[test]
fn sku_try_new_rejects_empty() {
    assert!(matches!(Sku::try_new(" "), Err(SkuErr::InvalidSku)));
}

#[test]
fn item_try_new_rejects_empty_name() {
    assert!(matches!(Item::try_new(ItemId::new(1), Sku::try_new("abc").unwrap(), " ", 10), Err(ItemErr::InvalidName)));
}

#[test]
fn list_sorted_on_empty_index_returns_empty() {
    let index = InventoryIndex::new();
    assert!(index.list_sorted(SortSpec::PriceDescNameAsc).is_empty());
    assert!(index.list_sorted(SortSpec::NameAsc).is_empty());
}

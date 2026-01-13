use day22::domain::types::{Item, ItemId, Sku};

#[test]
fn item_id_new() {
    let id = ItemId::new(42);
    assert_eq!(id.to_string(), "42");
}

#[test]
fn item_id_equality() {
    let id1 = ItemId::new(10);
    let id2 = ItemId::new(10);
    let id3 = ItemId::new(20);
    
    assert_eq!(id1, id2);
    assert_ne!(id1, id3);
}

#[test]
fn item_id_hash() {
    use std::collections::HashMap;
    
    let mut map = HashMap::new();
    let id1 = ItemId::new(1);
    let id2 = ItemId::new(2);
    
    map.insert(id1, "item1");
    map.insert(id2, "item2");
    
    assert_eq!(map.get(&ItemId::new(1)), Some(&"item1"));
    assert_eq!(map.get(&ItemId::new(2)), Some(&"item2"));
}

#[test]
fn sku_try_new_valid() {
    let sku = Sku::try_new("ABC123").expect("should be valid");
    assert_eq!(sku.to_string(), "abc123"); // normalized to lowercase
}

#[test]
fn sku_try_new_with_whitespace() {
    let sku = Sku::try_new("  ABC  ").expect("should be valid");
    assert_eq!(sku.to_string(), "abc"); // trimmed and lowercased
}

#[test]
fn sku_try_new_empty() {
    let result = Sku::try_new("");
    assert!(result.is_err(), "empty SKU should fail");
}

#[test]
fn sku_try_new_whitespace_only() {
    let result = Sku::try_new("   ");
    assert!(result.is_err(), "whitespace-only SKU should fail");
}

#[test]
fn sku_normalization() {
    let sku1 = Sku::try_new("ABC").expect("valid");
    let sku2 = Sku::try_new("abc").expect("valid");
    let sku3 = Sku::try_new("  ABC  ").expect("valid");
    
    assert_eq!(sku1, sku2);
    assert_eq!(sku1, sku3);
    assert_eq!(sku2, sku3);
}

#[test]
fn sku_hash() {
    use std::collections::HashMap;
    
    let sku1 = Sku::try_new("SKU1").expect("valid");
    let sku2 = Sku::try_new("SKU2").expect("valid");
    
    let mut map = HashMap::new();
    map.insert(sku1, "item1");
    map.insert(sku2, "item2");
    
    let lookup = Sku::try_new("sku1").expect("valid");
    assert_eq!(map.get(&lookup), Some(&"item1"));
}

#[test]
fn item_try_new_valid() {
    let id = ItemId::new(1);
    let sku = Sku::try_new("ABC").expect("valid sku");
    
    let item = Item::try_new(id, sku, "Apple", 100).expect("should be valid");
    
    assert_eq!(item.get_id(), ItemId::new(1));
    assert_eq!(item.get_sku().to_string(), "abc");
    assert_eq!(item.get_name(), "Apple");
    assert_eq!(item.get_price_cents(), 100);
}

#[test]
fn item_try_new_name_trimmed() {
    let id = ItemId::new(1);
    let sku = Sku::try_new("ABC").expect("valid sku");
    
    let item = Item::try_new(id, sku, "  Apple  ", 100).expect("should be valid");
    
    assert_eq!(item.get_name(), "Apple"); // trimmed
}

#[test]
fn item_try_new_empty_name() {
    let id = ItemId::new(1);
    let sku = Sku::try_new("ABC").expect("valid sku");
    
    let result = Item::try_new(id, sku, "", 100);
    assert!(result.is_err(), "empty name should fail");
}

#[test]
fn item_try_new_whitespace_name() {
    let id = ItemId::new(1);
    let sku = Sku::try_new("ABC").expect("valid sku");
    
    let result = Item::try_new(id, sku, "   ", 100);
    assert!(result.is_err(), "whitespace-only name should fail");
}

#[test]
fn item_getters() {
    let id = ItemId::new(42);
    let sku = Sku::try_new("TEST").expect("valid sku");
    let item = Item::try_new(id, sku, "Test Item", 999).expect("valid item");
    
    assert_eq!(item.get_id(), ItemId::new(42));
    assert_eq!(item.get_sku().to_string(), "test");
    assert_eq!(item.get_name(), "Test Item");
    assert_eq!(item.get_price_cents(), 999);
}

#[test]
fn item_equality() {
    let id = ItemId::new(1);
    let sku1 = Sku::try_new("ABC").expect("valid");
    let sku2 = Sku::try_new("ABC").expect("valid");
    
    let item1 = Item::try_new(id, sku1, "Apple", 100).expect("valid");
    let item2 = Item::try_new(id, sku2, "Apple", 100).expect("valid");
    
    assert_eq!(item1, item2);
}

#[test]
fn item_price_zero() {
    let id = ItemId::new(1);
    let sku = Sku::try_new("FREE").expect("valid sku");
    
    let item = Item::try_new(id, sku, "Free Item", 0).expect("zero price should be valid");
    assert_eq!(item.get_price_cents(), 0);
}

#[test]
fn item_price_large() {
    let id = ItemId::new(1);
    let sku = Sku::try_new("EXPENSIVE").expect("valid sku");
    
    let item = Item::try_new(id, sku, "Expensive Item", u64::MAX).expect("large price should be valid");
    assert_eq!(item.get_price_cents(), u64::MAX);
}

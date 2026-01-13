use day22::domain::index::InventoryIndex;
use day22::domain::types::{Item, ItemId, Sku};
use day22::persist::format::{deserialize, parse_item_line, serialize};

#[test]
fn parse_item_line_valid() {
    let line = "id=1 sku=ABC name=Apple price=100";
    let item = parse_item_line(line).expect("should parse");
    
    assert_eq!(item.get_id(), ItemId::new(1));
    assert_eq!(item.get_sku().to_string(), "abc");
    assert_eq!(item.get_name(), "Apple");
    assert_eq!(item.get_price_cents(), 100);
}

#[test]
fn parse_item_line_different_order() {
    let line = "sku=TEST price=999 name=Item id=42";
    let item = parse_item_line(line).expect("should parse");
    
    assert_eq!(item.get_id(), ItemId::new(42));
    assert_eq!(item.get_sku().to_string(), "test");
    assert_eq!(item.get_name(), "Item");
    assert_eq!(item.get_price_cents(), 999);
}

#[test]
fn parse_item_line_missing_id() {
    let line = "sku=ABC name=Apple price=100";
    let result = parse_item_line(line);
    assert!(result.is_err());
}

#[test]
fn parse_item_line_missing_sku() {
    let line = "id=1 name=Apple price=100";
    let result = parse_item_line(line);
    assert!(result.is_err());
}

#[test]
fn parse_item_line_missing_name() {
    let line = "id=1 sku=ABC price=100";
    let result = parse_item_line(line);
    assert!(result.is_err());
}

#[test]
fn parse_item_line_missing_price() {
    let line = "id=1 sku=ABC name=Apple";
    let result = parse_item_line(line);
    assert!(result.is_err());
}

#[test]
fn parse_item_line_duplicate_key() {
    let line = "id=1 id=2 sku=ABC name=Apple price=100";
    let result = parse_item_line(line);
    assert!(result.is_err());
}

#[test]
fn parse_item_line_unknown_key() {
    let line = "id=1 sku=ABC name=Apple price=100 unknown=value";
    let result = parse_item_line(line);
    assert!(result.is_err());
}

#[test]
fn parse_item_line_invalid_id() {
    let line = "id=not_a_number sku=ABC name=Apple price=100";
    let result = parse_item_line(line);
    assert!(result.is_err());
}

#[test]
fn parse_item_line_invalid_price() {
    let line = "id=1 sku=ABC name=Apple price=not_a_number";
    let result = parse_item_line(line);
    assert!(result.is_err());
}

#[test]
fn parse_item_line_empty_sku() {
    let line = "id=1 sku= name=Apple price=100";
    let result = parse_item_line(line);
    assert!(result.is_err());
}

#[test]
fn parse_item_line_empty_name() {
    let line = "id=1 sku=ABC name= price=100";
    let result = parse_item_line(line);
    assert!(result.is_err());
}

#[test]
fn parse_item_line_invalid_token_format() {
    let line = "id=1 sku=ABC invalidtoken name=Apple price=100";
    let result = parse_item_line(line);
    assert!(result.is_err());
}

#[test]
fn parse_item_line_empty_key() {
    let line = "id=1 =value sku=ABC name=Apple price=100";
    let result = parse_item_line(line);
    assert!(result.is_err());
}

#[test]
fn serialize_empty_index() {
    let idx = InventoryIndex::new();
    let text = serialize(&idx);
    assert_eq!(text, "");
}

#[test]
fn serialize_single_item() {
    let mut idx = InventoryIndex::new();
    let item = Item::try_new(
        ItemId::new(1),
        Sku::try_new("ABC").unwrap(),
        "Apple",
        100,
    ).unwrap();
    idx.insert(item).unwrap();
    
    let text = serialize(&idx);
    assert!(text.contains("id=1"));
    assert!(text.contains("sku=abc"));
    assert!(text.contains("name=Apple"));
    assert!(text.contains("price=100"));
}

#[test]
fn serialize_multiple_items_sorted_by_id() {
    let mut idx = InventoryIndex::new();
    
    // Insert in reverse ID order
    let item3 = Item::try_new(ItemId::new(3), Sku::try_new("C").unwrap(), "C", 3).unwrap();
    let item1 = Item::try_new(ItemId::new(1), Sku::try_new("A").unwrap(), "A", 1).unwrap();
    let item2 = Item::try_new(ItemId::new(2), Sku::try_new("B").unwrap(), "B", 2).unwrap();
    
    idx.insert(item3).unwrap();
    idx.insert(item1).unwrap();
    idx.insert(item2).unwrap();
    
    let text = serialize(&idx);
    let lines: Vec<&str> = text.lines().collect();
    
    assert_eq!(lines.len(), 3);
    // Should be sorted by ID
    assert!(lines[0].starts_with("id=1"));
    assert!(lines[1].starts_with("id=2"));
    assert!(lines[2].starts_with("id=3"));
}

#[test]
fn deserialize_empty() {
    let text = "";
    let idx = deserialize(text).expect("should deserialize");
    assert_eq!(idx.get_all_item().len(), 0);
}

#[test]
fn deserialize_single_item() {
    let text = "id=1 sku=ABC name=Apple price=100\n";
    let idx = deserialize(text).expect("should deserialize");
    
    let items = idx.get_all_item();
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].get_id(), ItemId::new(1));
}

#[test]
fn deserialize_multiple_items() {
    let text = "\
id=1 sku=ABC name=Apple price=100
id=2 sku=DEF name=Banana price=200
id=3 sku=GHI name=Cherry price=300
";
    let idx = deserialize(text).expect("should deserialize");
    
    let items = idx.get_all_item();
    assert_eq!(items.len(), 3);
}

#[test]
fn deserialize_with_blank_lines() {
    let text = "\
id=1 sku=ABC name=Apple price=100

id=2 sku=DEF name=Banana price=200

";
    let idx = deserialize(text).expect("should deserialize");
    
    let items = idx.get_all_item();
    assert_eq!(items.len(), 2);
}

#[test]
fn deserialize_with_comments() {
    let text = "\
# This is a comment
id=1 sku=ABC name=Apple price=100
# Another comment
id=2 sku=DEF name=Banana price=200
";
    let idx = deserialize(text).expect("should deserialize");
    
    let items = idx.get_all_item();
    assert_eq!(items.len(), 2);
}

#[test]
fn deserialize_with_whitespace() {
    let text = "\
  id=1 sku=ABC name=Apple price=100  
    id=2 sku=DEF name=Banana price=200
";
    let idx = deserialize(text).expect("should deserialize");
    
    let items = idx.get_all_item();
    assert_eq!(items.len(), 2);
}

#[test]
fn deserialize_duplicate_id() {
    let text = "\
id=1 sku=ABC name=Apple price=100
id=1 sku=DEF name=Banana price=200
";
    let result = deserialize(text);
    assert!(result.is_err());
}

#[test]
fn deserialize_duplicate_sku() {
    let text = "\
id=1 sku=ABC name=Apple price=100
id=2 sku=ABC name=Banana price=200
";
    let result = deserialize(text);
    assert!(result.is_err());
}

#[test]
fn deserialize_invalid_line() {
    let text = "\
id=1 sku=ABC name=Apple price=100
invalid line here
id=2 sku=DEF name=Banana price=200
";
    let result = deserialize(text);
    assert!(result.is_err());
}

#[test]
fn serialize_deserialize_roundtrip() {
    let mut idx = InventoryIndex::new();
    
    let item1 = Item::try_new(ItemId::new(1), Sku::try_new("ABC").unwrap(), "Apple", 100).unwrap();
    let item2 = Item::try_new(ItemId::new(2), Sku::try_new("DEF").unwrap(), "Banana", 200).unwrap();
    let item3 = Item::try_new(ItemId::new(3), Sku::try_new("GHI").unwrap(), "Cherry", 300).unwrap();
    
    idx.insert(item1).unwrap();
    idx.insert(item2).unwrap();
    idx.insert(item3).unwrap();
    
    let text = serialize(&idx);
    let idx2 = deserialize(&text).expect("deserialize should succeed");
    
    let items1 = idx.get_all_item();
    let items2 = idx2.get_all_item();
    
    assert_eq!(items1.len(), items2.len());
    
    // Check all items are present
    for item in &items1 {
        let found = idx2.get_by_id(item.get_id());
        assert!(found.is_some());
        assert_eq!(found.unwrap(), item);
    }
}

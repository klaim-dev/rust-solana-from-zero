
#[derive(Debug, PartialEq, PartialOrd)]
pub struct Item {
    pub id: u32,
    pub name: String,
    pub qty: u32,
}

pub struct Inventory {
    pub items: Vec<Item>,
}

pub fn get_item<'a>(inv: &'a Inventory, id: u32) -> Option<&'a Item> {
    inv.items
    .iter()
    .find(|i| i.id == id)
}
pub fn get_item_mut<'a>(inv: &'a mut Inventory, id: u32) -> Option<&'a mut Item> {
    inv.items
    .iter_mut()
    .find(|i| i.id == id)
}

pub fn add_stock(inv: &mut Inventory, id: u32, delta: u32) -> bool{
    if let Some(item) = get_item_mut(inv, id) {
        item.qty += delta;
        true
    } else  {
        false
    }
}
pub fn rename(inv: &mut Inventory, id: u32, new_name: &str) -> bool {
    if new_name.is_empty() {
       return false; 
    }

    if let Some(item) = get_item_mut(inv, id) {
        item.name = new_name.to_string();
        true
    } else {
        false
    }
}
pub fn bulk_add_stock(inv: &mut Inventory, ids: &[u32], delta: u32) -> usize {
    let mut count = 0;
    for n in ids {
        if let Some(item)= inv.items.iter_mut().find(|x| x.id == *n) {
            item.qty += delta;
            count += 1;
        }      
    }
    count
}

/// Returns two mutable references to items by their IDs.
///
/// **Important**: The order of returned items is determined by their positions in the vector
/// (indices), not by the order of input arguments. The first element in the tuple will always
/// have the lower index, and the second will have the higher index.
///
/// Returns `None` if:
/// - Both IDs are the same
/// - Either item is not found
///
/// # Example
/// ```
/// // If item with id=3 is at index 0 and item with id=1 is at index 2,
/// // then get_two_mut(inv, 3, 1) will return (item_at_index_0, item_at_index_2)
/// // regardless of the argument order
/// ```
pub fn get_two_mut<'a>(inv: &'a mut Inventory, id1: u32, id2: u32) -> Option<(&'a mut Item, &'a mut Item)> {
    if id1 == id2 {
        return None;
    }

    let idx1 = inv.items.iter().position(|x| x.id == id1)?;
    let idx2 = inv.items.iter().position(|x| x.id == id2)?;

    let (min, max) = if idx1 < idx2 {(idx1, idx2)} else {(idx2, idx1)};
    let (left, right) = inv.items.split_at_mut(max);
    let  first = &mut left[min];
    let second = &mut right[0];
    Some((first, second))
}


pub fn split_by_index<'a>( inv: &'a mut Inventory, mid: usize) -> (&'a mut [Item], &'a mut [Item]) {
    let (left, right) = inv.items.split_at_mut(mid);
    (left, right)
}

pub fn names_view<'a>(inv: &'a Inventory) -> Vec<&'a str> {
    inv.items.iter().map(|x| x.name.as_str()).collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_inventory() -> Inventory {
        Inventory {
            items: vec![
                Item { id: 1, name: "Alice".to_string(), qty: 32 },
                Item { id: 2, name: "Bob".to_string(), qty: 12 },
                Item { id: 3, name: "Dreg".to_string(), qty: 4 },
            ]
        }
    }

    #[test]
    fn get_item_basic() {
        let inv = create_test_inventory();
        let item = get_item(&inv, 1).expect("item should exist");

        assert_eq!(item.id, 1);
        assert_eq!(item.name, "Alice");
        assert_eq!(item.qty, 32);
    }

    #[test]
    fn get_item_not_found() {
        let inv = create_test_inventory();
        let item = get_item(&inv, 999);
        assert!(item.is_none());
    }

    #[test]
    fn get_item_mut_basic() {
        let mut inv = create_test_inventory();
        let item = get_item_mut(&mut inv, 2).expect("item should exist");
        
        assert_eq!(item.id, 2);
        assert_eq!(item.name, "Bob");
        item.qty = 100;
        assert_eq!(item.qty, 100);
    }

    #[test]
    fn get_item_mut_not_found() {
        let mut inv = create_test_inventory();
        let item = get_item_mut(&mut inv, 999);
        assert!(item.is_none());
    }

    #[test]
    fn add_stock_success() {
        let mut inv = create_test_inventory();
        let result = add_stock(&mut inv, 1, 10);
        
        assert!(result);
        let item = get_item(&inv, 1).unwrap();
        assert_eq!(item.qty, 42); // 32 + 10
    }

    #[test]
    fn add_stock_not_found() {
        let mut inv = create_test_inventory();
        let result = add_stock(&mut inv, 999, 10);
        
        assert!(!result);
    }

    #[test]
    fn rename_success() {
        let mut inv = create_test_inventory();
        let result = rename(&mut inv, 2, "Robert");
        
        assert!(result);
        let item = get_item(&inv, 2).unwrap();
        assert_eq!(item.name, "Robert");
    }

    #[test]
    fn rename_empty_name() {
        let mut inv = create_test_inventory();
        let result = rename(&mut inv, 2, "");
        
        assert!(!result);
        let item = get_item(&inv, 2).unwrap();
        assert_eq!(item.name, "Bob"); // unchanged
    }

    #[test]
    fn rename_not_found() {
        let mut inv = create_test_inventory();
        let result = rename(&mut inv, 999, "NewName");
        
        assert!(!result);
    }

    #[test]
    fn bulk_add_stock_success() {
        let mut inv = create_test_inventory();
        let ids = vec![1, 2];
        let count = bulk_add_stock(&mut inv, &ids, 5);
        
        assert_eq!(count, 2);
        assert_eq!(get_item(&inv, 1).unwrap().qty, 37); // 32 + 5
        assert_eq!(get_item(&inv, 2).unwrap().qty, 17); // 12 + 5
    }

    #[test]
    fn bulk_add_stock_partial() {
        let mut inv = create_test_inventory();
        let ids = vec![1, 999, 2];
        let count = bulk_add_stock(&mut inv, &ids, 5);
        
        assert_eq!(count, 2); // only 1 and 2 exist
        assert_eq!(get_item(&inv, 1).unwrap().qty, 37);
        assert_eq!(get_item(&inv, 2).unwrap().qty, 17);
    }

    #[test]
    fn bulk_add_stock_empty() {
        let mut inv = create_test_inventory();
        let ids = vec![999, 998];
        let count = bulk_add_stock(&mut inv, &ids, 5);
        
        assert_eq!(count, 0);
    }

    #[test]
    fn get_two_mut_success() {
        let mut inv = create_test_inventory();
        
        // Explicitly scope the mutable borrow to limit its lifetime
        {
            let (item1, item2) = get_two_mut(&mut inv, 1, 2).expect("items should exist");
            item1.qty = 100;
            item2.qty = 200;
        } // Mutable borrow ends here
        
        assert_eq!(get_item(&inv, 1).unwrap().qty, 100);
        assert_eq!(get_item(&inv, 2).unwrap().qty, 200);
    }

    #[test]
    fn get_two_mut_same_id() {
        let mut inv = create_test_inventory();
        let result = get_two_mut(&mut inv, 1, 1);
        
        assert!(result.is_none());
    }

    #[test]
    fn get_two_mut_not_found() {
        let mut inv = create_test_inventory();
        let result = get_two_mut(&mut inv, 1, 999);
        
        assert!(result.is_none());
    }

    #[test]
    fn get_two_mut_reverse_order() {
        let mut inv = create_test_inventory();
        
        // Explicitly scope the mutable borrow to limit its lifetime
        // Note: Order is by index, not by argument order
        {
            let (item1, item2) = get_two_mut(&mut inv, 3, 1).expect("items should exist");
            
            // Items are returned in index order, not argument order
            // id=1 is at index 0, id=3 is at index 2
            assert_eq!(item1.id, 1); // first by index (lower index)
            assert_eq!(item2.id, 3); // second by index (higher index)
        } // Mutable borrow ends here
    }

    #[test]
    fn test_split_by_index() {
        let mut inv = create_test_inventory();
        
        // Explicitly scope the mutable borrow to limit its lifetime
        {
            let (left, right) = super::split_by_index(&mut inv, 2);
            
            assert_eq!(left.len(), 2);
            assert_eq!(right.len(), 1);
            assert_eq!(left[0].id, 1);
            assert_eq!(left[1].id, 2);
            assert_eq!(right[0].id, 3);
            
            left[0].qty = 999;
            right[0].qty = 888;
        } // Mutable borrow ends here
        
        assert_eq!(get_item(&inv, 1).unwrap().qty, 999);
        assert_eq!(get_item(&inv, 3).unwrap().qty, 888);
    }

    #[test]
    fn test_split_by_index_empty_right() {
        let mut inv = Inventory {
            items: vec![
                Item { id: 1, name: "Alice".to_string(), qty: 32 },
            ]
        };
        
        // Explicitly scope the mutable borrow to limit its lifetime
        {
            let (left, right) = super::split_by_index(&mut inv, 1);
            
            assert_eq!(left.len(), 1);
            assert_eq!(right.len(), 0);
        } // Mutable borrow ends here
    }

    #[test]
    fn test_names_view() {
        let inv = create_test_inventory();
        let names = super::names_view(&inv);
        
        assert_eq!(names, vec!["Alice", "Bob", "Dreg"]);
    }

    #[test]
    fn test_names_view_empty() {
        let inv = Inventory { items: vec![] };
        let names = super::names_view(&inv);
        
        assert_eq!(names, Vec::<&str>::new());
    }



}












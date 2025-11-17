mod inventory;

use inventory::{Inventory, Item};

fn main() {
    println!("=== Inventory Management Demo ===\n");

    // Create inventory
    let mut inv = Inventory {
        items: vec![
            Item { id: 1, name: "Apple".to_string(), qty: 50 },
            Item { id: 2, name: "Banana".to_string(), qty: 30 },
            Item { id: 3, name: "Orange".to_string(), qty: 25 },
        ]
    };

    println!("Initial inventory state:");
    for item in &inv.items {
        println!("  ID: {}, Name: {}, Quantity: {}", item.id, item.name, item.qty);
    }

    // Get item
    println!("\n1. Getting item by ID (ID=2):");
    if let Some(item) = inventory::get_item(&inv, 2) {
        println!("   Found: {} (quantity: {})", item.name, item.qty);
    }

    // Add stock
    println!("\n2. Adding 20 units to item ID=1:");
    if inventory::add_stock(&mut inv, 1, 20) {
        println!("   Successfully added!");
        let item = inventory::get_item(&inv, 1).unwrap();
        println!("   New quantity: {}", item.qty);
    }

    // Rename
    println!("\n3. Renaming item ID=3:");
    if inventory::rename(&mut inv, 3, "Mandarin") {
        println!("   Successfully renamed!");
        let item = inventory::get_item(&inv, 3).unwrap();
        println!("   New name: {}", item.name);
    }

    // Bulk add stock
    println!("\n4. Bulk adding 10 units to items ID=1 and ID=2:");
    let ids = vec![1, 2];
    let count = inventory::bulk_add_stock(&mut inv, &ids, 10);
    println!("   Updated items: {}", count);

    // Get two items simultaneously
    println!("\n5. Getting two items simultaneously (ID=1 and ID=2):");
    if let Some((item1, item2)) = inventory::get_two_mut(&mut inv, 1, 2) {
        println!("   Item 1: {} (quantity: {})", item1.name, item1.qty);
        println!("   Item 2: {} (quantity: {})", item2.name, item2.qty);
        item1.qty += 5;
        item2.qty += 3;
        println!("   Added 5 to first, 3 to second");
    }

    // Split by index
    println!("\n6. Splitting inventory at index 2:");
    let (left, right) = inventory::split_by_index(&mut inv, 2);
    println!("   Left part ({} items):", left.len());
    for item in left.iter() {
        println!("     - {}", item.name);
    }
    println!("   Right part ({} items):", right.len());
    for item in right.iter() {
        println!("     - {}", item.name);
    }
    let _ = left;
    let _ = right;

    // Get names list
    println!("\n7. Getting list of all names:");
    let names = inventory::names_view(&inv);
    println!("   Names: {:?}", names);

    // Final state
    println!("\n=== Final inventory state ===");
    for item in &inv.items {
        println!("  ID: {}, Name: {}, Quantity: {}", item.id, item.name, item.qty);
    }

    // Error handling demonstration
    println!("\n=== Error Handling Demonstration ===");
    
    println!("\n8. Attempting to add stock to non-existent item:");
    if !inventory::add_stock(&mut inv, 999, 10) {
        println!("   Error: item not found (as expected)");
    }

    println!("\n9. Attempting to rename with empty name:");
    if !inventory::rename(&mut inv, 1, "") {
        println!("   Error: empty name not allowed (as expected)");
    }

    println!("\n10. Attempting to get two identical items:");
    if inventory::get_two_mut(&mut inv, 1, 1).is_none() {
        println!("   Error: cannot get two identical items (as expected)");
    }

    println!("\n=== Demo completed ===");
}

use day16::domain::errors::CatalogError;
use day16::domain::item::{Category, CreateItem, Filter, Sku, UpdateItem};
use day16::store::catalog::Catalog;

// Helper function to create SKU for tests
fn sku(s: &str) -> Sku {
    Sku::new(s.to_string()).unwrap()
}

#[test]
fn test_update_item_name() {
    let mut catalog = Catalog::new();
    
    let id = catalog
        .create_item(CreateItem {
            sku: "SKU001".to_string(),
            name: "Original Name".to_string(),
            category: Category::Books,
            price_cents: 1000,
            is_active: true,
        })
        .unwrap();

    let updated = catalog
        .update_item(
            id,
            UpdateItem {
                sku: None,
                name: Some("Updated Name".to_string()),
                category: None,
                price_cents: None,
                is_active: None,
            },
        )
        .unwrap();

    assert_eq!(updated.name(), "Updated Name");
    assert_eq!(catalog.get_by_id(&id).unwrap().name(), "Updated Name");
}

#[test]
fn test_update_item_sku_rebinding() {
    let mut catalog = Catalog::new();
    
    let id1 = catalog
        .create_item(CreateItem {
            sku: "SKU001".to_string(),
            name: "Item 1".to_string(),
            category: Category::Books,
            price_cents: 1000,
            is_active: true,
        })
        .unwrap();

    let id2 = catalog
        .create_item(CreateItem {
            sku: "SKU002".to_string(),
            name: "Item 2".to_string(),
            category: Category::Electronics,
            price_cents: 2000,
            is_active: true,
        })
        .unwrap();

    // Update SKU of item 1
    let updated = catalog
        .update_item(
            id1,
            UpdateItem {
                sku: Some("SKU001_NEW".to_string()),
                name: None,
                category: None,
                price_cents: None,
                is_active: None,
            },
        )
        .unwrap();

    assert_eq!(updated.sku().to_string(), "sku001_new");
    
    // Verify index rebinding: old SKU should not exist, new SKU should point to id1
    assert!(catalog.get_by_sku(&sku("SKU001")).is_none());
    assert_eq!(
        catalog.get_by_sku(&sku("SKU001_NEW")).unwrap().id(),
        id1
    );
    
    // Verify item 2 is still accessible by its original SKU
    assert_eq!(
        catalog.get_by_sku(&sku("SKU002")).unwrap().id(),
        id2
    );
}

#[test]
fn test_update_item_sku_collision_error() {
    let mut catalog = Catalog::new();
    
    let id1 = catalog
        .create_item(CreateItem {
            sku: "SKU001".to_string(),
            name: "Item 1".to_string(),
            category: Category::Books,
            price_cents: 1000,
            is_active: true,
        })
        .unwrap();

    let id2 = catalog
        .create_item(CreateItem {
            sku: "SKU002".to_string(),
            name: "Item 2".to_string(),
            category: Category::Electronics,
            price_cents: 2000,
            is_active: true,
        })
        .unwrap();

    // Store original state
    let original_item1 = catalog.get_by_id(&id1).unwrap().clone();
    let original_item2 = catalog.get_by_id(&id2).unwrap().clone();

    // Try to update id1's SKU to id2's SKU - should fail
    let result = catalog.update_item(
        id1,
        UpdateItem {
            sku: Some("SKU002".to_string()),
            name: None,
            category: None,
            price_cents: None,
            is_active: None,
        },
    );

    assert_eq!(
        result,
        Err(CatalogError::SkuCollision {
            sku: sku("sku002")
        })
    );

    // Verify state hasn't changed
    assert_eq!(catalog.get_by_id(&id1).unwrap().sku(), original_item1.sku());
    assert_eq!(catalog.get_by_id(&id2).unwrap().sku(), original_item2.sku());
    assert_eq!(catalog.get_by_id(&id1).unwrap().name(), original_item1.name());
    assert_eq!(catalog.get_by_id(&id2).unwrap().name(), original_item2.name());
}

#[test]
fn test_update_item_all_fields() {
    let mut catalog = Catalog::new();
    
    let id = catalog
        .create_item(CreateItem {
            sku: "SKU001".to_string(),
            name: "Original Name".to_string(),
            category: Category::Books,
            price_cents: 1000,
            is_active: true,
        })
        .unwrap();

    let updated = catalog
        .update_item(
            id,
            UpdateItem {
                sku: None,
                name: Some("New Name".to_string()),
                category: Some(Category::Electronics),
                price_cents: Some(2500),
                is_active: Some(false),
            },
        )
        .unwrap();

    assert_eq!(updated.name(), "New Name");
    assert_eq!(updated.category(), &Category::Electronics);
    assert_eq!(updated.price_cents(), 2500);
    assert_eq!(updated.is_active(), false);
}

#[test]
fn test_list_items_filter_category() {
    let mut catalog = Catalog::new();
    
    let _id1 = catalog
        .create_item(CreateItem {
            sku: "SKU001".to_string(),
            name: "Book 1".to_string(),
            category: Category::Books,
            price_cents: 1000,
            is_active: true,
        })
        .unwrap();

    let _id2 = catalog
        .create_item(CreateItem {
            sku: "SKU002".to_string(),
            name: "Electronics 1".to_string(),
            category: Category::Electronics,
            price_cents: 2000,
            is_active: true,
        })
        .unwrap();

    let _id3 = catalog
        .create_item(CreateItem {
            sku: "SKU003".to_string(),
            name: "Book 2".to_string(),
            category: Category::Books,
            price_cents: 1500,
            is_active: true,
        })
        .unwrap();

    let filter = Filter {
        category: Some(Category::Books),
        active_only: false,
        price_min: None,
        price_max: None,
        name_contains: None,
    };

    let items = catalog.list_items(filter);
    assert_eq!(items.len(), 2);
    assert!(items.iter().all(|item| item.category() == &Category::Books));
}

#[test]
fn test_list_items_filter_active_only() {
    let mut catalog = Catalog::new();
    
    let _id1 = catalog
        .create_item(CreateItem {
            sku: "SKU001".to_string(),
            name: "Active Item".to_string(),
            category: Category::Books,
            price_cents: 1000,
            is_active: true,
        })
        .unwrap();

    let _id2 = catalog
        .create_item(CreateItem {
            sku: "SKU002".to_string(),
            name: "Inactive Item".to_string(),
            category: Category::Electronics,
            price_cents: 2000,
            is_active: false,
        })
        .unwrap();

    let filter = Filter {
        category: None,
        active_only: true,
        price_min: None,
        price_max: None,
        name_contains: None,
    };

    let items = catalog.list_items(filter);
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].name(), "Active Item");
}

#[test]
fn test_list_items_filter_price_range() {
    let mut catalog = Catalog::new();
    
    let _id1 = catalog
        .create_item(CreateItem {
            sku: "SKU001".to_string(),
            name: "Cheap".to_string(),
            category: Category::Books,
            price_cents: 500,
            is_active: true,
        })
        .unwrap();

    let _id2 = catalog
        .create_item(CreateItem {
            sku: "SKU002".to_string(),
            name: "Medium".to_string(),
            category: Category::Electronics,
            price_cents: 1500,
            is_active: true,
        })
        .unwrap();

    let _id3 = catalog
        .create_item(CreateItem {
            sku: "SKU003".to_string(),
            name: "Expensive".to_string(),
            category: Category::Grocery,
            price_cents: 3000,
            is_active: true,
        })
        .unwrap();

    let filter = Filter {
        category: None,
        active_only: false,
        price_min: Some(1000),
        price_max: Some(2000),
        name_contains: None,
    };

    let items = catalog.list_items(filter);
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].name(), "Medium");
}

#[test]
fn test_list_items_filter_name_contains() {
    let mut catalog = Catalog::new();
    
    let _id1 = catalog
        .create_item(CreateItem {
            sku: "SKU001".to_string(),
            name: "Rust Book".to_string(),
            category: Category::Books,
            price_cents: 1000,
            is_active: true,
        })
        .unwrap();

    let _id2 = catalog
        .create_item(CreateItem {
            sku: "SKU002".to_string(),
            name: "Python Guide".to_string(),
            category: Category::Books,
            price_cents: 2000,
            is_active: true,
        })
        .unwrap();

    let filter = Filter {
        category: None,
        active_only: false,
        price_min: None,
        price_max: None,
        name_contains: Some("rust".to_string()),
    };

    let items = catalog.list_items(filter);
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].name(), "Rust Book");
}

#[test]
fn test_list_items_sorting_stable() {
    let mut catalog = Catalog::new();
    
    // Create items with same price to test tie-breaker
    let id1 = catalog
        .create_item(CreateItem {
            sku: "SKU001".to_string(),
            name: "Item 1".to_string(),
            category: Category::Books,
            price_cents: 1000,
            is_active: true,
        })
        .unwrap();

    let id2 = catalog
        .create_item(CreateItem {
            sku: "SKU002".to_string(),
            name: "Item 2".to_string(),
            category: Category::Electronics,
            price_cents: 1000, // Same price
            is_active: true,
        })
        .unwrap();

    let id3 = catalog
        .create_item(CreateItem {
            sku: "SKU003".to_string(),
            name: "Item 3".to_string(),
            category: Category::Grocery,
            price_cents: 500, // Lower price
            is_active: true,
        })
        .unwrap();

    let id4 = catalog
        .create_item(CreateItem {
            sku: "SKU004".to_string(),
            name: "Item 4".to_string(),
            category: Category::Other,
            price_cents: 2000, // Higher price
            is_active: true,
        })
        .unwrap();

    let filter = Filter {
        category: None,
        active_only: false,
        price_min: None,
        price_max: None,
        name_contains: None,
    };

    let items = catalog.list_items(filter);
    assert_eq!(items.len(), 4);
    
    // Should be sorted by price_cents ascending, then by id ascending
    assert_eq!(items[0].id(), id3); // price 500
    assert_eq!(items[1].id(), id1); // price 1000, id 1
    assert_eq!(items[2].id(), id2); // price 1000, id 2 (tie-breaker)
    assert_eq!(items[3].id(), id4); // price 2000
}

#[test]
fn test_list_items_anti_n1_architecture() {
    // This test verifies that list_items does NOT call get_by_id in a loop
    // by tracking the number of get_by_id calls
    
    let mut catalog = Catalog::new();
    
    // Create multiple items
    for i in 0..10 {
        catalog
            .create_item(CreateItem {
                sku: format!("SKU{:03}", i),
                name: format!("Item {}", i),
                category: if i % 2 == 0 {
                    Category::Books
                } else {
                    Category::Electronics
                },
                price_cents: (i + 1) * 100,
                is_active: i % 3 != 0,
            })
            .unwrap();
    }

    // Reset counter before list_items call
    catalog.reset_get_by_id_call_count();
    
    let filter = Filter {
        category: Some(Category::Books),
        active_only: false,
        price_min: Some(200),
        price_max: Some(800),
        name_contains: None,
    };

    let items = catalog.list_items(filter);
    
    // Verify filtering works correctly
    assert!(items.len() > 0);
    assert!(items.iter().all(|item| item.category() == &Category::Books));
    assert!(items.iter().all(|item| {
        item.price_cents() >= 200 && item.price_cents() <= 800
    }));
    
    // Verify sorting
    for i in 1..items.len() {
        assert!(
            items[i - 1].price_cents() <= items[i].price_cents(),
            "Items should be sorted by price_cents"
        );
        if items[i - 1].price_cents() == items[i].price_cents() {
            assert!(
                items[i - 1].id().as_u64() <= items[i].id().as_u64(),
                "Tie-breaker should be by id"
            );
        }
    }
    
    // The key assertion: get_by_id should NOT be called during list_items
    assert_eq!(
        catalog.get_by_id_call_count(),
        0,
        "list_items should not call get_by_id (anti-N+1 guarantee)"
    );
}

#[test]
fn test_update_item_empty_name_error() {
    let mut catalog = Catalog::new();
    
    let id = catalog
        .create_item(CreateItem {
            sku: "SKU001".to_string(),
            name: "Original Name".to_string(),
            category: Category::Books,
            price_cents: 1000,
            is_active: true,
        })
        .unwrap();

    let result = catalog.update_item(
        id,
        UpdateItem {
            sku: None,
            name: Some("   ".to_string()), // Empty after trim
            category: None,
            price_cents: None,
            is_active: None,
        },
    );

    assert_eq!(result, Err(CatalogError::EmptyName));
    
    // Verify original name is unchanged
    assert_eq!(catalog.get_by_id(&id).unwrap().name(), "Original Name");
}

// ============================================================================
// CRUD Tests
// ============================================================================

#[test]
fn test_create_and_get_by_id_and_sku() {
    let mut catalog = Catalog::new();
    
    let id = catalog
        .create_item(CreateItem {
            sku: "SKU001".to_string(),
            name: "Test Item".to_string(),
            category: Category::Books,
            price_cents: 1000,
            is_active: true,
        })
        .unwrap();

    // Test get_by_id
    let item_by_id = catalog.get_by_id(&id).unwrap();
    assert_eq!(item_by_id.name(), "Test Item");
    assert_eq!(item_by_id.sku().to_string(), "sku001");
    assert_eq!(item_by_id.category(), &Category::Books);
    assert_eq!(item_by_id.price_cents(), 1000);
    assert_eq!(item_by_id.is_active(), true);

    // Test get_by_sku
    let item_by_sku = catalog.get_by_sku(&sku("SKU001")).unwrap();
    assert_eq!(item_by_sku.id(), id);
    assert_eq!(item_by_sku.name(), "Test Item");
    
    // Verify both methods return the same item
    assert_eq!(item_by_id.id(), item_by_sku.id());
}

#[test]
fn test_duplicate_sku_is_rejected() {
    let mut catalog = Catalog::new();
    
    // Create first item
    let _id1 = catalog
        .create_item(CreateItem {
            sku: "ABC".to_string(),
            name: "First Item".to_string(),
            category: Category::Books,
            price_cents: 1000,
            is_active: true,
        })
        .unwrap();

    // Try to create second item with same SKU (different case and whitespace)
    // SKU normalization should make " abc " equal to "ABC"
    let result = catalog.create_item(CreateItem {
        sku: " abc ".to_string(),
        name: "Second Item".to_string(),
        category: Category::Electronics,
        price_cents: 2000,
        is_active: false,
    });

    assert_eq!(
        result,
        Err(CatalogError::DuplicateSku {
            sku: sku("abc") // Normalized SKU
        })
    );

    // Verify first item is still accessible (by normalized SKU)
    assert_eq!(
        catalog.get_by_sku(&sku("ABC")).unwrap().name(),
        "First Item"
    );
    // Also accessible by the normalized version of " abc " (which becomes "abc")
    assert_eq!(
        catalog.get_by_sku(&sku(" abc ")).unwrap().name(),
        "First Item"
    );
    
    // Verify second item was not created (we can check by trying to create it again)
    let result2 = catalog.create_item(CreateItem {
        sku: " abc ".to_string(),
        name: "Second Item".to_string(),
        category: Category::Electronics,
        price_cents: 2000,
        is_active: false,
    });
    assert_eq!(
        result2,
        Err(CatalogError::DuplicateSku {
            sku: sku("abc")
        })
    );
}

#[test]
fn test_delete_removes_secondary_index() {
    let mut catalog = Catalog::new();
    
    let id = catalog
        .create_item(CreateItem {
            sku: "SKU001".to_string(),
            name: "Test Item".to_string(),
            category: Category::Books,
            price_cents: 1000,
            is_active: true,
        })
        .unwrap();

    // Verify item exists and is accessible by both ID and SKU
    assert!(catalog.get_by_id(&id).is_some());
    assert!(catalog.get_by_sku(&sku("SKU001")).is_some());

    // Delete the item
    let deleted_item = catalog.delete_item(id).unwrap();
    assert_eq!(deleted_item.name(), "Test Item");

    // Verify item is removed from both indexes
    assert!(catalog.get_by_id(&id).is_none());
    assert!(catalog.get_by_sku(&sku("SKU001")).is_none());
    
    // Verify we can still create a new item with the same SKU
    let new_id = catalog
        .create_item(CreateItem {
            sku: "SKU001".to_string(),
            name: "New Item".to_string(),
            category: Category::Electronics,
            price_cents: 2000,
            is_active: false,
        })
        .unwrap();
    
    assert_ne!(new_id, id);
    assert_eq!(
        catalog.get_by_sku(&sku("SKU001")).unwrap().name(),
        "New Item"
    );
}

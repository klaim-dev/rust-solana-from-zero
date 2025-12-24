use crate::app::error::CliError;
use crate::app::view::{ItemView, Response};
use crate::cli::command::{Command, CreateArgs, ListFilter, UpdateItemArgs};
use catalog::domain::item::{CreateItem, Filter, UpdateItem};
use catalog::store::catalog::Catalog;

pub fn execute(cat: &mut Catalog, cmd: Command) -> Result<Response, CliError> {
    match cmd {
        Command::Create(args) => {
            let create_item = to_create_item(args);
            let id = cat.create_item(create_item)?;
            Ok(Response::Created { id })
        }
        Command::GetById { id } => {
            if let Some(item) = cat.get_by_id(&id) {
                Ok(Response::Item(ItemView::from(item)))
            } else {
                Err(CliError::NotFound {
                    what: "item",
                    value: id.to_string(),
                })
            }
        }
        Command::GetBySku { sku } => {
            if let Some(item) = cat.get_by_sku(&sku) {
                Ok(Response::Item(ItemView::from(item)))
            } else {
                Err(CliError::NotFound {
                    what: "item",
                    value: sku.to_string(),
                })
            }
        }
        Command::Update { id, changes } => {
            let patch = to_update_item(changes);
            let item = cat.update_item(id, patch)?;
            Ok(Response::Updated(ItemView::from(&item)))
        }

        Command::List { filter } => {
            let filter = to_filter_item(filter);
            let list = cat.list_items(filter);
            let result_filter = list.into_iter().map(ItemView::from).collect::<Vec<_>>();
            Ok(Response::Items(result_filter))
        }

        Command::Delete { id } => {
            let item = cat.delete_item(id)?;
            Ok(Response::Deleted {
                item: ItemView::from(&item),
            })
        }

        Command::Help => Ok(Response::Help("Help".to_string())),
        Command::Exit => Ok(Response::Exit),
    }
}

fn to_create_item(value: CreateArgs) -> CreateItem {
    CreateItem {
        sku: value.sku.to_string(),
        name: value.name,
        category: value.category,
        price_cents: value.price_cents,
        is_active: value.active,
    }
}

fn to_update_item(value: UpdateItemArgs) -> UpdateItem {
    let sku = if let Some(sku) = value.sku {
        Some(sku.to_string())
    } else {
        None
    };

    UpdateItem {
        sku,
        name: value.name,
        category: value.category,
        price_cents: value.price_cents,
        is_active: value.active,
    }
}

fn to_filter_item(value: ListFilter) -> Filter {
    let active = value.active.unwrap_or(false);
    Filter {
        category: value.category,
        active_only: active,
        price_min: value.min_price,
        price_max: value.max_price,
        name_contains: value.name_contains,
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::cli::command::{CreateArgs, ListFilter, UpdateItemArgs};
    use catalog::domain::item::{Category, ItemId};

    #[test]
    fn execute_create() {
        let mut catalog = Catalog::new();
        let cmd = Command::Create(CreateArgs {
            sku: "test-sku".parse().unwrap(),
            name: "Test Item".to_string(),
            category: Category::Books,
            price_cents: 1000,
            active: true,
        });
        let result = execute(&mut catalog, cmd).unwrap();
        match result {
            Response::Created { id } => {
                assert!(id.as_u64() > 0);
                let item = catalog.get_by_id(&id).unwrap();
                assert_eq!(item.name(), "Test Item");
            }
            _ => panic!("Expected Created response"),
        }
    }

    #[test]
    fn execute_create_duplicate_sku() {
        let mut catalog = Catalog::new();
        let cmd1 = Command::Create(CreateArgs {
            sku: "test-sku".parse().unwrap(),
            name: "Item 1".to_string(),
            category: Category::Books,
            price_cents: 1000,
            active: true,
        });
        execute(&mut catalog, cmd1).unwrap();

        let cmd2 = Command::Create(CreateArgs {
            sku: "test-sku".parse().unwrap(),
            name: "Item 2".to_string(),
            category: Category::Electronics,
            price_cents: 2000,
            active: true,
        });
        let result = execute(&mut catalog, cmd2);
        assert!(result.is_err());
    }

    #[test]
    fn execute_get_by_id_success() {
        let mut catalog = Catalog::new();
        let id = catalog
            .create_item(CreateItem {
                sku: "test-sku".to_string(),
                name: "Test Item".to_string(),
                category: Category::Books,
                price_cents: 1000,
                is_active: true,
            })
            .unwrap();

        let cmd = Command::GetById { id };
        let result = execute(&mut catalog, cmd).unwrap();
        match result {
            Response::Item(view) => {
                assert_eq!(view.id, id);
                assert_eq!(view.name, "Test Item");
            }
            _ => panic!("Expected Item response"),
        }
    }

    #[test]
    fn execute_get_by_id_not_found() {
        let mut catalog = Catalog::new();
        let id = ItemId::new(999).unwrap();
        let cmd = Command::GetById { id };
        let result = execute(&mut catalog, cmd);
        assert!(result.is_err());
        match result {
            Err(CliError::NotFound { what, value }) => {
                assert_eq!(what, "item");
                assert_eq!(value, "999");
            }
            _ => panic!("Expected NotFound error"),
        }
    }

    #[test]
    fn execute_get_by_sku_success() {
        let mut catalog = Catalog::new();
        let id = catalog
            .create_item(CreateItem {
                sku: "test-sku".to_string(),
                name: "Test Item".to_string(),
                category: Category::Books,
                price_cents: 1000,
                is_active: true,
            })
            .unwrap();

        let cmd = Command::GetBySku {
            sku: "test-sku".parse().unwrap(),
        };
        let result = execute(&mut catalog, cmd).unwrap();
        match result {
            Response::Item(view) => {
                assert_eq!(view.id, id);
                assert_eq!(view.name, "Test Item");
            }
            _ => panic!("Expected Item response"),
        }
    }

    #[test]
    fn execute_get_by_sku_not_found() {
        let mut catalog = Catalog::new();
        let cmd = Command::GetBySku {
            sku: "nonexistent".parse().unwrap(),
        };
        let result = execute(&mut catalog, cmd);
        assert!(result.is_err());
    }

    #[test]
    fn execute_update_success() {
        let mut catalog = Catalog::new();
        let id = catalog
            .create_item(CreateItem {
                sku: "test-sku".to_string(),
                name: "Original Name".to_string(),
                category: Category::Books,
                price_cents: 1000,
                is_active: true,
            })
            .unwrap();

        let cmd = Command::Update {
            id,
            changes: UpdateItemArgs {
                sku: None,
                name: Some("Updated Name".to_string()),
                category: Some(Category::Electronics),
                price_cents: Some(2000),
                active: Some(false),
            },
        };
        let result = execute(&mut catalog, cmd).unwrap();
        match result {
            Response::Updated(view) => {
                assert_eq!(view.name, "Updated Name");
                assert_eq!(view.category, Category::Electronics);
                assert_eq!(view.price_cents, 2000);
                assert_eq!(view.is_active, false);
            }
            _ => panic!("Expected Updated response"),
        }
    }

    #[test]
    fn execute_update_not_found() {
        let mut catalog = Catalog::new();
        let id = ItemId::new(999).unwrap();
        let cmd = Command::Update {
            id,
            changes: UpdateItemArgs {
                sku: None,
                name: Some("New Name".to_string()),
                category: None,
                price_cents: None,
                active: None,
            },
        };
        let result = execute(&mut catalog, cmd);
        assert!(result.is_err());
    }

    #[test]
    fn execute_list_empty() {
        let mut catalog = Catalog::new();
        let cmd = Command::List {
            filter: ListFilter::default(),
        };
        let result = execute(&mut catalog, cmd).unwrap();
        match result {
            Response::Items(items) => {
                assert_eq!(items.len(), 0);
            }
            _ => panic!("Expected Items response"),
        }
    }

    #[test]
    fn execute_list_with_items() {
        let mut catalog = Catalog::new();
        catalog
            .create_item(CreateItem {
                sku: "sku1".to_string(),
                name: "Item 1".to_string(),
                category: Category::Books,
                price_cents: 1000,
                is_active: true,
            })
            .unwrap();
        catalog
            .create_item(CreateItem {
                sku: "sku2".to_string(),
                name: "Item 2".to_string(),
                category: Category::Electronics,
                price_cents: 2000,
                is_active: true,
            })
            .unwrap();

        let cmd = Command::List {
            filter: ListFilter {
                category: Some(Category::Books),
                active: None,
                sku: None,
                name_contains: None,
                min_price: None,
                max_price: None,
            },
        };
        let result = execute(&mut catalog, cmd).unwrap();
        match result {
            Response::Items(items) => {
                assert_eq!(items.len(), 1);
                assert_eq!(items[0].name, "Item 1");
            }
            _ => panic!("Expected Items response"),
        }
    }

    #[test]
    fn execute_delete_success() {
        let mut catalog = Catalog::new();
        let id = catalog
            .create_item(CreateItem {
                sku: "test-sku".to_string(),
                name: "Test Item".to_string(),
                category: Category::Books,
                price_cents: 1000,
                is_active: true,
            })
            .unwrap();

        let cmd = Command::Delete { id };
        let result = execute(&mut catalog, cmd).unwrap();
        match result {
            Response::Deleted { item } => {
                assert_eq!(item.name, "Test Item");
            }
            _ => panic!("Expected Deleted response"),
        }

        // Verify item is deleted
        assert!(catalog.get_by_id(&id).is_none());
    }

    #[test]
    fn execute_delete_not_found() {
        let mut catalog = Catalog::new();
        let id = ItemId::new(999).unwrap();
        let cmd = Command::Delete { id };
        let result = execute(&mut catalog, cmd);
        assert!(result.is_err());
    }

    #[test]
    fn execute_help() {
        let mut catalog = Catalog::new();
        let cmd = Command::Help;
        let result = execute(&mut catalog, cmd).unwrap();
        match result {
            Response::Help(msg) => {
                assert_eq!(msg, "Help");
            }
            _ => panic!("Expected Help response"),
        }
    }

    #[test]
    fn execute_exit() {
        let mut catalog = Catalog::new();
        let cmd = Command::Exit;
        let result = execute(&mut catalog, cmd).unwrap();
        match result {
            Response::Exit => {}
            _ => panic!("Expected Exit response"),
        }
    }

    #[test]
    fn to_create_item_conversion() {
        let args = CreateArgs {
            sku: "test".parse().unwrap(),
            name: "Test".to_string(),
            category: Category::Books,
            price_cents: 100,
            active: true,
        };
        let create_item = to_create_item(args);
        assert_eq!(create_item.sku, "test");
        assert_eq!(create_item.name, "Test");
        assert_eq!(create_item.category, Category::Books);
        assert_eq!(create_item.price_cents, 100);
        assert_eq!(create_item.is_active, true);
    }

    #[test]
    fn to_update_item_conversion() {
        let args = UpdateItemArgs {
            sku: Some("new-sku".parse().unwrap()),
            name: Some("New Name".to_string()),
            category: Some(Category::Electronics),
            price_cents: Some(2000),
            active: Some(false),
        };
        let update_item = to_update_item(args);
        assert_eq!(update_item.sku, Some("new-sku".to_string()));
        assert_eq!(update_item.name, Some("New Name".to_string()));
        assert_eq!(update_item.category, Some(Category::Electronics));
        assert_eq!(update_item.price_cents, Some(2000));
        assert_eq!(update_item.is_active, Some(false));
    }

    #[test]
    fn to_update_item_partial() {
        let args = UpdateItemArgs {
            sku: None,
            name: Some("New Name".to_string()),
            category: None,
            price_cents: None,
            active: None,
        };
        let update_item = to_update_item(args);
        assert_eq!(update_item.sku, None);
        assert_eq!(update_item.name, Some("New Name".to_string()));
        assert_eq!(update_item.category, None);
        assert_eq!(update_item.price_cents, None);
        assert_eq!(update_item.is_active, None);
    }

    #[test]
    fn to_filter_item_conversion() {
        let filter = ListFilter {
            category: Some(Category::Books),
            active: Some(true),
            sku: None,
            name_contains: Some("test".to_string()),
            min_price: Some(100),
            max_price: Some(1000),
        };
        let result = to_filter_item(filter);
        assert_eq!(result.category, Some(Category::Books));
        assert_eq!(result.active_only, true);
        assert_eq!(result.name_contains, Some("test".to_string()));
        assert_eq!(result.price_min, Some(100));
        assert_eq!(result.price_max, Some(1000));
    }

    #[test]
    fn to_filter_item_active_defaults_to_false() {
        let filter = ListFilter::default();
        let result = to_filter_item(filter);
        assert_eq!(result.active_only, false);
    }

    // Negative test cases for engine/domain errors
    #[test]
    fn execute_create_duplicate_sku_conflict() {
        let mut catalog = Catalog::new();
        let cmd1 = Command::Create(CreateArgs {
            sku: "DUPLICATE".parse().unwrap(),
            name: "First Item".to_string(),
            category: Category::Books,
            price_cents: 1000,
            active: true,
        });
        execute(&mut catalog, cmd1).unwrap();

        let cmd2 = Command::Create(CreateArgs {
            sku: "DUPLICATE".parse().unwrap(),
            name: "Second Item".to_string(),
            category: Category::Electronics,
            price_cents: 2000,
            active: true,
        });
        let result = execute(&mut catalog, cmd2);

        assert!(result.is_err());
        match result {
            Err(CliError::Catalog(catalog::domain::errors::CatalogError::DuplicateSku {
                ..
            })) => {}
            _ => panic!("Expected DuplicateSku error"),
        }
    }

    #[test]
    fn execute_update_missing_id_not_found() {
        let mut catalog = Catalog::new();
        let non_existent_id = ItemId::new(999).unwrap();
        let cmd = Command::Update {
            id: non_existent_id,
            changes: UpdateItemArgs {
                sku: None,
                name: Some("Updated Name".to_string()),
                category: None,
                price_cents: None,
                active: None,
            },
        };
        let result = execute(&mut catalog, cmd);

        assert!(result.is_err());
        match result {
            Err(CliError::Catalog(catalog::domain::errors::CatalogError::ItemNotFound {
                ..
            })) => {}
            _ => panic!("Expected ItemNotFound error"),
        }
    }

    #[test]
    fn execute_delete_unknown_id_not_found() {
        let mut catalog = Catalog::new();
        let non_existent_id = ItemId::new(999).unwrap();
        let cmd = Command::Delete {
            id: non_existent_id,
        };
        let result = execute(&mut catalog, cmd);

        assert!(result.is_err());
        match result {
            Err(CliError::Catalog(catalog::domain::errors::CatalogError::ItemNotFound {
                ..
            })) => {}
            _ => panic!("Expected ItemNotFound error"),
        }
    }

    #[test]
    fn execute_update_sku_collision() {
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

        let cmd = Command::Update {
            id: id1,
            changes: UpdateItemArgs {
                sku: Some("SKU002".parse().unwrap()),
                name: None,
                category: None,
                price_cents: None,
                active: None,
            },
        };
        let result = execute(&mut catalog, cmd);

        assert!(result.is_err());
        match result {
            Err(CliError::Catalog(catalog::domain::errors::CatalogError::SkuCollision {
                ..
            })) => {}
            _ => panic!("Expected SkuCollision error"),
        }
    }
}

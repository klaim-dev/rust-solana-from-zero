use catalog::domain::item::{Category, Item, ItemId, Sku};
#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) struct ItemView {
    pub(crate) id: ItemId,
    pub(crate) sku: Sku,
    pub(crate) name: String,
    pub(crate) category: Category,
    pub(crate) price_cents: u64,
    pub(crate) is_active: bool,
}

impl From<&Item> for ItemView {
    fn from(value: &Item) -> Self {
        Self {
            id: value.id(),
            sku: value.sku().clone(),
            name: value.name().to_string(),
            category: value.category().clone(),
            price_cents: value.price_cents(),
            is_active: value.is_active(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Response {
    Created { id: ItemId },
    Item(ItemView),
    Items(Vec<ItemView>),
    Updated(ItemView),
    Deleted { item: ItemView },
    Help(String),
    Exit,
}

#[cfg(test)]
mod test {
    use super::*;
    use catalog::domain::item::{Category, CreateItem};
    use catalog::store::catalog::Catalog;

    fn create_test_item(catalog: &mut Catalog, name: &str, category: Category) -> Item {
        let id = catalog
            .create_item(CreateItem {
                sku: format!("sku-{}", name).to_string(),
                name: name.to_string(),
                category,
                price_cents: 1000,
                is_active: true,
            })
            .unwrap();
        catalog.get_by_id(&id).unwrap().clone()
    }

    #[test]
    fn item_view_from_item() {
        let mut catalog = Catalog::new();
        let item = create_test_item(&mut catalog, "Test Item", Category::Books);

        let view = ItemView::from(&item);
        assert_eq!(view.id, item.id());
        assert_eq!(view.sku, *item.sku());
        assert_eq!(view.name, item.name());
        assert_eq!(view.category, *item.category());
        assert_eq!(view.price_cents, item.price_cents());
        assert_eq!(view.is_active, item.is_active());
    }

    #[test]
    fn item_view_all_categories() {
        let mut catalog = Catalog::new();
        let categories = vec![
            Category::Books,
            Category::Electronics,
            Category::Grocery,
            Category::Other,
        ];

        for (idx, category) in categories.iter().enumerate() {
            let name = format!("Test{}", idx);
            let item = create_test_item(&mut catalog, &name, category.clone());
            let view = ItemView::from(&item);
            assert_eq!(view.category, *category);
        }
    }

    #[test]
    fn item_view_inactive() {
        let mut catalog = Catalog::new();
        let id = catalog
            .create_item(CreateItem {
                sku: "test".to_string(),
                name: "Test".to_string(),
                category: Category::Books,
                price_cents: 1000,
                is_active: false,
            })
            .unwrap();
        let item = catalog.get_by_id(&id).unwrap();
        let view = ItemView::from(item);
        assert_eq!(view.is_active, false);
    }

    #[test]
    fn item_view_clone() {
        let mut catalog = Catalog::new();
        let item = create_test_item(&mut catalog, "Test", Category::Books);
        let view1 = ItemView::from(&item);
        let view2 = view1.clone();
        assert_eq!(view1, view2);
    }
}

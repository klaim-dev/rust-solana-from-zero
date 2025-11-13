pub struct Item {
    pub id: u32,
    pub name: String,
}

pub struct Bag {
    items: Vec<Item>,
}

impl Bag {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

pub fn add_item(bag: &mut Bag, id: u32, name: &str) -> bool {
    if bag.items.iter().any(|i| i.id == id) {
        return false;
    }

    let new_item = Item {
        id,
        name: name.to_string(),
    };
    bag.items.push(new_item);
    true
}

pub fn get_name<'a>(bag: &'a Bag, id: u32) -> Option<&'a str> {
    bag.items
        .iter()
        .find(|i| i.id == id)
        .map(|i| i.name.as_str())
}

pub fn rename_item(bag: &mut Bag, id: u32, new_name: &str) -> bool {
    bag.items
        .iter_mut()
        .find(|i| i.id == id)
        .map(|item| {
            item.name = new_name.to_string();
        })
        .is_some()
}

pub fn id_list(bag: &Bag) -> Vec<u32> {
    bag.items.iter().map(|item| item.id).collect()
}

pub fn into_names(bag: Bag) -> Vec<String> {
    bag.items.into_iter().map(|item| item.name).collect()
}

pub fn get_name_owned(bag: &Bag, id: u32) -> Option<String> {
    bag.items
        .iter()
        .find(|item| item.id == id)
        .map(|item| item.name.clone())
}

pub fn swap_names(bag: &mut Bag, id1: u32, id2: u32) -> bool {
    if id1 == id2 {
        return false;
    }

    let pos1 = find_index_by_id(bag, id1);
    let pos2 = find_index_by_id(bag, id2);

    match (pos1, pos2) {
        (Some(i1), Some(i2)) => {
            if i1 == i2 {
                return false;
            }

            let (first_index, second_index) = if i1 < i2 { (i1, i2) } else { (i2, i1) };
            let (left, right) = bag.items.split_at_mut(second_index);
            let first = &mut left[first_index];
            let second = &mut right[0];
            std::mem::swap(&mut first.name, &mut second.name);
            true
        }
        _ => false,
    }
}

fn find_index_by_id(bag: &Bag, id: u32) -> Option<usize> {
    bag.items.iter().position(|item| item.id == id)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bag_with_items(items: &[(u32, &str)]) -> Bag {
        let mut bag = Bag::new();
        for (id, name) in items {
            assert!(add_item(&mut bag, *id, name));
        }
        bag
    }

    #[test]
    fn add_item_inserts_unique_ids() {
        let mut bag = Bag::new();
        assert!(bag.is_empty());
        assert!(add_item(&mut bag, 1, "first"));
        assert!(!add_item(&mut bag, 1, "duplicate"));
        assert_eq!(bag.len(), 1);
        assert!(!bag.is_empty());
    }

    #[test]
    fn bag_len_and_empty_reflects_items() {
        let mut bag = Bag::new();
        assert_eq!(bag.len(), 0);
        assert!(bag.is_empty());

        assert!(add_item(&mut bag, 7, "seven"));
        assert_eq!(bag.len(), 1);
        assert!(!bag.is_empty());

        assert!(add_item(&mut bag, 42, "answer"));
        assert_eq!(bag.len(), 2);
    }

    #[test]
    fn get_name_returns_reference() {
        let bag = bag_with_items(&[(1, "first")]);
        assert_eq!(get_name(&bag, 1), Some("first"));
        assert_eq!(get_name(&bag, 2), None);
    }

    #[test]
    fn rename_item_updates_existing() {
        let mut bag = bag_with_items(&[(1, "first")]);
        assert!(rename_item(&mut bag, 1, "updated"));
        assert_eq!(get_name(&bag, 1), Some("updated"));
        assert!(!rename_item(&mut bag, 2, "none"));
    }

    #[test]
    fn id_list_returns_all_ids() {
        let bag = bag_with_items(&[(1, "first"), (3, "third"), (2, "second")]);
        assert_eq!(id_list(&bag), vec![1, 3, 2]);
    }

    #[test]
    fn into_names_takes_ownership() {
        let bag = bag_with_items(&[(1, "first"), (2, "second")]);
        let names = into_names(bag);
        assert_eq!(names, vec!["first".to_string(), "second".to_string()]);
    }

    #[test]
    fn get_name_owned_clones_value() {
        let bag = bag_with_items(&[(1, "first")]);
        assert_eq!(get_name_owned(&bag, 1), Some("first".to_string()));
        assert_eq!(get_name_owned(&bag, 2), None);
    }

    #[test]
    fn swap_names_exchanges_names_only() {
        let mut bag = bag_with_items(&[(1, "first"), (2, "second")]);
        assert!(swap_names(&mut bag, 1, 2));
        assert_eq!(get_name(&bag, 1), Some("second"));
        assert_eq!(get_name(&bag, 2), Some("first"));
    }

    #[test]
    fn swap_names_handles_reverse_order() {
        let mut bag = bag_with_items(&[(1, "first"), (2, "second")]);
        assert!(swap_names(&mut bag, 2, 1));
        assert_eq!(get_name(&bag, 1), Some("second"));
        assert_eq!(get_name(&bag, 2), Some("first"));
    }

    #[test]
    fn swap_names_handles_missing_ids_and_same_id() {
        let mut bag = bag_with_items(&[(1, "first"), (2, "second")]);
        assert!(!swap_names(&mut bag, 1, 3));
        assert!(!swap_names(&mut bag, 1, 1));
    }
}

mod bag;

use bag::{Bag, add_item, get_name, get_name_owned, id_list, into_names, rename_item, swap_names};

fn main() {
    let mut bag = Bag::new();

    add_item(&mut bag, 1, "first");
    add_item(&mut bag, 2, "second");
    add_item(&mut bag, 3, "third");

    if let Some(name) = get_name(&bag, 2) {
        println!("Item 2 is currently named `{name}`.");
    }

    rename_item(&mut bag, 2, "updated second");
    println!(
        "After rename: {:?}",
        get_name_owned(&bag, 2).unwrap_or_else(|| "<missing>".into())
    );

    println!("Current ids: {:?}", id_list(&bag));

    swap_names(&mut bag, 1, 3);
    println!(
        "After swap: id 1 -> {:?}, id 3 -> {:?}",
        get_name(&bag, 1),
        get_name(&bag, 3)
    );

    let names = into_names(bag);
    println!("Names left in bag: {:?}", names);
}

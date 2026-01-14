use crate::cli::args::Args;
use crate::cli::args::Command;
use crate::cli::error::RunError;
use crate::domain::types::{Item, ItemId, Sku};
use crate::persist::fs::{load_from_file, save_to_file};
use std::path::Path;
pub fn run(args: Args) -> Result<String, RunError> {
    match args.cmd {
        Command::Print => {
            let path = args.file.ok_or_else(|| RunError::MissingFileForPrint)?;
            let ind = load_from_file(&path)?;
            // Use list_sorted to avoid cloning all items
            let items = ind.list_sorted(args.sort);
            let out = render(&items);
            Ok(out)
        }
        Command::Add { id, sku, name, price } => {
            let path = args.file.ok_or_else(|| RunError::MissingFileForAdd)?;
            
            // Parse ID
            let id_num = id.parse::<u64>().map_err(|e| RunError::InvalidId {
                input: id.clone(),
                reason: e.to_string(),
            })?;
            let item_id = ItemId::new(id_num);
            
            // Parse SKU
            let item_sku = Sku::try_new(&sku).map_err(|e| RunError::InvalidSku {
                reason: e.to_string(),
            })?;
            
            // Parse price
            let price_cents = price.parse::<u64>().map_err(|e| RunError::InvalidPrice {
                input: price.clone(),
                reason: e.to_string(),
            })?;
            
            // Create item
            let item = Item::try_new(item_id, item_sku, &name, price_cents)?;
            
            // Load or create index
            let mut ind = if Path::new(&path).exists() {
                load_from_file(&path)?
            } else {
                crate::domain::index::InventoryIndex::new()
            };
            
            // Insert item
            ind.insert(item)?;
            
            // Save to file
            save_to_file(&ind, &path)?;
            
            Ok(format!("OK\nAdded item: id={} sku={} name=\"{}\" price={}\n", 
                id_num, sku, name, price_cents))
        }
    }
}

fn render(items: &[&Item]) -> String {
    let mut out = String::new();

    out.push_str("OK\n");

    for it in items {
        out.push_str(&format!(
            "ITEM id={} sku={} name=\"{}\" price_cents={}\n",
            it.get_id(),
            it.get_sku(),
            it.get_name(),
            it.get_price_cents(),
        ));
    }
    out
}

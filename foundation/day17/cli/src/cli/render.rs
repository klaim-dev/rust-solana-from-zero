use crate::app::error::CliError;
use crate::app::view::Response;

pub fn render(resp: Response) -> String {
    match resp {
        Response::Created { id } => {
            format!("OK CREATED id=<{}>", id.as_u64())
        }
        Response::Deleted { item } => {
            format!("OK DELETED item =<{}>", item.id.to_string())
        }
        Response::Updated(item) => {
            format!("OK UPDATE item =<{}>", item.id.to_string())
        }
        Response::Item(item) => {
            format!(
                "ITEM id= {} | sku= {} | name= \"{}\" | category= {} | price= {} | active= {}",
                item.id, item.sku, item.name, item.category, item.price_cents, item.is_active
            )
        }
        Response::Items(items) => {
            if items.is_empty() {
                return "OK EMPTY".to_string();
            }
            items
                .iter()
                .map(format_item_line)
                .collect::<Vec<_>>()
                .join(" OK \n")
        }

        Response::Help(text) => {
            format!("OK: {}", text)
        }
        Response::Exit => {
            format!("OK BYE")
        }
    }
}

fn format_item_line(item: &crate::app::view::ItemView) -> String {
    format!(
        "ITEM id={} sku={} name=\"{}\" category={} price_cents={} active={}",
        item.id, item.sku, item.name, item.category, item.price_cents, item.is_active
    )
}

pub fn render_error(err: CliError) -> String {
    format!("ERR {}", err)
}

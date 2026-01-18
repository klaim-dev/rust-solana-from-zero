use crate::domain::order::{Order, OrderStatus};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AppOutput {
    Text(String),
}

pub fn render_order(order: &Order) -> String {
    let mut lines = Vec::new();
    lines.push(format!(
        "ORDER id={} customer=\"{}\" status={}",
        order.get_id(),
        order.get_customer(),
        status_to_str(order.get_status())
    ));

    let mut items: Vec<_> = order.get_items().collect();
    items.sort_by(|(left, _), (right, _)| left.get().cmp(right.get()));
    for (sku, qty) in items {
        lines.push(format!(
            "ITEM order_id={} sku={} qty={}",
            order.get_id(),
            sku.get(),
            qty.get()
        ));
    }

    lines.join("\n")
}

pub fn render_orders<'a>(orders: impl IntoIterator<Item = &'a Order>) -> String {
    let mut blocks = Vec::new();
    for order in orders {
        blocks.push(render_order(order));
    }
    blocks.join("\n\n")
}

fn status_to_str(status: OrderStatus) -> &'static str {
    match status {
        OrderStatus::Draft => "draft",
        OrderStatus::Confirmed => "confirmed",
        OrderStatus::Cancelled => "cancelled",
    }
}

use crate::app::error::AppError;
use crate::app::output::AppOutput;
use crate::app::repo::StoreRepo;
use crate::domain::order::{Order, OrderStatus};
use crate::domain::store::Store;
use crate::domain::types::{OrderId, Qty, Sku};
use crate::persist::format::{format_order, format_orders};

pub enum AppCommand {
    AddOrder { id: OrderId, customer: String },
    AddItem { id: OrderId, sku: Sku, qty: Qty },
    RemoveItem { id: OrderId, sku: Sku },
    Show { id: OrderId },
    List { customer: Option<String> },
    Total { id: OrderId },
}

pub struct OrderService<R: StoreRepo> {
    repo: R,
}

struct CommandResult {
    output: AppOutput,
    changed: bool,
}

impl CommandResult {
    fn changed(output: AppOutput) -> Self {
        Self {
            output,
            changed: true,
        }
    }

    fn unchanged(output: AppOutput) -> Self {
        Self {
            output,
            changed: false,
        }
    }
}

impl<R: StoreRepo> OrderService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub fn run(&self, cmd: AppCommand) -> Result<AppOutput, AppError> {
        let mut store = self.repo.load()?;
        let result = Self::apply_command(&mut store, cmd)?;
        if result.changed {
            self.repo.save(store)?;
        }
        Ok(result.output)
    }

    fn apply_command(store: &mut Store, cmd: AppCommand) -> Result<CommandResult, AppError> {
        match cmd {
            AppCommand::AddOrder { id, customer } => {
                let order = Order::new(id, customer, OrderStatus::Draft)?;
                store.add_order(order)?;
                Ok(CommandResult::changed(AppOutput::ok()))
            }
            AppCommand::AddItem { id, sku, qty } => {
                store.add_item(id, sku, qty)?;
                Ok(CommandResult::changed(AppOutput::ok()))
            }
            AppCommand::RemoveItem { id, sku } => {
                store.remove_item(id, &sku)?;
                Ok(CommandResult::changed(AppOutput::ok()))
            }
            AppCommand::Show { id } => {
                let order = store.show(id)?;
                Ok(CommandResult::unchanged(AppOutput::Text(format_order(
                    order,
                ))))
            }
            AppCommand::List { customer } => {
                let orders = select_orders(store, customer.as_deref());
                Ok(CommandResult::unchanged(AppOutput::Text(format_orders(
                    orders,
                ))))
            }
            AppCommand::Total { id } => {
                let total = store.total_qty(id)?;
                Ok(CommandResult::unchanged(AppOutput::Text(format_total(
                    total,
                ))))
            }
        }
    }
}

fn select_orders<'a>(store: &'a Store, customer: Option<&str>) -> Vec<&'a Order> {
    match customer {
        Some(customer) => store.list_by_customer(customer),
        None => store.list_all(),
    }
}

fn format_total(total: u32) -> String {
    format!("total_qty={}", total)
}

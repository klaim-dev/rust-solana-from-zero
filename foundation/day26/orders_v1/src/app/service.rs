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

impl<R: StoreRepo> OrderService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub fn run(&self, cmd: AppCommand) -> Result<AppOutput, AppError> {
        let mut store = self.load_or_new()?;
        let (out, changed) = Self::apply_command(&mut store, cmd)?;
        self.save_if_changed(store, changed)?;
        Ok(out)
    }

    fn load_or_new(&self) -> Result<Store, AppError> {
        self.repo.load()
    }

    fn save_if_changed(&self, store: Store, changed: bool) -> Result<(), AppError> {
        if changed {
            self.repo.save(store)?;
        }
        Ok(())
    }

    fn apply_command(store: &mut Store, cmd: AppCommand) -> Result<(AppOutput, bool), AppError> {
        match cmd {
            AppCommand::AddOrder { id, customer } => {
                let order = Order::new(id, customer, OrderStatus::Draft)?;
                store.add_order(order)?;
                Ok((AppOutput::ok(), true))
            }
            AppCommand::AddItem { id, sku, qty } => {
                store.add_item(id, sku, qty)?;
                Ok((AppOutput::ok(), true))
            }
            AppCommand::RemoveItem { id, sku } => {
                store.remove_item(id, &sku)?;
                Ok((AppOutput::ok(), true))
            }
            AppCommand::Show { id } => {
                let order = store.show(id)?;
                Ok((AppOutput::Text(format_order(order)), false))
            }
            AppCommand::List { customer } => {
                let orders = match customer {
                    Some(customer) => store.list_by_customer(&customer),
                    None => store.list_all(),
                };
                Ok((AppOutput::Text(format_orders(orders)), false))
            }
            AppCommand::Total { id } => {
                let total = store.total_qty(id)?;
                Ok((AppOutput::Text(format!("total_qty={}", total)), false))
            }
        }
    }
}

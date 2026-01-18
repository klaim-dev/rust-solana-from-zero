use crate::app::error::AppError;
use crate::app::output::{render_order, render_orders, AppOutput};
use crate::app::repo::StoreRepo;
use crate::domain::order::{Order, OrderStatus};
use crate::domain::store::Store;
use crate::domain::types::{OrderId, Qty, Sku};

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

    pub fn apply(store: &mut Store, cmd: AppCommand) -> Result<AppOutput, AppError> {
        match cmd {
            AppCommand::AddOrder { id, customer } => {
                let order = Order::new(id, customer, OrderStatus::Draft)?;
                store.add_order(order)?;
                Ok(AppOutput::Text("OK".into()))
            }
            AppCommand::AddItem { id, sku, qty } => {
                store.add_item(id, sku, qty)?;
                Ok(AppOutput::Text("OK".into()))
            }
            AppCommand::RemoveItem { id, sku } => {
                store.remove_item(id, &sku)?;
                Ok(AppOutput::Text("OK".into()))
            }
            AppCommand::Show { id } => {
                let order = store.show(id)?;
                Ok(AppOutput::Text(render_order(order)))
            }
            AppCommand::List { customer } => {
                let orders = match customer {
                    Some(customer) => store.list_by_customer(&customer),
                    None => store.list_all(),
                };
                Ok(AppOutput::Text(render_orders(orders)))
            }
            AppCommand::Total { id } => {
                let total = store.total_item(id)?;
                Ok(AppOutput::Text(format!("total_qty={}", total)))
            }
        }
    }

    pub fn run(&self, cmd: AppCommand) -> Result<AppOutput, AppError> {
        let mut store = self.repo.load()?;
        let out = Self::apply(&mut store, cmd)?;
        self.repo.save(store)?;
        Ok(out)
    }
}

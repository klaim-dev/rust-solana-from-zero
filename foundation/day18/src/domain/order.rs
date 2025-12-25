use crate::domain::{error::OrderError, line_item::LineItem, state::OrderState};
pub struct OrderId(u64);
impl OrderId{
    pub fn new(id: u64) -> Result<Self, OrderError>{
        if id == 0 {
            return Err(OrderError::InvariantViolation { msg: "order must be > 0" });
        }
        Ok(Self(id))
    }
    pub fn as_u64(&self) -> u64{
        self.0
    }
}

pub struct Order{
    id:OrderId,
    items:Vec<LineItem>,
    state: OrderState,
}
impl Order {
    pub fn new(id: OrderId) -> Self{
        Self { id, items: Vec::new(), state: OrderState::Draft }
    }
    pub fn ensure_editable(&self) -> Result<(), OrderError> {
        if self.state != OrderState::Draft{
            return Err(OrderError::OrderNotEditable { state: self.state.to_string() });
        }
        Ok(())
    }
    
    pub fn add_item(&mut self, item: LineItem) -> Result<(), OrderError> {
        self.ensure_editable()?;
        if self.items.iter().any(|x| x.sku() == item.sku()) {
           return  Err(OrderError::DuplicateItem { sku: item.sku().to_string() });
        }

        self.items.push(item);
        Ok(())
    }

    pub fn remove_item_by_sku(&mut self, sku: &str) -> Result<(), OrderError> {
        self.ensure_editable()?;
        let idx = self.items
        .iter()
        .position(|l| l.sku() == sku)
        .ok_or(OrderError::ItemNotFound { sku: sku.to_string()})?;
    self.items.remove(idx);
    Ok(())
    }

    pub fn set_qty(&mut self, sku: &str, qty: u32) -> Result<(), OrderError> {
        self.ensure_editable()?;
        let norm_sku = sku.trim().to_ascii_lowercase();
        let item = self.items
            .iter_mut()
            .find(|l| l.sku() == norm_sku)
            .ok_or(OrderError::ItemNotFound { sku: norm_sku.clone() })?;
        item.set_qty(qty)
    }

    pub fn submit(&mut self, now: u64) -> Result<(), OrderError> {
        if self.items.is_empty() {
            return Err(OrderError::EmptyOrder);
        }
        self.state.submit(now)
    }

    pub fn pay(&mut self, now: u64, tx_id: String) -> Result<(), OrderError> {
        self.state.pay(now, tx_id)
    }

    pub fn cancel(&mut self, now: u64, reason: String) -> Result<(), OrderError> {
        self.state.cancel(now, reason)
    }

    pub fn total_cents(&self) -> Result<u64, OrderError> {
        self.items
            .iter()
            .try_fold(0u64, |acc, item| {
                let line_total = item.line_total_cents()?;
                acc.checked_add(line_total)
                    .ok_or(OrderError::InvariantViolation { msg: "order total overflow" })
            })
    }

}
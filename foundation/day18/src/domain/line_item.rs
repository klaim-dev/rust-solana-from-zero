use crate::domain::error::OrderError;
#[derive(Debug,PartialEq, Eq)]
pub struct LineItem{
    sku: String,
    qty: u32,
    price_cents: u64,
}

impl LineItem{
    pub fn new(sku:String, qty:u32, price_cents:u64)-> Result<Self, OrderError>{
        let norm_sku = sku.trim().to_ascii_lowercase();
        if norm_sku.is_empty(){
            return Err(OrderError::EmptySku);
        }

        if qty == 0 {
            return Err(OrderError::ZeroQuantity);
        }
        Ok(Self { sku: norm_sku, qty, price_cents})
    }

    pub fn sku(&self) -> &str {
        self.sku.as_str()
    }

    pub fn line_total_cents(&self) -> Result<u64, OrderError> {
        Ok((self.qty as u64).checked_mul(self.price_cents).ok_or(OrderError::InvariantViolation { msg: "line total overflow" })?)
    }

    pub fn set_qty(&mut self, qty: u32) -> Result<(), OrderError> {
        if qty == 0 {
            return Err(OrderError::ZeroQuantity);
        }
        self.qty = qty;
        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ItemErr {
    #[error("invalid item name")]
    InvalidName,
}

#[derive(Debug, thiserror::Error)]
pub enum SkuErr {
    #[error("invalid sku")]
    InvalidSku,
}

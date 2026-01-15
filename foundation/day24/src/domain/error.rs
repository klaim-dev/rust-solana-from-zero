#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    #[error("empty name")]
    EmptyName,
    #[error("empty sku")]
    EmptySku,
    #[error("price must be bigger then 0, input: {input}")]
    ZeroPrice { input: u64 },
}

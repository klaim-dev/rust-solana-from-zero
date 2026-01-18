use crate::domain::error::DomainError;
use std::fmt;
#[derive(Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd, Debug)]
pub struct OrderId(u64);
impl OrderId {
    pub fn new(input: u64) -> Result<Self, DomainError> {
        if input == 0 {
            return Err(DomainError::ZeroOrderId);
        }
        Ok(Self(input))
    }

    pub fn get(&self) -> u64 {
        self.0
    }
}
impl fmt::Display for OrderId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Eq, PartialEq, Hash, Ord, PartialOrd, Debug)]
pub struct Sku(String);
impl Sku {
    pub fn new(input: String) -> Result<Self, DomainError> {
        let normalized = input.trim().to_ascii_lowercase();
        if normalized.is_empty() {
            return Err(DomainError::EmptySku);
        }
        if normalized.chars().any(|ch| ch.is_whitespace()) {
            return Err(DomainError::SkuWhitespace);
        }
        if normalized.contains('"') {
            return Err(DomainError::SkuQuote);
        }
        Ok(Self(normalized))
    }

    pub fn get(&self) -> &str {
        &self.0
    }
}
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct Qty(u32);
impl Qty {
    pub fn new(input: u32) -> Result<Self, DomainError> {
        if input == 0 {
            return Err(DomainError::ZeroQty);
        }
        Ok(Self(input))
    }

    pub fn get(&self) -> u32 {
        self.0
    }
}
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct MoneyCents(u64);
impl MoneyCents {
    pub fn new(input: u64) -> Result<Self, DomainError> {
        if input == 0 {
            return Err(DomainError::ZeroMoneyCents);
        }
        Ok(Self(input))
    }

    pub fn get(&self) -> u64 {
        self.0
    }
}

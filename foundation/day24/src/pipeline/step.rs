use crate::domain::{
    error::DomainError,
    record::{Name, Record, Sku},
};

pub trait Step {
    fn apply(&self, rec: Record) -> Result<Record, DomainError>;
}

pub struct TrimName;
impl Step for TrimName {
    fn apply(&self, rec: Record) -> Result<Record, DomainError> {
        let (sku, old_name, price_cents) = rec.into_parts();
        let trimmed = old_name.get().trim();
        let name = Name::new(trimmed.to_owned())?;
        Ok(Record::from_parts(sku, name, price_cents))
    }
}

pub struct LowerSku;
impl Step for LowerSku {
    fn apply(&self, rec: Record) -> Result<Record, DomainError> {
        let (old_sku, name, price_cents) = rec.into_parts();
        let lower = old_sku.get().trim().to_lowercase();
        let sku = Sku::new(lower)?;
        Ok(Record::from_parts(sku, name, price_cents))
    }
}

pub struct NormalizeSpaceName;
impl Step for NormalizeSpaceName {
    fn apply(&self, rec: Record) -> Result<Record, DomainError> {
        let (sku, old_name, price_cents) = rec.into_parts();
        let normalized = old_name
            .get()
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");
        let name = Name::new(normalized)?;
        Ok(Record::from_parts(sku, name, price_cents))
    }
}

pub struct SpaceToUnderscoreName;
impl Step for SpaceToUnderscoreName {
    fn apply(&self, rec: Record) -> Result<Record, DomainError> {
        let (sku, old_name, price_cents) = rec.into_parts();
        let replaced = old_name.get().replace(' ', "_");
        let name = Name::new(replaced)?;
        Ok(Record::from_parts(sku, name, price_cents))
    }
}

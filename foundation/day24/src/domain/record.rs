use crate::domain::error::DomainError;
pub struct Record {
    sku: Sku,
    name: Name,
    price_cents: PriceCents,
}
impl Record {
    pub fn new(sku: Sku, name: Name, price_cents: PriceCents) -> Record {
        Self {
            sku,
            name,
            price_cents,
        }
    }

    pub fn into_parts(self) -> (Sku, Name, PriceCents) {
        (self.sku, self.name, self.price_cents)
    }
    pub fn from_parts(sku: Sku, name: Name, price_cents: PriceCents) -> Self {
        Self {
            sku,
            name,
            price_cents,
        }
    }

    pub fn with_name(self, name: Name) -> Record {
        let (sku, _old_name, price_cents) = self.into_parts();
        Self::from_parts(sku, name, price_cents)
    }
    pub fn with_price_cents(self, price_cents: PriceCents) -> Record {
        let (sku, name, _old_price_cents) = self.into_parts();
        Self::from_parts(sku, name, price_cents)
    }
    pub fn with_sku(self, sku: Sku) -> Record {
        let (_old_sku, name, price_cents) = self.into_parts();
        Self::from_parts(sku, name, price_cents)
    }
}

pub struct Sku(String);
impl Sku {
    pub fn new(input: String) -> Result<Self, DomainError> {
        if input.trim().is_empty() {
            return Err(DomainError::EmptySku);
        }
        Ok(Self(input))
    }

    pub fn get(&self) -> &str {
        &self.0
    }
}

pub struct Name(String);
impl Name {
    pub fn new(input: String) -> Result<Self, DomainError> {
        if input.trim().is_empty() {
            return Err(DomainError::EmptyName);
        }
        Ok(Self(input))
    }
    pub fn get(&self) -> &str {
        &self.0
    }
}

pub struct PriceCents(u64);
impl PriceCents {
    pub fn new(input: u64) -> Result<Self, DomainError> {
        if input == 0 {
            return Err(DomainError::ZeroPrice { input });
        }
        Ok(Self(input))
    }

    pub fn get(&self) -> u64 {
        self.0
    }
}

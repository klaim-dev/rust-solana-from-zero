use thiserror::Error;

#[derive(Debug, PartialEq, Eq)]
pub struct User {
    id: u64,
    email: String,
    age: u8,
    is_active: bool,
}

#[derive(Debug, Error, PartialEq)]
pub enum UserError {
    #[error("invalid id {0}")]
    InvalidId(u64),
    #[error("invalid email {0}")]
    InvalidEmail(String),
    #[error("invalid age {0}")]
    InvalidAge(u8),
}

impl User {
    pub fn new(id: u64, email: String, age: u8) -> Result<Self, UserError> {
        if id == 0 {
            return Err(UserError::InvalidId(id));
        }

        let trimmed_email = email.trim();

        if !trimmed_email.contains('@') || trimmed_email.contains(' ') || trimmed_email.is_empty() {
            return Err(UserError::InvalidEmail(trimmed_email.to_string()));
        }

        if age > 120 {
            return Err(UserError::InvalidAge(age));
        }

        Ok(Self {
            id,
            email: trimmed_email.to_string(),
            age,
            is_active: true,
        })
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn email(&self) -> &str {
        &self.email
    }

    pub fn age(&self) -> u8 {
        self.age
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }
}

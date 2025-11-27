use thiserror::Error;

#[derive(Debug, Clone, PartialEq)]
pub struct User {
    id: u64,
    email: String,
    age: u8,
    is_active: bool,
}

#[derive(Error, Debug, PartialEq)]
pub enum UserError {
    #[error("id must be positive, got {0}")]
    InvalidId(u64),

    #[error("invalid email format: {0}")]
    InvalidEmail(String),

    #[error("age must be in 0..=120, got {0}")]
    InvalidAge(u8),
}

impl User {
    pub fn new(id: u64, email: String, age: u8) -> Result<Self, UserError> {
        if id == 0 {
            return Err(UserError::InvalidId(id));
        }

        let trimmed_email = email.trim();
        if trimmed_email.is_empty() || !trimmed_email.contains('@') || trimmed_email.contains(' ') {
            return Err(UserError::InvalidEmail(email.to_string()));
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
        self.email.as_str()
    }

    pub fn age(&self) -> u8 {
        self.age
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }

    pub fn is_adult(&self) -> bool {
        self.age >= 18
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
    }

    pub fn activate(&mut self) {
        self.is_active = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_happy_path() {
        let user = User::new(1, "alice@example.com".to_string(), 30).unwrap();
        assert_eq!(user.id(), 1);
        assert_eq!(user.email(), "alice@example.com");
        assert_eq!(user.age(), 30);
        assert!(user.is_active());
    }

    #[test]
    fn new_invalid_id() {
        let err = User::new(0, "alice@example.com".to_string(), 30).unwrap_err();
        assert_eq!(err, UserError::InvalidId(0));
    }

    #[test]
    fn new_invalid_email() {
        let err = User::new(1, "invalid".to_string(), 30).unwrap_err();
        assert_eq!(err, UserError::InvalidEmail("invalid".to_string()));
    }

    #[test]
    fn new_invalid_age() {
        let err = User::new(1, "alice@example.com".to_string(), 121).unwrap_err();
        assert_eq!(err, UserError::InvalidAge(121));
    }
}

use super::user::{User, UserError};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum RegistrationError {
    #[error("user with id {0} already exists")]
    DuplicateId(u64),

    #[error("user with email {0} already exists")]
    DuplicateEmail(String),

    #[error(transparent)]
    User(#[from] UserError),
}

#[derive(Debug, Default)]
pub struct UserRegistry {
    users: Vec<User>,
}

impl UserRegistry {
    pub fn new() -> Self {
        Self { users: Vec::new() }
    }

    pub fn all(&self) -> &[User] {
        &self.users
    }

    pub fn find_by_id(&self, id: u64) -> Option<&User> {
        self.users.iter().find(|u| u.id() == id)
    }

    pub fn find_by_email(&self, email: &str) -> Option<&User> {
        self.users.iter().find(|u| u.email() == email.trim())
    }

    pub fn register(
        &mut self,
        id: u64,
        email: String,
        age: u8,
    ) -> Result<&User, RegistrationError> {
        if self.find_by_id(id).is_some() {
            return Err(RegistrationError::DuplicateId(id));
        }

        if self.find_by_email(&email).is_some() {
            return Err(RegistrationError::DuplicateEmail(email.to_string()));
        }

        let user = User::new(id, email, age)?;
        self.users.push(user);

        let idx = self.users.len() - 1;
        Ok(&self.users[idx])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn register_happy_path() {
        let mut reg = UserRegistry::new();
        let user = reg
            .register(1, "alice@example.com".to_string(), 30)
            .unwrap();
        assert_eq!(user.id(), 1);
        assert_eq!(reg.all().len(), 1);
    }

    #[test]
    fn register_duplicate_id() {
        let mut reg = UserRegistry::new();
        reg.register(1, "alice@example.com".to_string(), 30)
            .unwrap();
        let err = reg
            .register(1, "bob@example.com".to_string(), 25)
            .unwrap_err();
        assert_eq!(err, RegistrationError::DuplicateId(1));
    }

    #[test]
    fn register_duplicate_email() {
        let mut reg = UserRegistry::new();
        reg.register(1, "alice@example.com".to_string(), 30)
            .unwrap();
        let err = reg
            .register(2, "alice@example.com".to_string(), 25)
            .unwrap_err();
        assert_eq!(
            err,
            RegistrationError::DuplicateEmail("alice@example.com".to_string())
        );
    }
}

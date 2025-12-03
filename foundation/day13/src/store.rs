use std::collections::HashMap;

use thiserror::Error;

use crate::{User, UserError};

#[derive(Debug, Error, PartialEq)]
pub enum StoreError {
    #[error("id already exists {0}")]
    DuplicateId(u64),
    #[error("email already exists {0}")]
    DuplicateEmail(String),
    #[error(transparent)]
    User(#[from] UserError),
}

#[derive(Debug, PartialEq)]
pub struct UserStore {
    users_by_id: HashMap<u64, User>,
    ids_by_email: HashMap<String, u64>,
}

impl UserStore {
    pub fn new() -> Self {
        Self {
            users_by_id: HashMap::new(),
            ids_by_email: HashMap::new(),
        }
    }

    pub fn get_by_id(&self, id: u64) -> Option<&User> {
        self.users_by_id.get(&id)
    }

    pub fn get_by_email(&self, email: &str) -> Option<&User> {
        let normalized_email = email.trim();
        self.ids_by_email
            .get(normalized_email)
            .and_then(|id| self.get_by_id(*id))
    }

    pub fn remove_by_id(&mut self, id: u64) -> Option<User> {
        if let Some(user) = self.users_by_id.remove(&id) {
            self.ids_by_email.remove(user.email());
            Some(user)
        } else {
            None
        }
    }

    pub fn register(&mut self, id: u64, email: &str, age: u8) -> Result<&User, StoreError> {
        let normalized_email = email.trim().to_string();

        if self.users_by_id.contains_key(&id) {
            return Err(StoreError::DuplicateId(id));
        }

        if self.ids_by_email.contains_key(&normalized_email) {
            return Err(StoreError::DuplicateEmail(normalized_email));
        }

        let user = User::new(id, normalized_email.clone(), age)?;
        let user_ref = self.users_by_id.entry(id).or_insert(user);

        self.ids_by_email
            .insert(normalized_email, user_ref.id());

        Ok(user_ref)
    }
}

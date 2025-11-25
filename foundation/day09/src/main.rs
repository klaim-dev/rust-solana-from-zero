fn main() {
    println!("Hello, world!");
}
pub struct User {
    id: u64,
    name: String,
    email: Option<String>,
    age: u8,
    is_active: bool,
}

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum UserError {
    #[error("id must be positive, got {0}")]
    InvalidId(u64),

    #[error("name must not be empty")]
    EmptyName,

    #[error("age must be in 0..=120, got {0}")]
    InvalidAge(u8),

    #[error("invalid email format: {0}")]
    InvalidEmail(String),
}

impl User {
    pub fn new(id: u64, name: String, email: Option<String>, age: u8) -> Result<User, UserError> {
        if id == 0 {
            return Err(UserError::InvalidId(id));
        }

        let trimmed_name = name.trim();

        if trimmed_name.is_empty() {
            return Err(UserError::EmptyName);
        }

        let name = trimmed_name.to_string();

        if age > 120 {
            return Err(UserError::InvalidAge(age));
        }

        if let Some(s) = &email {
            if !s.contains('@') || s.contains(' ') {
                return Err(UserError::InvalidEmail(s.clone()));
            }
        }

        Ok(Self {
            id,
            name,
            email,
            age,
            is_active: true,
        })
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn email(&self) -> Option<&str> {
        self.email.as_deref()
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
    pub fn rename(&mut self, new_name: &str) -> Result<(), UserError> {
        let trimmed = new_name.trim();
        if trimmed.is_empty() {
            return Err(UserError::EmptyName);
        }

        self.name = trimmed.to_string();
        Ok(())
    }
    pub fn set_email(&mut self, new_email: Option<String>) -> Result<(), UserError> {
        match new_email {
            None => {
                self.email = None;
                Ok(())
            }
            Some(s) => {
                if !s.contains('@') || s.contains(' ') {
                    return Err(UserError::InvalidEmail(s));
                }

                self.email = Some(s);
                Ok(())
            }
        }
    }
    pub fn deactivate(&mut self) {
        self.is_active = false
    }

    pub fn activate(&mut self) {
        self.is_active = true;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn is_adult_works() {
        let u1 = User::new(1, "neo".into(), None, 17).unwrap();
        let u2 = User::new(2, "morpheus".into(), None, 18).unwrap();

        assert!(!u1.is_adult());
        assert!(u2.is_adult());
    }

    #[test]
    fn rename_trims_and_changes_name() {
        let mut user = User::new(1, "neo".into(), None, 17).unwrap();
        user.rename("  new  ").unwrap();
        assert_eq!(user.name(), "new");
    }
    #[test]
    fn rename_rejects_empty() {
        let mut user = User::new(1, "neo".into(), None, 17).unwrap();
        let err = user.rename("   ").unwrap_err();
        assert_eq!(err, UserError::EmptyName);
        assert_eq!(user.name(), "neo");
    }

    #[test]
    fn rename_does_not_trim_inside_spaces() {
        let mut user = User::new(1, "neo".into(), None, 17).unwrap();
        user.rename(" my new name  ").unwrap();
        assert_eq!(user.name(), "my new name");
    }
    #[test]
    fn set_email_updates_value() {
        let mut u = User::new(1, "neo".into(), None, 20).unwrap();
        u.set_email(Some("neo@zion.com".into())).unwrap();
        assert_eq!(u.email(), Some("neo@zion.com"));
    }

    #[test]
    fn set_email_clears_when_none() {
        let mut u = User::new(1, "neo".into(), Some("old@a.com".into()), 20).unwrap();
        u.set_email(None).unwrap();
        assert_eq!(u.email(), None);
    }

    #[test]
    fn set_email_rejects_missing_at() {
        let mut u = User::new(1, "neo".into(), None, 20).unwrap();
        let err = u.set_email(Some("invalid.com".into())).unwrap_err();
        assert_eq!(err, UserError::InvalidEmail("invalid.com".into()));
    }

    #[test]
    fn set_email_rejects_space() {
        let mut u = User::new(1, "neo".into(), None, 20).unwrap();
        let err = u.set_email(Some("bad email@x.com".into())).unwrap_err();
        assert_eq!(err, UserError::InvalidEmail("bad email@x.com".into()));
    }

    #[test]
    fn deactivate_sets_flag_false() {
        let mut u = User::new(1, "neo".into(), None, 20).unwrap();
        assert!(u.is_active());
        u.deactivate();
        assert!(!u.is_active());
    }

    #[test]
    fn activate_sets_flag_true() {
        let mut u = User::new(1, "neo".into(), None, 20).unwrap();
        u.deactivate();
        assert!(!u.is_active());
        u.activate();
        assert!(u.is_active());
    }
}

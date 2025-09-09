use std::fmt;
use bs58;
use rand::Rng;
use crate::errors::user_errors::UserError;
use crate::all_models::normalize::name_ascii;


#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub struct Pubkey(pub [u8;32]);

impl fmt::Display for Pubkey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let base58_str = bs58::encode(self.0).into_string();
        write!(f, "{}", base58_str)
    }
}

impl Pubkey {
    pub fn new_random() -> Self {
        let mut bytes = [0u8;32];
        rand::thread_rng().fill(&mut bytes);
        Pubkey(bytes)

    }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Default)]
pub struct Lamports(pub u64);

impl fmt::Display for Lamports {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} lamports", self.0)
    }
}
#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct TokenAccount {
    pub owner: Pubkey,
    pub mint: Pubkey,
    pub amount: Lamports,
}

impl TokenAccount {
    pub fn new_dummy(byte: u8) -> Self {
        TokenAccount {
            owner: Pubkey([byte; 32]),
            mint: Pubkey([byte; 32]),
            amount: Lamports(100),
        }
    }

    pub fn new_with(owner_byte: u8, amount: u64, mint: Pubkey) -> Self {
        TokenAccount {
            owner: Pubkey([owner_byte; 32]),
            mint,
            amount: Lamports(amount),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Transfer {
    pub from: Pubkey,
    pub to: Pubkey,
    pub amount: Lamports,
}

#[derive(Debug)]
pub struct User {
    name: String,
    age: u32,
    email: String,
}

impl User {
    pub const MAX_NAME: usize = 64;
    pub const MAX_AGE: u32 = 150;

    pub fn try_new( name: &str, age: u32, email: &str) -> Result<Self, UserError> {
        let name = name_ascii(name)?;

        if name.len() > Self::MAX_NAME {
            return Err(UserError::NameTooLong { max: Self::MAX_NAME });
        };

        if age > Self::MAX_AGE {
            return Err(UserError::AgeOverflow);
        };

        if !(email.contains('@') && email.contains('.')) {
            return Err(UserError::InvalidEmail);
        }

        let user = User{
            name: name.into(),
            age,
            email: email.into(),
        };

        Ok(user)
    }
    pub fn greet(&self) -> String {
        format!("Hi {}, age: {}, email: {}", self.name, self.age, self.email)
    } 
    pub fn name_len(&self) -> usize {
        self.name.len()
    }
    pub fn rename(&mut self, new_name: &str) -> Result<(), UserError> {
        if new_name.len() > Self::MAX_NAME {
            return Err(UserError::NameTooLong { max: Self::MAX_NAME });
        } else {
            self.name = new_name.into();
            Ok(())
        }
    }
    pub fn have_birthday(&mut self) -> Result<(), UserError> {
         if self.age  + 1  > Self::MAX_AGE  as u32 {
            return Err(UserError::AgeOverflow);
        } else {
            self.age += 1;
            Ok(())
        }

    }

    pub fn email_domain(&self) -> Option<&str> {
        self.email.split_once('@').map(|(_, domain)| domain)
    }

#[cfg(test)]
pub fn new_unchecked(name: String, age: u32, email: String) -> Self {
    Self { name, age, email }
}
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::all_models::models::{TokenAccount, Lamports, Pubkey};


#[test]
fn happy_path_works() {
    let user = User::try_new(" alice ", 30, "a@b.com").unwrap();
    assert_eq!(user.name, "Alice");
    assert_eq!(user.age, 30);
    assert_eq!(user.email, "a@b.com");
    assert_eq!(User::email_domain(&user), Some("b.com"));
}

#[test]
fn empty_name_fails() {
    let res = User::try_new("    ", 25, "a@b.com");
    assert!(matches!(res, Err(UserError::EmptyName)));
}

#[test]
fn name_too_long_fails() {
    let name = "a".repeat(User::MAX_NAME + 1);
    let res = User::try_new(&name, 25, "a@b.com");
    assert!(matches!(res, Err(UserError::NameTooLong { .. })));
}

#[test]
fn age_out_of_range_fails() {
    let res = User::try_new("Alice", 200, "a@b.com");
    assert!(matches!(res, Err(UserError::AgeOverflow))); 
}

#[test]
fn invalid_email_fails() {
    let res = User::try_new("Alice", 25, "not-an-email");
    assert!(matches!(res, Err(UserError::InvalidEmail)));
}

#[test]
fn birthday_fails_at_max_age() {
    let mut user = User::new_unchecked("Alice".into(), User::MAX_AGE as u32, "a@b.com".into());
    let res = user.have_birthday();
    assert!(matches!(res, Err(UserError::AgeOverflow)));
}

#[test]
fn rename_fails_if_name_too_long() {
    let mut user = User::new_unchecked("Alice".into(), 25, "a@b.com".into());
    let name = "a".repeat(User::MAX_NAME + 1);
    let res = user.rename(&name);
    assert!(matches!(res, Err(UserError::NameTooLong { .. })));
}
}


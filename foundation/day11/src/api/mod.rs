use crate::domain::registry::{RegistrationError, UserRegistry};
use crate::domain::user::User;

pub fn register_user<'a>(
    registry: &'a mut UserRegistry,
    id: u64,
    email: &str,
    age: u8,
) -> Result<&'a User, RegistrationError> {
    registry.register(id, email.to_string(), age)
}

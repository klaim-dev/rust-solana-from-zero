pub mod api;
pub mod domain;

pub use domain::user::{User, UserError};
pub use domain::registry::{UserRegistry, RegistrationError};
pub use api::register_user;
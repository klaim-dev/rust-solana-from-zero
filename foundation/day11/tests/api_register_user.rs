use day11::{UserRegistry, register_user, RegistrationError};

#[test]
fn register_user_happy_path() {
    let mut reg = UserRegistry::new();
    let user = register_user(&mut reg, 1, "alice@example.com", 30).unwrap();
    assert_eq!(user.id(), 1);
}

#[test]
fn register_user_duplicate_email() {
    let mut reg = UserRegistry::new();
    register_user(&mut reg, 1, "alice@example.com", 30).unwrap();
    let err = register_user(&mut reg, 2, "alice@example.com", 25).unwrap_err();
    assert!(matches!(err, RegistrationError::DuplicateEmail(_)));
}

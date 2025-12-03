use day13::{StoreError, User, UserError, UserStore};

#[test]
fn user_new_success() {
    let user = User::new(1, " user@example.com ".to_string(), 30).unwrap();

    assert_eq!(user.id(), 1);
    assert_eq!(user.email(), "user@example.com");
    assert_eq!(user.age(), 30);
    assert!(user.is_active());
}

#[test]
fn user_new_invalid_id() {
    let err = User::new(0, "test@example.com".to_string(), 25).unwrap_err();
    assert_eq!(err, UserError::InvalidId(0));
}

#[test]
fn user_new_invalid_email() {
    let err = User::new(1, "invalid".to_string(), 25).unwrap_err();
    assert_eq!(err, UserError::InvalidEmail("invalid".to_string()));
}

#[test]
fn user_new_invalid_age() {
    let err = User::new(1, "test@example.com".to_string(), 200).unwrap_err();
    assert_eq!(err, UserError::InvalidAge(200));
}

#[test]
fn store_register_and_getters() {
    let mut store = UserStore::new();
    store
        .register(1, " first@example.com ", 28)
        .expect("failed to register");

    let user = store.get_by_id(1).expect("user should be stored");
    assert_eq!(user.email(), "first@example.com");
    assert_eq!(store.get_by_id(1), Some(user));
    assert_eq!(store.get_by_email("first@example.com"), Some(user));
    assert_eq!(store.get_by_email(" first@example.com "), Some(user));
}

#[test]
fn store_register_duplicate_id() {
    let mut store = UserStore::new();
    store.register(1, "a@example.com", 25).unwrap();

    let err = store.register(1, "b@example.com", 30).unwrap_err();
    assert_eq!(err, StoreError::DuplicateId(1));
}

#[test]
fn store_register_duplicate_email() {
    let mut store = UserStore::new();
    store.register(1, "a@example.com", 25).unwrap();

    let err = store.register(2, " a@example.com ", 30).unwrap_err();
    assert_eq!(err, StoreError::DuplicateEmail("a@example.com".to_string()));
}

#[test]
fn store_register_invalid_user_error_propagates() {
    let mut store = UserStore::new();
    let err = store.register(1, "bad-age@example.com", 130).unwrap_err();
    assert!(matches!(err, StoreError::User(UserError::InvalidAge(130))));
}

#[test]
fn store_remove_by_id() {
    let mut store = UserStore::new();
    store.register(1, "remove@example.com", 20).unwrap();

    let removed = store.remove_by_id(1).expect("user should be removed");
    assert_eq!(removed.email(), "remove@example.com");
    assert!(store.get_by_id(1).is_none());
    assert!(store.get_by_email("remove@example.com").is_none());
}

#[test]
fn store_remove_missing_user_returns_none() {
    let mut store = UserStore::new();
    assert!(store.remove_by_id(99).is_none());
}

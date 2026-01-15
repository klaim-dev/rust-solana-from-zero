use day24::domain::error::DomainError;
use day24::domain::record::{Name, Sku};

#[test]
fn name_rejects_whitespace_only() {
    let err = match Name::new("   ".to_string()) {
        Ok(_) => panic!("expected empty name error"),
        Err(err) => err,
    };
    assert!(matches!(err, DomainError::EmptyName));
}

#[test]
fn sku_rejects_whitespace_only() {
    let err = match Sku::new("   ".to_string()) {
        Ok(_) => panic!("expected empty sku error"),
        Err(err) => err,
    };
    assert!(matches!(err, DomainError::EmptySku));
}

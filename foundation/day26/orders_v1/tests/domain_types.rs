use day25::domain::error::DomainError;
use day25::domain::types::Sku;

#[test]
fn sku_rejects_whitespace() {
    let err = Sku::new("a b".to_string()).expect_err("whitespace");
    assert!(matches!(err, DomainError::SkuWhitespace));
}

#[test]
fn sku_rejects_quote() {
    let err = Sku::new("a\"b".to_string()).expect_err("quote");
    assert!(matches!(err, DomainError::SkuQuote));
}

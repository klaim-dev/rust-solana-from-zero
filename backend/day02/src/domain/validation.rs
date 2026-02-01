use crate::domain::error::DomainError;

pub fn validate_user(id: u64) -> Result<(), DomainError> {
    if id == 0 {
        return Err(DomainError::NotFound {
            entity: "user",
            message: "user not found".into(),
        });
    }

    Ok(())
}

pub fn validate_limit(limit: u32) -> Result<(), DomainError> {
    if !(1..=100).contains(&limit) {
        return Err(DomainError::Validation {
            field: "q",
            message: "limit must be 1..=100".into(),
        });
    }

    Ok(())
}

pub fn validate_query(q: &str) -> Result<(), DomainError> {
    if q.is_empty() {
        return Err(DomainError::Validation {
            field: "q",
            message: "q must not be empty".into(),
        });
    }

    Ok(())
}

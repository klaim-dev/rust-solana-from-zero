#[allow(dead_code)]
pub enum DomainError {
    NotFound {
        entity: &'static str,
        message: String,
    },
    Validation {
        field: &'static str,
        message: String,
    },
    Conflict {
        field: &'static str,
        message: String,
    },
}

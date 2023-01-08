use crate::domain::model::error::DomainError;

pub type Result<T> = std::result::Result<T, DomainError>;
pub type ValidationResult = std::result::Result<(), DomainError>;

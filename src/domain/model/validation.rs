
use crate::domain::model::result::ValidationResult;

pub trait Validator {
    fn validate(&self) -> ValidationResult;
}

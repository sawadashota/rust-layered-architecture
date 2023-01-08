use crate::domain::model::user::UserID;

pub enum DomainError {
    UserNotFound(UserID),

    Conflict,

    InvalidPagingToken,

    ValidationError(String, String),
}

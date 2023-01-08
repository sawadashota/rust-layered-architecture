use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("not found an user [{0}]")]
    NotFound(String),

    #[error("request is conflict")]
    Conflict,

    #[error("invalid page token")]
    InvalidPageToken,

    #[error("validation error: {1} at {0}")]
    ValidationError(String, String),

    #[error("unexpected error")]
    InternalServerError,
}

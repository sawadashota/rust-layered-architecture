use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use crate::usecase::error::Error;

#[derive(Serialize)]
pub struct ErrorResponse {
    message: String,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let res = Json(ErrorResponse {
            message: self.to_string()
        });
        match self {
            Error::NotFound(_) => (StatusCode::NOT_FOUND, res),
            Error::Conflict => (StatusCode::CONFLICT, res),
            Error::ValidationError(_, _) => (StatusCode::BAD_REQUEST, res),
            Error::InvalidPageToken => (StatusCode::BAD_REQUEST, res),
            Error::InternalServerError => (StatusCode::INTERNAL_SERVER_ERROR, res),
        }.into_response()
    }
}

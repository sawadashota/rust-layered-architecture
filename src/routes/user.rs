use std::sync::Arc;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use paging::Paging;
use crate::domain::model::user::{User, UserID};
use crate::routes::pagination::Pagination;
use crate::usecase::error::Error;
use crate::usecase::user::{CreateUserRequest, HaveUserUseCase, UpdateUserRequest, UserUseCase};


pub async fn find_user(
    Path(id): Path<String>,
    State(r): State<Arc<impl HaveUserUseCase>>,
) -> Result<impl IntoResponse, Error> {
    let user = r.user_usecase().find(&UserID::new(id))?;
    Ok(Json(user))
}

#[derive(Serialize)]
pub struct UserList {
    users: Vec<User>,
    next_token: String,
}

pub async fn list_users(
    Query(pagination): Query<Pagination>,
    State(r): State<Arc<impl HaveUserUseCase>>,
) -> Result<impl IntoResponse, Error> {
    let paging = Paging::from(pagination);
    let (users, next_token) = r.user_usecase().list(paging)?;
    Ok(Json(UserList {
        users,
        next_token,
    }))
}

#[derive(Debug, Deserialize)]
pub struct CreateUser {
    name: String,
}

pub async fn create_user(
    State(r): State<Arc<impl HaveUserUseCase>>,
    Json(input): Json<CreateUser>,
) -> Result<impl IntoResponse, Error> {
    let user = r.user_usecase().create(CreateUserRequest::new(input.name))?;
    Ok(Json(user))
}

pub async fn update_user(
    Path(id): Path<String>,
    State(r): State<Arc<impl HaveUserUseCase>>,
    Json(input): Json<CreateUser>,
) -> Result<impl IntoResponse, Error> {
    r.user_usecase().update(&UserID::new(id), UpdateUserRequest::new(input.name))?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn delete_user(
    Path(id): Path<String>,
    State(r): State<Arc<impl HaveUserUseCase>>,
) -> Result<impl IntoResponse, Error> {
    r.user_usecase().delete(&UserID::new(id))?;
    Ok(StatusCode::NO_CONTENT)
}

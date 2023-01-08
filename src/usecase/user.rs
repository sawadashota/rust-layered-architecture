use derive_new::new;
use paging::Paging;
use crate::domain::model::error::DomainError;
use crate::domain::model::user::{User, UserID, UserName};
use crate::domain::service::user::{HaveUserService, UserService};
use crate::usecase::error::{Error, Result};

#[derive(new)]
pub struct CreateUserRequest {
    pub name: String,
}

#[derive(new)]
pub struct UpdateUserRequest {
    pub name: String,
}

pub trait UserUseCase: HaveUserService {
    fn find(&self, id: &UserID) -> Result<User> {
        match self.user_service().find(id) {
            Ok(user) => Ok(user),
            Err(DomainError::ValidationError(model, message)) => Err(Error::ValidationError(model, message)),
            Err(DomainError::UserNotFound(e)) => Err(Error::NotFound(e.to_string())),
            _ => Err(Error::InternalServerError),
        }
    }

    fn list(&self, paging: Paging) -> Result<(Vec<User>, paging::Token)> {
        match self.user_service().list(paging) {
            Ok((users, next_token)) => Ok((users, next_token)),
            Err(DomainError::ValidationError(model, message)) => Err(Error::ValidationError(model, message)),
            Err(DomainError::InvalidPagingToken) => Err(Error::InvalidPageToken),
            _ => Err(Error::InternalServerError),
        }
    }

    fn create(&self, req: CreateUserRequest) -> Result<User> {
        let user = User::generate(UserName::new(req.name));
        match self.user_service().create(user.clone()) {
            Ok(_) => Ok(user),
            Err(DomainError::ValidationError(model, message)) => Err(Error::ValidationError(model, message)),
            Err(DomainError::Conflict) => Err(Error::Conflict),
            _ => Err(Error::InternalServerError),
        }
    }

    fn update(&self, id: &UserID, req: UpdateUserRequest) -> Result<()> {
        let user = User::new(id.clone(), UserName::new(req.name));
        match self.user_service().update(user) {
            Ok(_) => Ok(()),
            Err(DomainError::UserNotFound(e)) => Err(Error::NotFound(e.to_string())),
            Err(DomainError::ValidationError(model, message)) => Err(Error::ValidationError(model, message)),
            Err(DomainError::Conflict) => Err(Error::Conflict),
            _ => Err(Error::InternalServerError),
        }
    }

    fn delete(&self, id: &UserID) -> Result<()> {
        match self.user_service().delete(id) {
            Ok(_) => Ok(()),
            Err(DomainError::UserNotFound(e)) => Err(Error::NotFound(e.to_string())),
            Err(DomainError::ValidationError(model, message)) => Err(Error::ValidationError(model, message)),
            _ => Err(Error::InternalServerError),
        }
    }
}

impl <T: HaveUserService> UserUseCase for T {}

pub trait HaveUserUseCase {
    type UserUseCase: UserUseCase;
    fn user_usecase(&self) -> &Self::UserUseCase;
}

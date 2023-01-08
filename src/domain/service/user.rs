use paging::Paging;
use crate::domain::model::result::Result;
use crate::domain::model::validation::Validator;
use crate::domain::model::user::{UserID, User};
use crate::domain::repository::{HaveUserRepository, UserRepository};

pub trait UserService: HaveUserRepository {
    fn find(&self, id: &UserID) -> Result<User> {
        id.validate()?;
        self.user_repository().find(id)
    }

    fn list(&self, paging: Paging) -> Result<(Vec<User>, paging::Token)> {
        self.user_repository().list(paging.token, paging.limit)
    }

    fn create(&self, user: User) -> Result<()> {
        user.validate()?;
        self.user_repository().create(user)
    }

    fn update(&self, user: User) -> Result<()> {
        user.validate()?;
        self.user_repository().update(user)
    }

    fn delete(&self, id: &UserID) -> Result<()> {
        id.validate()?;
        self.user_repository().delete(id)
    }
}

impl<T: HaveUserRepository> UserService for T {}

pub trait HaveUserService {
    type UserService: UserService;
    fn user_service(&self) -> &Self::UserService;
}

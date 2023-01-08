use crate::domain::model::result::Result;
use crate::domain::model::user::{User, UserID};

pub trait UserRepository {
    fn create(&self, user: User) -> Result<()>;
    fn update(&self, user: User) -> Result<()>;
    fn find(&self, id: &UserID) -> Result<User>;
    fn list(&self, page_token: paging::Token, limit: usize) -> Result<(Vec<User>, paging::Token)>;
    fn delete(&self, id: &UserID) -> Result<()>;
}

pub trait HaveUserRepository {
    type UserRepository: UserRepository;
    fn user_repository(&self) -> &Self::UserRepository;
}

use std::cmp::min;

use std::sync::Mutex;
use lazy_static::lazy_static;
use crate::domain::model::result::Result;
use crate::domain::model::error::{DomainError};
use crate::domain::model::user::{User, UserID, UserName};
use crate::domain::repository::UserRepository;
use crate::infrastructure::memory::UserMemoryRepository;

lazy_static! {
    static ref STORE: Mutex<Vec<User>> = Mutex::new(
        (0..6).map(|i| User::generate(UserName::new(format!("user_{i}")))).collect()
    );
}


impl UserMemoryRepository {
    pub fn new() -> Self {
        UserMemoryRepository {}
    }
}

impl UserRepository for UserMemoryRepository {
    fn create(&self, user: User) -> Result<()> {
        let mut g = STORE
            .lock()
            .map_err(|_| DomainError::Conflict)?;
        g.push(user);
        Ok(())
    }

    fn update(&self, user: User) -> Result<()> {
        let mut g = STORE
            .lock()
            .map_err(|_| DomainError::Conflict)?;
        let index = g.iter()
            .position(|u| u.id == user.id.clone())
            .ok_or(DomainError::UserNotFound(user.id.clone()))?;
        g[index] = user;
        Ok(())
    }

    fn find(&self, id: &UserID) -> Result<User> {
        let g = STORE.lock().map_err(|_| DomainError::Conflict)?;
        let user = g.iter()
            .find(|u| &u.id == id)
            .ok_or(DomainError::UserNotFound(id.clone()))?;
        Ok(user.clone())
    }

    fn list(&self, page_token: paging::Token, limit: usize) -> Result<(Vec<User>, paging::Token)> {
        let g = STORE.lock().map_err(|_| DomainError::Conflict)?;

        let index = if page_token.is_empty() {
            0
        } else {
            let from: UserID = paging::decode_token(page_token)
                .ok_or(DomainError::InvalidPagingToken)?;
            g.iter().position(|u| u.id == from).ok_or(DomainError::InvalidPagingToken)?
        };


        let limit_for_next_token = min(index + limit, g.len());
        let a = &g.to_vec()[index..limit_for_next_token];
        let next_token: paging::Token = match g.get(limit_for_next_token) {
            None => "".to_string(),
            Some(u) => paging::encode_token(u.id.clone()),
        };
        Ok((a.to_vec(), next_token))
    }

    fn delete(&self, id: &UserID) -> Result<()> {
        let mut g = STORE.lock().map_err(|_| DomainError::Conflict)?;
        let index = g.iter()
            .position(|u| u.id == id.clone())
            .ok_or(DomainError::UserNotFound(id.clone()))?;
        g.remove(index);
        Ok(())
    }
}

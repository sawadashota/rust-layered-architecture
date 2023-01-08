use crate::domain::repository::HaveUserRepository;
use crate::domain::service::user::HaveUserService;
use crate::infrastructure::memory::UserMemoryRepository;
use crate::usecase::health_check::HaveHealthCheckUseCase;
use crate::usecase::user::HaveUserUseCase;

pub struct Registry {
    user_repository: UserMemoryRepository,
}

impl Registry {
    pub fn new() -> Self {
        Self {
            user_repository: UserMemoryRepository::new(),
        }
    }
}

impl HaveUserRepository for Registry {
    type UserRepository = UserMemoryRepository;
    fn user_repository(&self) -> &Self::UserRepository {
        &self.user_repository
    }
}

impl HaveUserService for Registry {
    type UserService = Self;
    fn user_service(&self) -> &Self::UserService {
        self
    }
}

impl HaveUserUseCase for Registry {
    type UserUseCase = Self;
    fn user_usecase(&self) -> &Self::UserUseCase {
        self
    }
}

impl HaveHealthCheckUseCase for Registry {
    type HealthCheckUseCase = Self;
    fn health_check_usecase(&self) -> &Self::HealthCheckUseCase {
        self
    }
}

use crate::domain::model::health::Health;

pub trait HealthCheckUseCase {
    fn alive(&self) -> Health {
        Health::ok()
    }
}

impl <T: HaveHealthCheckUseCase> HealthCheckUseCase for T {}

pub trait HaveHealthCheckUseCase {
    type HealthCheckUseCase: HealthCheckUseCase;
    fn health_check_usecase(&self) -> &Self::HealthCheckUseCase;
}

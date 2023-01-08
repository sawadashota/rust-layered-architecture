use std::sync::Arc;
use axum::extract::State;
use axum::Json;
use crate::domain::model::health::Health;
use crate::usecase::health_check::{HaveHealthCheckUseCase, HealthCheckUseCase};

pub async fn alive(
    State(r): State<Arc<impl HaveHealthCheckUseCase>>
) -> Json<Health> {
    Json(r.health_check_usecase().alive())
}

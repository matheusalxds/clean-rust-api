use std::sync::Arc;

use axum::extract::State;
use axum::routing::get;
use axum::{Json, Router};

use super::health_service::HealthService;

async fn health(State(service): State<Arc<HealthService>>) -> Json<impl serde::Serialize> {
    Json(service.healthcheck())
}

pub fn health_router() -> Router<Arc<HealthService>> {
    Router::new().route("/health", get(health))
}

use std::sync::Arc;

use axum::Router;
use tracing_subscriber::EnvFilter;

use self::modules::health::health_service::HealthService;
use self::modules::health::router::health_router;
use self::modules::user::main::factory::make_user_router;
mod modules;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_target(false)
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            let env = std::env::var("RUST_ENV").unwrap_or_default();
            if env == "production" {
                EnvFilter::new("info")
            } else {
                EnvFilter::new("debug")
            }
        }))
        .init();

    let health_service = Arc::new(HealthService::new());

    let app = Router::new()
        .merge(health_router().with_state(health_service))
        .merge(make_user_router());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3033").await.unwrap();

    println!("Server is running on port: 3033");

    axum::serve(listener, app).await.unwrap();
}

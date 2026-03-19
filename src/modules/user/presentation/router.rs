use std::sync::Arc;

use axum::Router;
use axum::routing::post;

use crate::modules::user::application::user_service::UserService;

use super::controller::{create_user, list_users};

pub fn user_router(service: Arc<UserService>) -> Router {
    Router::new()
        .route("/users", post(create_user).get(list_users))
        .with_state(service)
}

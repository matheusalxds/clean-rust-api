use std::sync::Arc;

use axum::Json;
use axum::extract::{Query, State};
use axum::response::IntoResponse;

use crate::modules::shared::http_helper::{created, ok};
use crate::modules::user::application::user_service::UserService;

use super::dto::{CreateUserBody, ListUsersQuery, UserResponse};

pub async fn create_user(
    State(service): State<Arc<UserService>>,
    Json(body): Json<CreateUserBody>,
) -> impl IntoResponse {
    service.create(body.into());

    created()
}

pub async fn list_users(
    State(service): State<Arc<UserService>>,
    Query(query): Query<ListUsersQuery>,
) -> impl IntoResponse {
    let users = match query.email {
        Some(email) => service.find_by_email(&email),
        None => service.find_all(),
    };
    let response: Vec<UserResponse> = users.iter().map(UserResponse::from).collect();

    ok(response)
}

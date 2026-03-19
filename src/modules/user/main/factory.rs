use std::sync::Arc;

use axum::Router;

use crate::modules::user::application::user_service::UserService;
use crate::modules::user::infra::repo::UserRepository;
use crate::modules::user::presentation::router::user_router;

pub fn make_user_router() -> Router {
    let repo = Arc::new(UserRepository::new());
    let service = Arc::new(UserService::new(repo));
    user_router(service)
}

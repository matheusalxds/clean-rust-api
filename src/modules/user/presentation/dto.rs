use serde::{Deserialize, Serialize};

use crate::modules::user::domain::dto::CreateUserDTO;
use crate::modules::user::domain::entity::User;

#[derive(Deserialize)]
pub struct CreateUserBody {
    pub name: String,
    pub email: String,
    pub password: String,
}

impl From<CreateUserBody> for CreateUserDTO {
    fn from(body: CreateUserBody) -> Self {
        Self {
            name: body.name,
            email: body.email,
            password: body.password,
        }
    }
}

#[derive(Serialize)]
pub struct UserResponse {
    pub name: String,
    pub email: String,
}

impl From<&User> for UserResponse {
    fn from(user: &User) -> Self {
        Self {
            name: user.name.clone(),
            email: user.email.clone(),
        }
    }
}

#[derive(Deserialize)]
pub struct ListUsersQuery {
    pub email: Option<String>,
}

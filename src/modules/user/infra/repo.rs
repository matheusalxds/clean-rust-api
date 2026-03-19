use std::sync::Mutex;

use crate::modules::user::domain::dto::CreateUserDTO;
use crate::modules::user::domain::entity::User;
use crate::modules::user::domain::repo::UserRepoTrait;

// Declare internal fields
pub struct UserRepository {
    users: Mutex<Vec<User>>,
}

// constructor
impl UserRepository {
    pub fn new() -> Self {
        Self {
            users: Mutex::new(vec![]),
        }
    }
}

impl UserRepoTrait for UserRepository {
    fn save(&self, dto: CreateUserDTO) {
        let mut users = self.users.lock().unwrap();
        users.push(User {
            name: dto.name,
            email: dto.email,
            password: dto.password,
        })
    }

    fn find_all(&self) -> Vec<User> {
        let users = self.users.lock().unwrap();
        users
            .iter()
            .map(|u| User {
                name: u.name.clone(),
                email: u.email.clone(),
                password: u.password.clone(),
            })
            .collect()
    }

    fn find_by_email(&self, email: &str) -> Vec<User> {
        let users = self.users.lock().unwrap();
        users
            .iter()
            .filter(|u| u.email == email)
            .map(|u| User {
                name: u.name.clone(),
                email: u.email.clone(),
                password: u.password.clone(),
            })
            .collect()
    }
}

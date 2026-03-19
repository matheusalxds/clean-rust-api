use std::sync::Arc;

use crate::modules::shared::logger::{EMPTY, Logger};
use crate::modules::user::domain::dto::CreateUserDTO;
use crate::modules::user::domain::entity::User;
use crate::modules::user::domain::repo::UserRepoTrait;

pub struct UserService {
    repo: Arc<dyn UserRepoTrait>,
    logger: Logger,
}

impl UserService {
    pub fn new(repo: Arc<dyn UserRepoTrait>) -> Self {
        Self {
            repo,
            logger: Logger::new("UserService"),
        }
    }

    pub fn create(&self, dto: CreateUserDTO) {
        self.logger.info_start("create", &dto);
        self.repo.save(dto);
        self.logger.info_end("create", &EMPTY);
    }

    pub fn find_all(&self) -> Vec<User> {
        self.logger.info_start("find_all", &EMPTY);
        let users = self.repo.find_all();
        self.logger.info_end("find_all", &users);

        users
    }

    pub fn find_by_email(&self, email: &str) -> Vec<User> {
        self.logger.info_start("find_by_email", &email);
        let user = self.repo.find_by_email(email);
        self.logger.info_end("find_by_email", &user);

        user
    }
}

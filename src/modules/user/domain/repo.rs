use super::dto::CreateUserDTO;
use super::entity::User;

// Interface
pub trait UserRepoTrait: Send + Sync {
    fn save(&self, dto: CreateUserDTO);
    fn find_all(&self) -> Vec<User>;
    fn find_by_email(&self, email: &str) -> Vec<User>;
}

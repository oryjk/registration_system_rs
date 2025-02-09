use crate::models::user::User;
use crate::ports::user_repository::UserRepository;
use std::error::Error;

pub struct UserService<R: UserRepository> {
    user_repository: R,
}

impl<R: UserRepository> UserService<R> {
    pub fn new(user_repository: R) -> Self {
        UserService { user_repository }
    }

    pub async fn get_user(&self, user_id: &str) -> Result<Option<User>, Box<dyn Error>> {
        self.user_repository.get_user(user_id).await
    }

    pub async fn update_user_avatar(&self, user_id: &str, avatar_url: &str) -> Result<(), Box<dyn Error>> {
        self.user_repository.update_user_avatar(user_id, avatar_url).await
    }
}
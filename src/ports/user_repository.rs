use crate::models::user::User;
use async_trait::async_trait;
use sqlx::MySqlPool;

#[async_trait]
pub trait UserRepository: Send + Sync + 'static {
    async fn get_user(&self, user_id: &str) -> Result<Option<User>, Box<dyn std::error::Error>>;
    async fn update_user_avatar(&self, user_id: &str, avatar_url: &str) -> Result<(), Box<dyn std::error::Error>>;
}

pub struct MySQLUserRepository {
    pub pool: MySqlPool,
}

impl MySQLUserRepository {
    pub fn new(pool: MySqlPool) -> Self {
        MySQLUserRepository { pool }
    }
}

#[async_trait]
impl UserRepository for MySQLUserRepository {
    async fn get_user(&self, user_id: &str) -> Result<Option<User>, Box<dyn std::error::Error>> {
        let result: Option<User> = sqlx::query_as("SELECT open_id, avatar_url, is_manager, latest_login_date, nickname, real_name, union_id, username FROM rs_user_info WHERE open_id = ?")
            .bind(user_id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(result)
    }

    async fn update_user_avatar(&self, user_id: &str, avatar_url: &str) -> Result<(), Box<dyn std::error::Error>> {

        sqlx::query("UPDATE rs_user_info SET avatar_url = ? WHERE open_id = ?")
            .bind(avatar_url)
            .bind(user_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
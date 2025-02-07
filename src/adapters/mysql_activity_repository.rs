use async_trait::async_trait;
use sqlx::MySqlPool;
use crate::ports::activity_repository::ActivityRepository;  // Correct relative path
use uuid::Uuid;
use crate::models::activity::Activity;

#[derive(Debug, Clone)]
pub struct MySqlActivityRepository {
    pool: MySqlPool,
}

impl MySqlActivityRepository {
    pub fn new(pool: MySqlPool) -> Self {
        MySqlActivityRepository { pool }
    }
}

#[async_trait]
impl ActivityRepository for MySqlActivityRepository {
    async fn get_activity(&self, id: &str) -> Result<Option<Activity>, Box<dyn std::error::Error>> {
        let result = sqlx::query_as!(
            Activity,
            "SELECT id, cover, end_time, holding_date, location, name, start_time, status FROM rs_activity WHERE id = ?",
            id
        )
            .fetch_optional(&self.pool)
            .await?;

        Ok(result)
    }

    async fn update_activity_status(&self, id: &str, new_status: i32) -> Result<(), Box<dyn std::error::Error>> {
        sqlx::query!("UPDATE rs_activity SET status = ? WHERE id = ?", new_status, id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn create_activity(&self, activity: &Activity) -> Result<(), Box<dyn std::error::Error>> {
        let id = if activity.id.is_empty() {
            Uuid::new_v4().to_string()
        } else {
            activity.id.clone()
        };

        sqlx::query!(
            "INSERT INTO rs_activity (id, cover, end_time, holding_date, location, name, start_time, status) VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
            id,
            activity.cover,
            activity.end_time,
            activity.holding_date,
            activity.location,
            activity.name,
            activity.start_time,
            activity.status
        )
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn list_activities(&self) -> Result<Vec<Activity>, Box<dyn std::error::Error>> {
        let result = sqlx::query_as!(
            Activity,
            "SELECT id, cover, end_time, holding_date, location, name, start_time, status FROM rs_activity"
        )
            .fetch_all(&self.pool)
            .await?;

        Ok(result)
    }
}
use async_trait::async_trait;


#[async_trait]
pub trait ActivityRepository: Send + Sync + Debug {
    async fn get_activity(&self, id: &str) -> Result<Option<Activity>, Box<dyn std::error::Error>>;
    async fn update_activity_status(&self, id: &str, new_status: i32) -> Result<(), Box<dyn std::error::Error>>;
    async fn create_activity(&self, activity: &Activity) -> Result<(), Box<dyn std::error::Error>>;
    async fn list_activities(&self) -> Result<Vec<Activity>, Box<dyn std::error::Error>>;
}

use std::fmt::Debug;
use crate::models::activity::Activity;
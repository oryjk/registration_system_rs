use crate::models::activity::Activity;
use crate::ports::activity_repository::ActivityRepository;

#[derive(Debug)]
pub struct ActivityService<R: ActivityRepository> {
    repository: R,
}

impl<R: ActivityRepository> ActivityService<R> {
    pub fn new(repository: R) -> Self {
        ActivityService { repository }
    }

    pub async fn get_activity(&self, id: &str) -> Result<Option<Activity>, Box<dyn std::error::Error>> {
        self.repository.get_activity(id).await
    }

    pub async fn update_activity_status(&self, id: &str, new_status: i32) -> Result<(), Box<dyn std::error::Error>> {
        self.repository.update_activity_status(id, new_status).await
    }

    pub async fn create_activity(&self, activity: &Activity) -> Result<(), Box<dyn std::error::Error>> {
        self.repository.create_activity(activity).await
    }

    pub async fn list_activities(&self) -> Result<Vec<Activity>, Box<dyn std::error::Error>> {
        self.repository.list_activities().await
    }
}
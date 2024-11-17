use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Activity {
    pub(crate) id: String,
    pub(crate) cover: Option<String>,
    pub(crate) end_time: Option<NaiveDateTime>,
    pub(crate) holding_date: Option<NaiveDateTime>,
    pub(crate) location: Option<String>,
    pub(crate) name: Option<String>,
    pub(crate) start_time: Option<NaiveDateTime>,
    pub(crate) status: i32,
}


#[derive(Debug, FromRow, Serialize, Deserialize, Clone)]
pub struct ActivityInfo {
    pub(crate) activity_id: String,
    pub(crate) color: Option<String>,
    pub(crate) opposing: Option<String>,
    pub(crate) opposing_color: Option<String>,
}

#[derive(Serialize)]
pub struct ActivityWithInfo {
    pub activity: Activity,
    pub activity_info: Option<ActivityInfo>,
}
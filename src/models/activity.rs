use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Deserialize, Serialize, FromRow, Clone)]
pub struct Activity {
    pub id: String,
    pub cover: Option<String>,
    pub end_time: Option<NaiveDateTime>, // Use NaiveDateTime initially
    pub holding_date: Option<NaiveDateTime>, // Use NaiveDateTime initially
    pub location: Option<String>,
    pub name: Option<String>,
    pub start_time: Option<NaiveDateTime>, // Use NaiveDateTime initially
    pub status: i32,
}
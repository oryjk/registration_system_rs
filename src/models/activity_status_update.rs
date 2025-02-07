use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ActivityStatusUpdate {
    pub status: i32,
}
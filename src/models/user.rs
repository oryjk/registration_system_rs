use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserTeam {
    pub user_id: String,
    pub team_id: i32,
}

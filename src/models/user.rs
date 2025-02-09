use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct User {
    pub open_id: String,          // 主键，对应数据库的 open_id
    pub avatar_url: Option<String>, // 可选的头像 URL, 可以为 NULL
    pub is_manager: bool,        // 是否是管理员，对应数据库的 is_manager (bit 类型映射为 bool)
    pub latest_login_date: Option<chrono::DateTime<chrono::Utc>>, // 最近登录日期, 可以为 NULL
    pub nickname: Option<String>,    // 昵称, 可以为 NULL
    pub real_name: Option<String>,   // 真实姓名, 可以为 NULL
    pub union_id: Option<String>,    // union_id, 可以为 NULL
    pub username: Option<String>,    // 用户名, 可以为 NULL
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserTeam {
    pub user_id: String,
    pub team_id: i32,
}

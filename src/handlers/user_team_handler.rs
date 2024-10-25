use actix_web::{web, HttpResponse};
use chrono::{TimeZone, Utc};
use chrono_tz::Tz;
use serde::Serialize;
use sqlx::MySqlPool;

use crate::models::user::UserTeam;

#[derive(Serialize)]
struct MyResponse {
    key: String,
}
pub async fn bind_user_team(path: web::Path<UserTeam>, pool: web::Data<MySqlPool>) -> HttpResponse {
    let china_tz: Tz = "Asia/Shanghai".parse().unwrap();
    let local_time = china_tz.from_utc_datetime(&Utc::now().naive_utc()).with_timezone(&Utc);
    let result: Result<_, sqlx::Error> =
        sqlx::query("INSERT INTO rs_user_team (user_id, team_id, join_time) VALUES (?, ?, ?)")
            .bind(&path.user_id)
            .bind(&path.team_id)
            .bind(&local_time)
            .execute(pool.get_ref())
            .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("创建成功"),
        Err(_) => {
            let response = MyResponse {
                key: "绑定球队失败".to_string()
            };
            let json_response = serde_json::to_string(&response).unwrap();

            HttpResponse::InternalServerError()
                .content_type("application/json; charset=utf-8")
                .body(json_response)
        }
    }
}

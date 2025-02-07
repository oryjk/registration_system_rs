use crate::models::user::UserTeam;
use actix_web::http::header::ContentType;
use actix_web::{web, HttpResponse};
use chrono::{NaiveDateTime, TimeZone, Utc};
use chrono_tz::Tz;
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;

#[derive(Serialize)]
struct MyResponse {
    key: String,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow, Clone)]
struct UserTeamDO {
    id: i64,
    user_id: String,
    team_id: String,
    join_time: NaiveDateTime,
}

// #[cfg(not(feature = "skip_db_check"))]
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
        Ok(_) => HttpResponse::Ok().insert_header(ContentType::plaintext()).body("创建成功"),
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
// #[cfg(not(feature = "skip_db_check"))]
pub async fn unbind_user_team(path: web::Path<UserTeam>, pool: web::Data<MySqlPool>) -> HttpResponse {
    let user_id = &path.user_id;
    let team_id = &path.team_id.to_string();


    let user_team = sqlx::query_as!(UserTeamDO, "SELECT * FROM rs_user_team WHERE user_id=? AND team_id=?", user_id, team_id)
        .fetch_one(pool.get_ref())
        .await;

    match user_team {
        Ok(re) => {
            if let Err(_) = sqlx::query!("DELETE FROM rs_user_team WHERE id=?",re.id)
                .execute(pool.get_ref())
                .await {
                let response = MyResponse {
                    key: "解绑球队失败".to_string()
                };
                let json_response = serde_json::to_string(&response).unwrap();

                return HttpResponse::InternalServerError()
                    .content_type("application/json; charset=utf-8")
                    .body(json_response);
            }

            HttpResponse::Ok().body("解绑成功")
        }
        Err(err) => {
            eprintln!("Error fetching user team: {:?}", err);
            let response = MyResponse {
                key: "解绑球队失败".to_string()
            };
            let json_response = serde_json::to_string(&response).unwrap();

            HttpResponse::InternalServerError()
                .content_type("application/json; charset=utf-8")
                .body(json_response)
        }
    }
}

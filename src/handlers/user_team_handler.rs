use actix_web::{web, HttpResponse, Responder};
use sqlx::{MySql, MySqlPool};

pub async fn bind_user_team(pool: web::Data<MySqlPool>, user_id: &str, team_id: i32) ->impl Responder {
    let result = sqlx::query("INSERT INTO rs_user_teams (user_id, team_id) VALUES (?, ?)")
        .bind(user_id)
        .bind(team_id)
        .execute(pool.get_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("创建成功"),
        Err(_) => HttpResponse::InternalServerError().body("绑定球队失败")
    }
}

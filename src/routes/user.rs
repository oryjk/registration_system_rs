use crate::handlers::user_team_handler::{bind_user_team, unbind_user_team};
use actix_web::web;
use crate::handlers::user_handler::upload_avatar_handler;
use crate::ports::user_repository::MySQLUserRepository;

pub fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/users")
            .route("/teams/bind/{user_id}/{team_id}", web::post().to(bind_user_team))
            .route("/upload/avatar/{user_id}", web::post().to(upload_avatar_handler::<MySQLUserRepository>))
            .route("/teams/unbind/{user_id}/{team_id}", web::post().to(unbind_user_team))
            .route("/teams/unbind/{user_id}/{team_id}", web::post().to(unbind_user_team))
    );
}

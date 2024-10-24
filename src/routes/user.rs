use actix_web::{web, Scope};
use crate::handlers::user_team_handler::bind_user_team;

pub fn user_routes() -> Scope {
    web::scope("/api/users")
        .route("/teams/{user_id}/{team_id}", web::post().to(bind_user_team))
}
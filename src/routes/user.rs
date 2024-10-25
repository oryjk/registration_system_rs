use crate::handlers::user_team_handler::bind_user_team;
use actix_web::web;

pub fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/users")
            .route("/teams/{user_id}/{team_id}", web::get().to(bind_user_team)),
    );
}

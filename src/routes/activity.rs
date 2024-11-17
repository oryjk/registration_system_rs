use actix_web::web;
use crate::handlers::activity_handler::query_all_activities;

pub fn activity_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/activity")
            .route("/all", web::get().to(query_all_activities)),
    );
}
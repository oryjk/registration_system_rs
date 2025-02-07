use crate::adapters::mysql_activity_repository::MySqlActivityRepository;
use crate::handlers::activity_handler::{create_activity_handler, get_activity_handler, list_activities_handler, update_activity_status_handler};
use actix_web::web;

pub fn activity_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/activity")
            .route("/{id}", web::get().to(get_activity_handler::<MySqlActivityRepository>))
            .route("/{id}/status", web::put().to(update_activity_status_handler::<MySqlActivityRepository>))
            .route("/", web::post().to(create_activity_handler::<MySqlActivityRepository>))
            .route("/all", web::get().to(list_activities_handler::<MySqlActivityRepository>))
    );
}
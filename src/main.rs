use std::env;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Route, Scope};

mod db;
mod handlers;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = db::connections::create_pool(&database_url)
        .await
        .expect("Failed to create pool");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(routes::user::user_routes)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

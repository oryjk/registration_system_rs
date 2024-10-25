use actix_web::{web, App, HttpServer};

mod db;
mod handlers;
mod models;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = db::connections::create_pool()
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

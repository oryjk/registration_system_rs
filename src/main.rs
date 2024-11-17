use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::env;

mod db;
mod handlers;
mod models;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 加载环境变量
    dotenv().ok();
    let server_port = env::var("SERVER_PORT")
        .expect("无法获取到端口环境变量，请设置")
        .parse::<u16>()
        .expect("端口环境变量不是有效的u16数字");
    let pool = db::connections::create_pool()
        .await
        .expect("Failed to create pool");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(routes::user::user_routes)
            .configure(routes::activity::activity_routes)
    })
        .bind(("127.0.0.1", server_port))?
        .run()
        .await
}

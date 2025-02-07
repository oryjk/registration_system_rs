use crate::adapters::mysql_activity_repository::MySqlActivityRepository;
use crate::application::activity_service::ActivityService;
use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use dotenv::dotenv;
use std::env;

mod adapters;
mod application;
mod db;
mod handlers;
mod models;
mod ports;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 加载环境变量
    dotenv().ok();
    env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let server_port = env::var("SERVER_PORT")
        .expect("无法获取到端口环境变量，请设置")
        .parse::<u16>()
        .expect("端口环境变量不是有效的u16数字");
    let pool = db::connections::create_pool()
        .await
        .expect("Failed to create pool");

    let activity_repository = MySqlActivityRepository::new(pool.clone());
    let activity_service = ActivityService::new(activity_repository);
    let activity_service_data = Data::new(activity_service);

    HttpServer::new(move || {
        let cors = Cors::permissive(); // 允许所有来源的请求 (不安全，仅用于开发)
        App::new()
            .wrap(Logger::default())
            .wrap(cors) // 添加 CORS 中间件
            .app_data(activity_service_data.clone()) // 重要：使用 .clone() 创建共享的所有权
            .app_data(Data::new(pool.clone()))
            .configure(routes::user::user_routes)
            .configure(routes::activity::activity_routes)
    })
    .bind(("127.0.0.1", server_port))?
    .run()
    .await
}

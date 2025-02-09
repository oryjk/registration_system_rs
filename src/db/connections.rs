use std::env;

use dotenv::dotenv;
use sqlx::mysql::MySqlPool;
pub async fn create_pool() -> Result<MySqlPool, sqlx::Error> {
    // 加载环境变量
    dotenv().ok();

    // 从环境变量中获取数据库连接字符串
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("DATABASE_URL: {}", database_url);
    // 创建连接池
    let pool = MySqlPool::connect(&database_url).await?;

    Ok(pool)
}

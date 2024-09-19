use std::env;
use dotenvy::dotenv;
use sqlx::{Pool, Sqlite,migrate::MigrateDatabase,sqlite::SqlitePoolOptions};

pub async fn establish_connection() -> Pool<Sqlite> {
    dotenv().ok(); // 加载 .env 文件中的环境变量
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    if !Sqlite::database_exists(&database_url).await.unwrap_or(false) {
        println!("创建数据库{}", database_url);
        match Sqlite::create_database(&database_url).await {
            Ok(_) => println!("创建数据库成功"),
            Err(err) => panic!("创建数据库失败：{}", err)
        }
    }else{
        println!("数据库已存在，无需创建")
    }
    // 创建连接池
    let pool = SqlitePoolOptions::new()
        .max_connections(20)
        .connect(&database_url)
        .await
        .expect("can't connect to database");
    pool
}

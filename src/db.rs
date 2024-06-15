use std::env;

use sqlx::{types::time::PrimitiveDateTime, Pool, Sqlite, SqlitePool};

pub async fn establish_connection() -> Pool<Sqlite> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = SqlitePool::connect(&database_url)
        .await
        .expect("can't connect to database");
    pool
}
#[derive(sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub openid: String,
    pub session_id: String,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
}

use serde::Serialize;
use time::PrimitiveDateTime;

#[derive(sqlx::FromRow, Debug, Serialize)]
pub struct CounterRecord {
    pub id: i32,
    pub counter_id: i32,
    pub step: i32,
    pub begin: bool,
    pub end: i32,
    pub create_at: PrimitiveDateTime,
    pub update_at: PrimitiveDateTime,
}
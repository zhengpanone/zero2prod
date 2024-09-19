use serde::Serialize;
use time::PrimitiveDateTime;

#[derive(sqlx::FromRow, Debug, Serialize)]
pub struct CounterDO {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub value: i32,
    pub step: i32,
    pub input_step: bool,
    pub sequence: i32,
    pub create_at: PrimitiveDateTime,
    pub update_at: PrimitiveDateTime,
}
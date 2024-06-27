use axum::{
    extract::{Path, State},
    Json,
};
use serde::Deserialize;
use serde_json::{json, Value};
use sqlx::{ Pool, Sqlite};

use crate::db::CounterRecord;

use super::{counter::get_user_counter, jwt::Uid, ApiError};

#[derive(Debug, Deserialize)]
pub struct AddCounterRecord {
    pub counter_id: i32,
    pub step: i32,
}

pub async fn add(
    Uid(user_id): Uid,
    State(pool): State<Pool<Sqlite>>,
    Json(counter_record): Json<AddCounterRecord>,
) -> Result<Json<Value>, ApiError> {
    let counter = get_user_counter(counter_record.counter_id, user_id, &pool).await?;
    let next_value = counter.value + counter_record.step;

    sqlx::query(
        r#"insert into counter_records (counter_id,step,begin,end) values(?,?,?,?);
      update counters set value=?, updated_at=CURRENT_TIMESTAMP where id = ? "#,
    )
    .bind(counter_record.counter_id)
    .bind(counter_record.step)
    .bind(counter.value)
    .bind(next_value)
    .bind(next_value)
    .bind(counter_record.counter_id)
    .execute(&pool)
    .await?;
    Ok(Json(json!({})))
}

pub async fn list(
    Path(count_id): Path<i32>,
    State(pool): State<Pool<Sqlite>>,
    Uid(user_id): Uid,
) -> Result<Json<Vec<CounterRecord>>, ApiError> {
    get_user_counter(count_id, user_id, &pool).await?;
    let records = sqlx::query_as::<_, CounterRecord>(
        "select * from counter_records where counter_id = ? order by desc",
    )
    .bind(count_id)
    .fetch_all(&pool)
    .await?;
    Ok(Json(records))
}

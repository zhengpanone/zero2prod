use std::sync::Arc;
use crate::db::CounterDO;
use axum::{
    extract::{Path, State},
    Json
};
use serde::Deserialize;
use serde_json::{json, Value};
use sqlx::{Error, Pool, Sqlite};
use crate::AppState;
use crate::handlers::handlers::ApiError;

use super::{jwt::Uid};

pub async fn list_counter(
    Uid(user_id): Uid,
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<CounterDO>>, ApiError> {
    let pool = &state.db_pool;
    let counters = sqlx::query_as::<_, CounterDO>(
        "select * from counters where user_id = ? order by sequence desc",
    )
        .bind(user_id)
        .fetch_all(pool)
        .await?;
    Ok(Json(counters))
}

#[derive(Debug, Deserialize)]
pub struct AddCounter {
    pub name: String,
    pub value: i32,
    pub step: i32,
    pub input_step: bool,
}

pub async fn add_counter(
    Uid(user_id): Uid,
    State(state): State<Arc<AppState>>,
    Json(counter): Json<AddCounter>,
) -> Result<Json<Value>, ApiError> {
    let pool = &state.db_pool;
    let sequence: Result<(i32,), Error> = sqlx::query_as::<_, (i32,)>(
        "select sequence from counter where user_id = ? order by sequence desc limit 1",
    )
        .bind(user_id)
        .fetch_one(pool)
        .await;

    let sequence = match sequence {
        Ok((sequence, )) => sequence + 1,
        Err(Error::RowNotFound) => 0,
        Err(e) => return Err(ApiError::from(e)),
    };

    sqlx::query(
        "insert into counters (user_id,name,value,step,input_step,sequence) values(?,?,?,?,?,?)",
    )
        .bind(user_id)
        .bind(counter.name)
        .bind(counter.value)
        .bind(counter.step)
        .bind(counter.input_step)
        .bind(sequence)
        .execute(pool)
        .await?;
    Ok(Json(json!({})))
}

pub async fn show_counter(
    Uid(user_id): Uid,
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<Json<CounterDO>, ApiError> {
    let pool = &state.db_pool;
    let counter = get_user_counter(id, user_id, pool).await?;
    Ok(Json(counter))
}

pub async fn get_user_counter(
    id: i32,
    user_id: i32,
    pool: &Pool<Sqlite>,
) -> Result<CounterDO, ApiError> {
    sqlx::query_as::<_, CounterDO>("select * from counters where id=? and user_id=?")
        .bind(id)
        .bind(user_id)
        .fetch_one(pool)
        .await
        .map_err(|e| match e {
            Error::RowNotFound => ApiError::NotFound,
            _ => ApiError::from(e),
        })
}

#[derive(Debug, Deserialize)]
pub struct UpdateCounter {
    pub name: String,
    pub step: i32,
    pub input_step: bool,
}

pub async fn update_counter(
    Path(id): Path<i32>,
    Uid(user_id): Uid,
    State(state): State<Arc<AppState>>,
    Json(counter): Json<UpdateCounter>,
) -> Result<Json<Value>, ApiError> {
    let pool = &state.db_pool;
    get_user_counter(id, user_id, &pool).await?;
    sqlx::query(
        "update counters set name=?, step=?, input_step=?,update_at= CURRENT_TIMESTAMP where id=?",
    )
        .bind(counter.name)
        .bind(counter.step)
        .bind(counter.input_step)
        .bind(id)
        .execute(pool)
        .await?;
    Ok(Json(json!({})))
}

pub async fn delete_counter(
    Uid(user_id): Uid,
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<Json<Value>, ApiError> {
    let pool = &state.db_pool;
    get_user_counter(id, user_id, &pool).await?;
    sqlx::query(
        r#"delete from counters where id =?;
    delete from counters_records where counter_id =?;"#,
    )
        .bind(id)
        .bind(id)
        .execute(pool)
        .await?;
    Ok(Json(json!({})))
}

pub async fn top_counter(
    Path(id): Path<i32>,
    State(state): State<Arc<AppState>>,
    Uid(user_id): Uid,
) -> Result<Json<Value>, ApiError> {

    let pool = &state.db_pool;

    get_user_counter(id, user_id, &pool).await?;
    let sequence = sqlx::query_as::<_, (i32,)>(
        "select sequence from counters where user_id =? order by sequence desc limit 1",
    )
        .bind(user_id)
        .fetch_one(pool)
        .await;

    let sequence = match sequence {
        Ok((sequence, )) => sequence + 1,
        Err(e) => return Err(ApiError::from(e)),
    };

    sqlx::query("update counters set sequence =?, updated_at = CURRENT_TIMESTAMP where user_id =?")
        .bind(sequence)
        .bind(id)
        .execute(pool)
        .await?;

    Ok(Json(json!({})))
}

use std::sync::Arc;
use axum::Router;
use axum::routing::{get, post};
use crate::AppState;
use crate::handlers::counter_record::{add_counter_record, list_counter_record};

pub fn counter_record_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/counter_records", post(add_counter_record))
        .route("/counter_records", get(list_counter_record))
        .with_state(state)  // 定义用户信息路由
}
use std::sync::Arc;

use axum::Router;
use axum::routing::{get,post,put,delete};

use crate::AppState;
use crate::handlers::counter::{add_counter,list_counter,show_counter,update_counter,delete_counter,top_counter};

// 定义 /counter 路由分组
pub fn counter_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/add", get(add_counter))
        .route("/list", get(list_counter))
        .route("/get/:id", get(show_counter))
        .route("/put/:id", put(update_counter))
        .route("/delete/:id", delete(delete_counter))
        .route("/top/:id", post(top_counter))
        .with_state(state) // 定义用户信息路由
}
use std::sync::Arc;

use axum::Router;
use axum::routing::{get, post};

use crate::AppState;
use crate::handlers::user::{login, create_user, wx_login};

// 定义 /user 路由分组
pub fn user_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/login", post(login))
        .route("/users", post(create_user))
        .with_state(state)  // 定义用户信息路由
}
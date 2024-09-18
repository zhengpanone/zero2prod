use std::sync::Arc;
use axum::Router;
use crate::AppState;

mod user;
mod counter;
mod counter_record;
mod demo;





// 创建路由函数
pub fn create_app(state: Arc<AppState>) -> Router {
    Router::new()
        .nest("/user", user::user_routes(state.clone()))   // 将 /user 相关的路由分组
        .nest("/counter", counter::counter_routes(state.clone())) // 将 /counter 相关的路由分组
        .nest("/counter_record", counter_record::counter_record_routes(state.clone()))
        .nest("/demo",demo::demo_routes(state.clone()))
}
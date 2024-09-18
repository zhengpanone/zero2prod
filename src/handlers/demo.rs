use std::collections::HashMap;

use axum::extract::{Path, Query};
use axum::response::Html;

use crate::AppError;

pub async fn return_static() -> &'static str {
    "Hello, World!"
}
pub async fn return_hml() -> Html<&'static str> {
    Html("<h1>Hello world</h1>")
}

pub async fn handle_path(Path(id): Path<String>) -> String {
    format!("User ID: {}", id)
}

pub async fn query_handler(Query(params): Query<HashMap<String, String>>) -> String {
    format!("Query parameters: {:?}", params)
}


// 示例路由处理函数，返回错误或正常响应
pub async fn error_handler() -> Result<&'static str, AppError> {
    Err(AppError::BadRequest("Invalid request data".into()))
}

pub async fn not_found_handler() -> Result<&'static str, AppError> {
    // 返回一个 NotFound 错误
    Err(AppError::NotFound)
}
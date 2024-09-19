use std::collections::HashMap;
use std::time::Duration;
use askama::Template;
use axum::body::Body;

use axum::extract::{Path, Query, Request};
use axum::Json;
use axum::response::{Html, IntoResponse, Response};
use axum_extra::headers::UserAgent;
use axum_extra::TypedHeader;
use http::{HeaderMap, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::time::sleep;

use crate::AppError;

pub async fn return_static() -> &'static str {
    "Hello, World!"
}

pub async fn return_hml() -> Html<&'static str> {
    Html("<h1>Hello world</h1>")
}

pub async fn handle_path(Path(id): Path<String>) -> Json<serde_json::Value> {
    let result = format!("User ID: {}", id);
    Json(json!({ "result": result }))
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RequestData {
    ids: Vec<i32>, // Assuming you're expecting a vector of integers
}
pub async fn handle_json(Json(req): Json<RequestData>) -> Json<serde_json::Value> {
    let result = req.ids;
    Json(json!({ "data": result }))
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

pub async fn header_handler(TypedHeader(user_agent): TypedHeader<UserAgent>) -> String {
    format!("header.user_agent: {user_agent:?}")
}

pub async fn headers_handler(headers: HeaderMap) -> String {
    format!("headers: {headers:?}")
}

#[derive(Template, Default)]
#[template(path = "hello.html")]
struct HelloTemplate {
    name: String,
}

struct TemplateRespone<T>(T);

impl<T> IntoResponse for TemplateRespone<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {err}"),
            )
                .into_response(),
        }
    }
}

pub async fn handler_template(Path(name): Path<String>) ->  impl IntoResponse {
    let tpl = HelloTemplate{name};
    TemplateRespone(tpl)
}

pub async fn index_handler(req: Request<Body>) -> String {
    if rand::random() {
        sleep(Duration::new(0, 300000)).await;
    }
    if let Some(req_id) = req.headers().get("x-request-id") {
        // CompressionLayer只有当长度大于32时才会压缩
        format!("request[{:?}] {} with method {} ; make the body length longer than 32", req_id ,req.uri(), req.method())
    } else {
        format!("request[none] {} with method {} ; make the body length longer than 32", req.uri(), req.method())
    }

}


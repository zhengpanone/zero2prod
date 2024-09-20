use std::convert::Infallible;
use std::net::Ipv4Addr;
use std::sync::Arc;
use std::time::Duration;

use axum::{http::StatusCode, Json, Router};
use axum::body::Body;
use axum::error_handling::HandleErrorLayer;
use axum::extract::Request;
use axum::response::{Html, IntoResponse, Response};
use axum::routing::any_service;
use http::HeaderName;
use serde_json::json;
use tower::{BoxError, service_fn, ServiceBuilder};
use tower_http::request_id::{MakeRequestUuid, SetRequestIdLayer};
use tower_http::trace;
use tower_http::trace::TraceLayer;
use tracing::Level;

mod db;       // 数据库模块
mod routers;
mod handlers;// 路由处理模块
mod models;   // 数据模型模块
mod utils;    // 工具模块
mod config;
mod errors;

use config::APP_CONFIG;
use crate::db::connection::establish_connection;




// 定义自定义错误类型
enum AppError {
    NotFound,
    BadRequest(String),
}

// 实现 IntoResponse trait，以便将 AppError 转换为 HTTP 响应
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::NotFound => (
                StatusCode::NOT_FOUND,
                "The requested resource was not found",
            ).into_response(),
            AppError::BadRequest(message) => {
                // 返回 400 状态码和 JSON 消息
                let body = Json(json!({
                    "error": message,
                }));
                (StatusCode::BAD_REQUEST, body).into_response()
            }
        }
    }
}


// 自定义中间件处理错误
async fn handle_error(err: BoxError) -> impl IntoResponse {
    if err.is::<tower::timeout::error::Elapsed>() {
        // 处理请求超时的情况
        (StatusCode::REQUEST_TIMEOUT, "Request took too long").into_response()
    } else {
        // 处理其他错误
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Unhandled internal error: {}", err),
        )
            .into_response()
    }
}

#[derive(Clone)]
pub struct AppState {
    pub app_name: String,
    pub db_pool: sqlx::SqlitePool,  // 数据库连接池
}


/// 便于测试用例获取router实例
#[allow(dead_code)]
fn app(state: AppState) -> Router {

    let shared_state = Arc::new(state);

    // // 定义跟踪层 添加请求跟踪日志中间件
    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
        .on_request(trace::DefaultOnRequest::new().level(Level::INFO))
        .on_response(trace::DefaultOnResponse::new().level(Level::INFO));

    // 定义超时层
    let timeout_layer = tower::timeout::TimeoutLayer::new(Duration::from_secs(30));

    // 将共享状态传递给路由
    let app = routers::create_app(shared_state)
        // 使用 `handle_error` 作为全局错误处理的中间件
        .layer(
            // 官方推荐在ServiceBuilder上一次性载入
            ServiceBuilder::new()
                // 使用 HandleErrorLayer 捕获并处理错误
                .layer(HandleErrorLayer::new(handle_error))
                // 需要设置一个请求头的键名，一般叫x-request-id
                .layer(SetRequestIdLayer::new(HeaderName::from_static("x-request-id"), MakeRequestUuid))
                // 默认情况下不放行，所以需要根据自己需求设置必要的允许规则。
                // .layer(CorsLayer::new().allow_methods(axum::http::Method::GET).allow_origin(Any))
                .layer(trace_layer)
                .layer(timeout_layer)
        )
        .route(
            "/service1",
            any_service(service_fn(|req: Request<Body>| async move {
                let body = Body::from(format!("Hi from `{} /service1`", req.method()));
                let res = Response::new(body);
                Ok::<_, Infallible>(res)
            })))
        .fallback(fallback);

    app
}

#[tokio::main]
async fn main() {
    println!("{}", APP_CONFIG.server_port);
    println!("{}", APP_CONFIG.test.debug);

    // 创建数据库连接池
    let pool = establish_connection().await;
    // 创建共享状态
    let state = AppState {
        app_name: String::from("My Axum App"),
        db_pool:pool,  // 传入数据库连接池
    };

    // initialize tracing
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let addr = format!("{}:{}", Ipv4Addr::UNSPECIFIED, 8099);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("listening on http://{:?}", listener.local_addr().unwrap());
    axum::serve(listener, app(state)).await.unwrap();
}


async fn fallback() -> Html<&'static str> {
    Html("<h1>404</h1>")
}

#[cfg(test)]
mod tests {
    use axum::{
        body::Body,
        http::{self, Request, StatusCode},
    };
    use dotenvy::dotenv;
    use serde_json::{json, Value};
    // 实现了trait的对象要trait当前环境才能调用, 所以引用Service trait
    use tower::ServiceExt;
    use crate::db::connection::establish_connection;

    use super::*;

    // 提供`oneshot` 和 `ready`的便捷方法
    #[tokio::test]
    async fn hello_world() {
        dotenv().ok(); // 加载 .env 文件中的环境变量
        // 创建数据库连接池
        let pool = establish_connection().await;
        // 创建共享状态
        let state = AppState {
            app_name: String::from("My Axum App"),
            db_pool:pool,  // 传入数据库连接池
        };

        let app = app(state);

        // 因为router本身就是一个Service对象，所以可以直接像其他Service对象那样调用, 这里使用的是ServiceExt提供的oneshot方法
        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        // 检查状态码
        assert_eq!(response.status(), StatusCode::OK);

        // 检查响应内容
        // let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        // assert_eq!(&body[..], b"Hello, World!");
    }

    #[tokio::test]
    async fn json() {
        dotenv().ok(); // 加载 .env 文件中的环境变量
        // 创建数据库连接池
        let pool = establish_connection().await;
        // 创建共享状态
        let state = AppState {
            app_name: String::from("My Axum App"),
            db_pool:pool,  // 传入数据库连接池
        };
        let app = app(state);

        let response = app
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/demo/handle_json")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::to_vec(&json!({ "ids": [1, 2, 3, 4] })).unwrap(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(),1000).await.unwrap();
        let body: Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(body, json!({ "data": [1, 2, 3, 4] }));
    }

}

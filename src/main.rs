use std::net::Ipv4Addr;
use std::sync::Arc;
use std::time::Duration;

use axum::{http::StatusCode,  Json};
use axum::error_handling::HandleErrorLayer;
use axum::response::{IntoResponse, Response};
use config::APP_CONFIG;
use dotenvy::dotenv;
use serde_json::json;
use tower::{BoxError, ServiceBuilder};
use tower_http::trace;
use tower_http::trace::TraceLayer;
use tracing::Level;


mod config;
mod db;
mod handlers;
mod routers;


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

#[tokio::main]
async fn main() {
    println!("{}", APP_CONFIG.server_port);
    println!("{}", APP_CONFIG.test.debug);
    dotenv().ok(); // 加载 .env 文件中的环境变量

    // 创建数据库连接池
    let pool = db::establish_connection().await;
    // 创建共享状态
    let state = AppState {
        app_name: String::from("My Axum App"),
        db_pool:pool,  // 传入数据库连接池
    };
    let shared_state = Arc::new(state);

    // initialize tracing
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

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
            ServiceBuilder::new()
                // 使用 HandleErrorLayer 捕获并处理错误
                .layer(HandleErrorLayer::new(handle_error))
                .layer(trace_layer)
                .layer(timeout_layer)
        );

    let addr = format!("{}:{}", Ipv4Addr::UNSPECIFIED, 8099);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("listening on http://{:?}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}


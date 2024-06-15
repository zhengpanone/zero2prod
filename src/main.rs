pub mod api;
pub mod db;

use std::net::Ipv4Addr;

use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use tower_http::trace;
use tower_http::trace::TraceLayer;
use tracing::Level;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let pool = db::establish_connection().await;
    // initialize tracing
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
        .on_request(trace::DefaultOnRequest::new().level(Level::INFO))
        .on_response(trace::DefaultOnResponse::new().level(Level::INFO));

    let app = Router::new()
        .route("/", get(root))
        .route("/users", post(create_user))
        .route("/api/wx_counter/login", post(api::user::login))
        .layer(trace_layer)
        .with_state(pool);

    let addr = format!("{}:{}",Ipv4Addr::UNSPECIFIED, 8099);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("listening on http://{:?}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn create_user(Json(payload): Json<CreateUser>) -> (StatusCode, Json<User>) {
    let user = User {
        id: 1337,
        username: payload.username,
    };
    (StatusCode::CREATED, Json(user))
}

#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}

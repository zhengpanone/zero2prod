pub mod api;
pub mod config;
pub mod db;

use std::net::Ipv4Addr;

use axum::{
    http::StatusCode,
    routing::{delete, get, post, put},
    Json, Router,
};
use config::APP_CONFIG;
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use tower_http::trace;
use tower_http::trace::TraceLayer;
use tracing::Level;

#[tokio::main]
async fn main() {
    println!("{}", APP_CONFIG.server_port);
    println!("{}", APP_CONFIG.test.debug);
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
        .route("/api/wx_counter/counters", get(api::counter::list))
        .route("/api/wx_counter/counters", post(api::counter::add))
        .route("/api/wx_counter/counters/:id", get(api::counter::show))
        .route("/api/wx_counter/counters/:id", put(api::counter::update))
        .route("/api/wx_counter/counters/:id", delete(api::counter::delete))
        .route("/api/wx_counter/counters/:id/top", post(api::counter::top))
        .route("/api/xw_counter/counter_records", post(api::counter_record::add))
        .route("/api/xw_counter/counter_records", get(api::counter_record::list))
        .layer(trace_layer)
        .with_state(pool);

    let addr = format!("{}:{}", Ipv4Addr::UNSPECIFIED, 8099);

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

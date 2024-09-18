use std::sync::Arc;

use axum::Router;
use axum::routing::get;

use crate::AppState;
use crate::handlers::demo::{error_handler, handle_path, not_found_handler, query_handler, return_hml, return_static};

pub fn demo_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/bad_request", get(error_handler))
        .route("/not_found", get(not_found_handler))
        .route("/static", get(return_static))
        .route("/return_html", get(return_hml))
        .route("/handle_path/:id", get(handle_path))
        .route("/query_handler", get(query_handler))
        .with_state(state)
}
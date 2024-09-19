use std::sync::Arc;

use axum::Router;
use axum::routing::{get, post};

use crate::AppState;
use crate::handlers::demo::{error_handler, handle_json, handle_path, handler_template, header_handler, headers_handler, index_handler, not_found_handler, query_handler, return_hml, return_static};

pub fn demo_routes(state: Arc<AppState>) -> Router {
    Router::new()

        .route("/index2", get(index_handler))
        .route("/bad_request", get(error_handler))
        .route("/not_found", get(not_found_handler))
        .route("/static", get(return_static))
        .route("/return_html", get(return_hml))
        .route("/handle_path/:id", get(handle_path))
        .route("/handle_json", post(handle_json))
        .route("/query_handler", get(query_handler))
        .route("/header_handler", get(header_handler))
        .route("/headers_handler", get(headers_handler))
        .route("/:name", get(handler_template))

    .with_state(state)
}
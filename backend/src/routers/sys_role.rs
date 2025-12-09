use std::sync::Arc;

use axum::{
	routing::{delete, get, post, put},
	Router,
};

use crate::{
	handlers::sys_role::{create_role, delete_role, list_role, update_role},
	state::AppState,
};

#[allow(clippy::let_and_return)]
pub fn routes(state: Arc<AppState>) -> Router<Arc<AppState>> {
	let protect_routes = Router::new()
		.route("/list", get(list_role))
		.route("/create", post(create_role))
		.route("/update", put(update_role))
		.route("/delete", delete(delete_role))
		.with_state(state);
	// TODO  后续需要认证
	protect_routes
}

#[allow(clippy::let_and_return)]
pub fn admin_routes(state: Arc<AppState>) -> Router<Arc<AppState>> {
	let admin_routes = Router::new()
		.route("/list", get(list_role))
		.route("/create", post(create_role))
		.route("/update", put(update_role))
		.route("/delete", delete(delete_role));
	admin_routes
}

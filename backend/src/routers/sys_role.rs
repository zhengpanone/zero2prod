use std::sync::Arc;

use axum::{
	middleware,
	routing::{delete, get, post, put},
	Router,
};

use crate::middleware::auth;
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
		.route_layer(middleware::from_fn_with_state(
			state.clone(),
			auth::auth_middleware,
		));
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

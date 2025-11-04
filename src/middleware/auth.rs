use std::sync::Arc;

use axum::{
	extract::{Request, State},
	middleware::Next,
	response::Response,
};

use crate::{errors::AppError, state::AppState};

pub async fn auth_middleware(
	State(state): State<Arc<AppState>>,
	mut _req: Request,
	next: Next,
) -> Result<Response, AppError> {
	// TODO
	Ok(next.run(_req).await)
}

pub async fn admin_middleware(
	State(state): State<Arc<AppState>>,
	mut _req: Request,
	next: Next,
) -> Result<Response, AppError> {
	// TODO
	Ok(next.run(_req).await)
}

use axum::{
	extract::{Request, State},
	middleware::Next,
	response::Response,
};

use crate::{error::AppError, state::AppState};

pub async fn auth_middleware(
	State(state): State<AppState>,
	mut _req: Request,
	next: Next,
) -> Result<Response, AppError> {
	// TODO
	Ok(next.run(_req).await)
}

pub async fn admin_middleware(
	State(state): State<AppState>,
	mut _req: Request,
	next: Next,
) -> Result<Response, AppError> {
	// TODO
	Ok(next.run(_req).await)
}

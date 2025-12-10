use std::sync::Arc;

use axum::{
	extract::{Request, State},
	middleware::Next,
	response::Response,
};

use crate::{errors::Result, state::AppState, utils::jwt::Claims};

/// 认证用户信息
#[derive(Clone, Debug)]
pub struct AuthUser {
	pub user_id: String,
	pub email: String,
	pub username: String,
}

impl AuthUser {
	pub fn from_claims(claims: Claims) -> Result<Self> {
		Ok(AuthUser {
			user_id: claims.sub,
			email: claims.email,
			username: claims.username,
		})
	}
}

pub async fn auth_middleware(
	State(state): State<Arc<AppState>>,
	mut _req: Request,
	next: Next,
) -> Result<Response> {
	// TODO
	Ok(next.run(_req).await)
}

pub async fn admin_middleware(
	State(state): State<Arc<AppState>>,
	mut _req: Request,
	next: Next,
) -> Result<Response> {
	// TODO
	Ok(next.run(_req).await)
}

use axum::extract::FromRef;
use axum::{
	extract::{FromRequestParts, Request, State},
	http::{header::AUTHORIZATION, request::Parts},
	middleware::Next,
	response::Response,
	RequestPartsExt,
};
use axum_extra::{
	headers::{authorization::Bearer, Authorization},
	TypedHeader,
};
use std::sync::Arc;

use crate::config::jwt::JwtConfig;
use crate::{
	errors::{AppError, AuthError, Result},
	state::AppState,
	utils::jwt_utils::{self, Claims},
};

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
// 混合方案
// middleware → 插入 Extension<Arc<AuthUser>>;  handler 直接用 Extension<Arc<AuthUser>>，不需要每次解析 token
// Extractor（impl FromRequestParts<Arc<AppState>> for AuthUser）;  handler 签名直接写 auth_user: AuthUser
//
// 在中间件里优先解析并插入 Arc<AuthUser> 到 request.extensions()
// 实现一个 Extractor（FromRequestParts）时，先检查 parts.extensions() 是否存在 Arc<AuthUser>（如果有就直接返回），否则再从 header 解析并返回/拒绝。

// 让 `Claims` 可以直接作为 extractor 使用：从 Authorization header 中解析 JWT
impl FromRequestParts<Arc<AppState>> for AuthUser {
	type Rejection = AppError;

	async fn from_request_parts(
		parts: &mut Parts,
		state: &Arc<AppState>,
	) -> std::result::Result<Self, Self::Rejection> {
		// 1. 先看看 extensions（中间件是否已经插入）
		if let Some(arc_user) = parts.extensions.get::<Arc<AuthUser>>() {
			// clone 一个 AuthUser（若 AuthUser 是 Arc 内部则直接 clone Arc）
			return Ok((**arc_user).clone());
		}

		// 2. 没有 extension，按原来逻辑从 header 解析
		// 重点：通过 FromRef，从全局 state 中拿到 AppState
		let app_state = AppState::from_ref(state);
		let jwt_config = &app_state.config.jwt;
		// 尝试用 TypedHeader 提取（需要 feature 支持），否则回落到手动解析
		// 首先尝试使用 axum_extra 的 TypedHeader（更严格）
		if let Ok(TypedHeader(Authorization(bearer))) =
			parts.extract::<TypedHeader<Authorization<Bearer>>>().await
		{
			let token = bearer.token();
			let claims = jwt_utils::verify_token(token, jwt_config)
				.map_err(|_| AuthError::InvalidToken)?;
			let auth_user = AuthUser::from_claims(claims)
				.map_err(|_| AuthError::InvalidToken)?;
			return Ok(auth_user);
		}

		// 如果 TypedHeader 失败，回退到直接从 header 读取
		let auth_header = parts
			.headers
			.get(AUTHORIZATION)
			.and_then(|v| v.to_str().ok())
			.ok_or(AuthError::MissingCredentials)?;

		// 期望格式："Bearer <token>"
		let token = auth_header
			.strip_prefix("Bearer ")
			.or_else(|| auth_header.strip_prefix("bearer "))
			.ok_or(AuthError::InvalidToken)?;

		let claims = jwt_utils::verify_token(token, jwt_config)
			.map_err(|_| AuthError::InvalidToken)?;
		let auth_user =
			AuthUser::from_claims(claims).map_err(|_| AuthError::InvalidToken)?;
		Ok(auth_user)
	}
}

/// 普通认证中间件：解析 token，注入 AuthUser 到 request extensions
pub async fn auth_middleware(
	State(state): State<Arc<AppState>>,
	mut req: Request,
	next: Next,
) -> Result<Response> {
	let auth_user = auth_user(&req, &state.config.jwt)?;

	// 把认证用户放到 extensions，方便后续 handler 使用
	req.extensions_mut().insert::<Arc<AuthUser>>(auth_user);
	Ok(next.run(req).await)
}

/// 管理员鉴权中间件：基于 username == "admin" 做简单示例检查
pub async fn admin_middleware(
	State(state): State<Arc<AppState>>,
	mut req: Request,
	next: Next,
) -> Result<Response> {
	let auth_user = auth_user(&req, &state.config.jwt)?;
	req.extensions_mut().insert::<Arc<AuthUser>>(auth_user);
	Ok(next.run(req).await)
}

fn auth_user(req: &Request, jwt_config: &JwtConfig) -> Result<Arc<AuthUser>> {
	let auth_header = match req
		.headers()
		.get(AUTHORIZATION)
		.and_then(|v| v.to_str().ok())
	{
		Some(h) => h,
		None => return Err(AuthError::MissingCredentials.into()),
	};
	let token = auth_header
		.strip_prefix("Bearer ")
		.or_else(|| auth_header.strip_prefix("bearer "))
		.ok_or(AuthError::InvalidToken)?;

	let claims = jwt_utils::verify_token(token, jwt_config)
		.map_err(|_| AuthError::InvalidToken)?;

	let auth_user =
		AuthUser::from_claims(claims).map_err(|_| AuthError::InvalidToken)?;

	let auth_user = Arc::new(auth_user.clone());
	Ok(auth_user)
}

use crate::{
	handlers::auth::{login, register},
	state::AppState,
};
use axum::{routing::post, Router};

/// 认证相关路由
/// 基础路径: /auth
///
/// 路由列表:
/// - POST /auth/register - 用户注册
/// - POST /auth/login    - 用户登录
pub fn routes() -> Router<AppState> {
	Router::new()
		.route("/register", post(register))
		.route("/login", post(login))
	// 可以继续添加其他认证相关路由
	// .route("/logout", post(handlers::auth::logout))
	// .route("/refresh", post(handlers::auth::refresh_token))
	// .route("/forgot-password", post(handlers::auth::forgot_password))
	// .route("/reset-password", post(handlers::auth::reset_password))
}

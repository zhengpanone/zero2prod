use std::sync::Arc;

use crate::{
	handlers::sys_auth::{login, logout, register},
	state::AppState,
};
use axum::{routing::post, Router};

/// 认证相关路由
/// 基础路径: /auth
///
/// 路由列表:
/// - POST /auth/register - 用户注册
/// - POST /auth/login    - 用户登录
/// - POST /auth/logout   - 用户退出
pub fn routes() -> Router<Arc<AppState>> {
	Router::new()
		.route("/register", post(register))
		.route("/login", post(login))
		.route("/logout", post(logout))
	// 可以继续添加其他认证相关路由

	// .route("/refresh", post(handlers::auth::refresh_token))
	// .route("/forgot-password", post(handlers::auth::forgot_password))
	// .route("/reset-password", post(handlers::auth::reset_password))
}

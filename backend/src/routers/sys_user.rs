use std::sync::Arc;

use crate::{
	handlers::sys_user::{
		create_user, delete_user, get_by_id, list_users, update_user,
	},
	middleware::auth,
	state::AppState,
};
use axum::{
	middleware,
	routing::{delete, get, post, put},
	Router,
};

/// 用户相关路由
/// 基础路径: /api/users
///
/// 路由列表:
/// - GET    /api/user/detail - 获取用户详情 (需要认证)
/// - PUT    /api/users/update - 更新用户 (需要认证)
///
/// Constructs and returns a router with both public and protected routes.
///
/// The router includes:
/// - Public routes (no authentication required)
/// - Protected routes (require authentication middleware)
///
/// # Public Routes
/// - GET `/list`: List all users
///
/// # Protected Routes
/// - POST `/create`: Create a new user
/// - DELETE `/delete`: Delete a user
/// - PUT `/update`: Update a user
///
/// # Middleware
/// The protected routes are wrapped with an authentication middleware that verifies
/// requests before allowing access to the protected endpoints.
///
/// # Arguments
/// * `state` - Shared application state containing required dependencies
///
/// # Returns
/// A configured Router instance with both public and protected routes merged together
pub fn routes(state: Arc<AppState>) -> Router<Arc<AppState>> {
	// 公开路由（无需认证）
	let public_routes = Router::new().route("/list", get(list_users));
	// 受保护的 API 路由（需要认证）
	let protect_routes = Router::new()
		.route("/create", post(create_user))
		.route("/getById", get(get_by_id))
		.route("/delete", delete(delete_user))
		.route("/update", put(update_user))
		// 挂载认证中间件
		.route_layer(middleware::from_fn_with_state(
			state.clone(),
			auth::auth_middleware,
		));

	// 合并公开 + 受保护路由
	public_routes.merge(protect_routes)
}

/// 管理员用户路由
/// 需要管理员权限
/// 基础路径: /admin/users
///
/// 路由列表:
/// - GET    /api/user/list     - 获取用户列表
/// - DELETE /api/users/delete - 删除用户 (需要认证)
/// - POST   /api/user/create     - 创建用户 (需要认证)
///
#[allow(clippy::let_and_return)]
pub fn admin_routes(state: Arc<AppState>) -> Router<Arc<AppState>> {
	// 管理员路由（需要认证 + 管理员权限）
	let admin_routes = Router::new()
		.route("/list", get(list_users))
		.route("/create", post(create_user))
		.route("/update", put(list_users))
		.route("/delete", delete(list_users))
		.layer(middleware::from_fn_with_state(
			state.clone(),
			auth::admin_middleware,
		))
		.layer(middleware::from_fn_with_state(
			state.clone(),
			auth::auth_middleware,
		));

	admin_routes
}

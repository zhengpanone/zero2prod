use std::sync::Arc;

use axum::{routing::get, Router};

use crate::{handlers, state::AppState};

/// 健康检查相关路由
/// 所有路由都挂载在 /user 下
pub fn routes(state: Arc<AppState>) -> Router<Arc<AppState>> {
	// 公开路由（无需认证）
	Router::new()
		.route("/health", get(handlers::health::health_check))
		.route("/", get(handlers::health::hello))
	// 受保护的 API 路由（需要认证）

	// 管理员路由（需要认证 + 管理员权限）

	// 合并公开 + 受保护路由
}

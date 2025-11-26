use std::sync::Arc;

use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::state::AppState;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct HealthResponse {
	pub status: String,
	pub database: String,
}
/// 健康检查
#[utoipa::path(
    get,
    path = "/health",
    tag = "health",
    responses(
        (status = 200, description = "服务健康", body = HealthResponse),
        (status = 503, description = "服务不健康", body = HealthResponse)
    )
)]
pub async fn health_check(
	State(state): State<Arc<AppState>>,
) -> Json<HealthResponse> {
	let db_status = match sqlx::query("SELECT 1").fetch_one(&state.db).await {
		Ok(_) => "healthy".to_string(),
		Err(_) => "unhealthy".to_string(),
	};
	Json(HealthResponse {
		status: "ok".to_string(),
		database: db_status,
	})
}

pub async fn hello() -> &'static str {
	"Hello from web template!"
}

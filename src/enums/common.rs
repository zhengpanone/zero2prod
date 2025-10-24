use serde::{Deserialize, Serialize};
use sqlx::Type;
use utoipa::ToSchema;

/// 数据库中的 user_status_enum
#[derive(Debug, Clone, Serialize, Deserialize, Type, ToSchema)]
#[sqlx(type_name = "user_status_enum", rename_all = "lowercase")] // 对应 Postgres 枚举类型名
#[serde(rename_all = "lowercase")]
pub enum UserStatus {
	Active,
	Inactive,
	Banned,
}
#[allow(clippy::derivable_impls)]
impl Default for UserStatus {
	fn default() -> Self {
		UserStatus::Active
	}
}

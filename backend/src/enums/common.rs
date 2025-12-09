use serde::{Deserialize, Serialize};
use sqlx::Type;
use tracing::warn;
use utoipa::ToSchema;
/// 用户状态枚举
///
/// 定义用户在系统中的不同状态
/// 数据库中的 user_status_enum
#[derive(Debug, Clone, Serialize, Deserialize, Type, ToSchema)]
#[sqlx(type_name = "user_status_enum", rename_all = "lowercase")] // 对应 Postgres 枚举类型名
#[serde(rename_all = "lowercase")]
pub enum UserStatus {
	/// 活跃用户 - 可以正常使用系统功能
	#[schema(rename = "active")]
	Active,
	/// 非活跃用户 - 暂时无法使用系统
	#[schema(rename = "inactive")]
	Inactive,
	/// 被封禁用户 - 因违规行为被管理员封禁
	#[schema(rename = "banned")]
	Banned,
}
#[allow(clippy::derivable_impls)]
impl Default for UserStatus {
	fn default() -> Self {
		UserStatus::Active
	}
}

impl UserStatus {
	/// 检查用户是否活跃
	pub fn is_active(&self) -> bool {
		matches!(self, UserStatus::Active)
	}
	/// 检查用户是否可以被禁用
	pub fn can_be_deleted(&self) -> bool {
		matches!(self, UserStatus::Active)
	}
}

#[derive(Debug, Clone, Serialize, Deserialize, Type, ToSchema)]
#[sqlx(type_name = "role_status_enum")] // 对应 postgres 的类型名
#[sqlx(rename_all = "lowercase")] // 如果枚举变体小写对应 db 字符串
pub enum RoleStatus {
	Active,
	Inactive,
	Banned,
}

impl RoleStatus {
	pub fn from_option_str(status: Option<&str>) -> Self {
		match status.map(|s| s.to_lowercase()) {
			Some(s) if s == "active" || s == "0" => RoleStatus::Active,
			Some(s) if s == "inactive" || s == "1" => RoleStatus::Inactive,
			Some(s) if s == "banned" || s == "2" => RoleStatus::Banned,
			Some(s) => {
				warn!("Unknown status input: {}, default to Active", s);
				RoleStatus::Active
			}
			None => RoleStatus::Active,
		}
	}

	pub fn as_str(&self) -> &'static str {
		match self {
			RoleStatus::Active => "active",
			RoleStatus::Inactive => "inactive",
			RoleStatus::Banned => "banned",
		}
	}
}

#[allow(clippy::derivable_impls)]
impl Default for RoleStatus {
	fn default() -> Self {
		RoleStatus::Active
	}
}

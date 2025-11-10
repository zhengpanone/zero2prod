use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct SysDept {
	/// 部门ID
	pub id: Uuid,
	/// 部门名称
	pub name: String,

	/// 上级部门ID
	pub parent_id: Uuid,
	/// 排序值
	pub order_num: i32,
	/// 部门领导
	pub leader: String,
	/// 部门领导电话
	pub phone: String,
	/// 部门领导邮箱
	pub email: String,
	/// 部门状态
	pub status: String,
	pub created_at: DateTime<Utc>,
	pub updated_at: DateTime<Utc>,
}

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct SysRole {
	pub id: Uuid,
	// 角色名称
	pub name: String,

	pub description: String,

	// 角色状态
	pub status: String,

	// 排序值
	pub order_num: i32,

	// 角色备注
	pub remark: String,

	// 创建时间
	pub created_at: DateTime<Utc>,
	// 创建人
	pub created_by: String,

	// 更新时间
	pub updated_at: DateTime<Utc>,
	// 更新人
	pub updated_by: String,

	// 是否删除
	pub is_deleted: bool,
	// 删除时间
	pub deleted_at: DateTime<Utc>,
}

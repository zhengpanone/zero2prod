use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct SysRole {
	pub id: Uuid,
	// 角色名称
	pub name: String,

	pub description: String,

	// 角色状态
	pub status: String,
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

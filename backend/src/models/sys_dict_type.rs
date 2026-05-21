use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

// 字典类型
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct SysDictType {
	pub id: String,
	// 字典类型
	pub dict_type: String,
	pub order_num: i32,
	// 字典类型描述
	pub description: String,
	// 是否系统内置
	pub system_flag: bool,
	pub status: String,
	// 备注信息
	pub remark: String,
	pub created_at: DateTime<Utc>,
	pub create_by: String,
	pub updated_at: DateTime<Utc>,
	pub update_by: String,
	pub is_deleted: bool,
	pub deleted_at: DateTime<Utc>,
}

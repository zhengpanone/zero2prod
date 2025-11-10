use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

// 岗位信息
#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, ToSchema)]
pub struct Post {
	// 岗位ID
	pub id: Uuid,
	// 岗位名称
	pub name: String,
	// 排序值
	pub order_num: i32,
	// 岗位状态
	pub status: String,
	// 岗位备注
	pub remark: String,

	pub created_at: DateTime<Utc>,
	pub create_by: String,

	pub update_at: DateTime<Utc>,
	pub update_by: String,

	pub is_deleted: bool,
	pub deleted_at: DateTime<Utc>,
}

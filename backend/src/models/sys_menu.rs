use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

// 菜单管理
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct SysMenu {
	pub id: Uuid,
	pub name: String,
	pub parent_id: Uuid,
	pub order_num: i32,
	pub url: String,
	pub menu_type: String,
	pub visible: String,
	pub perms: String,
	pub icon: String,
	pub remark: String,
	// 创建人
	pub create_by: String,
	pub create_at: DateTime<Utc>,
	pub update_by: String,
	pub update_at: DateTime<Utc>,

	pub is_deleted: bool,
	pub deleted_at: DateTime<Utc>,
}

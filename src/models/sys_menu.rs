use chrono::{DateTime, Utc};
use uuid::Uuid;

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
	pub create_by: String,
	pub create_time: DateTime<Utc>,
	pub update_by: String,
	pub update_time: DateTime<Utc>,
	pub remark: String,
	pub is_deleted: bool,
	pub deleted_at: DateTime<Utc>,
}

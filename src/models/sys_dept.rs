use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct SysDept {
	pub id: Uuid,
	pub name: String,
	pub parent_id: Uuid,
	pub order_num: i32,
	pub leader: String,
	pub phone: String,
	pub email: String,
	pub status: String,
	pub created_at: DateTime<Utc>,
	pub updated_at: DateTime<Utc>,
}

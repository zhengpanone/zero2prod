use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct SysDictData {
	pub id: Uuid,
	pub dict_type: String,
	pub dict_label: String,
	pub dict_value: String,
	pub order_num: i32,
	pub status: String,
	pub created_at: DateTime<Utc>,
	pub create_by: String,
	pub updated_at: DateTime<Utc>,
	pub update_by: String,
	pub is_deleted: bool,
	pub deleted_at: DateTime<Utc>,
}

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Role {
	pub id: Uuid,
	pub name: String,
	pub description: String,
	pub created_at: DateTime<Utc>,
	pub updated_at: DateTime<Utc>,
}

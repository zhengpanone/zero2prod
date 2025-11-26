use chrono::{DateTime, Utc};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(FromRow, Debug, Clone)]
pub struct RefreshToken {
	pub jti: Uuid,
	pub user_id: Uuid,
	pub expires_at: DateTime<Utc>,
	pub revoked: bool,
	pub created_at: DateTime<Utc>,
}

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

/// 派生特性说明
/// Debug：用于调试打印
/// Clone：允许创建副本
/// Serialize/Deserialize：JSON 序列化支持
/// FromRow：自动将数据库行转换为 Rust 结构
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct User {
	pub id: Uuid,
	pub username: String,
	pub email: String,
	// pub role: String,
	#[serde(skip_serializing)]
	pub password_hash: String,
	// pub bio: String,
	// pub image: String,
	pub created_at: DateTime<Utc>,
	pub updated_at: DateTime<Utc>,
}

#[derive(FromRow, Debug, Clone)]
pub struct EmailVerification {
	pub user_id: Uuid,
	pub token: Uuid,
	pub expires_at: DateTime<Utc>,
	pub created_at: DateTime<Utc>,
}

#[derive(FromRow, Debug, Clone)]
pub struct PasswordReset {
	pub user_id: Uuid,
	pub token: Uuid,
	pub expires_at: DateTime<Utc>,
	pub created_at: DateTime<Utc>,
}

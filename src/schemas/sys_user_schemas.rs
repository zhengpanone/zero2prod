use crate::{enums::common::UserStatus, models::sys_user::SysUser};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, ToSchema)]
#[schema(
	title = "CreateUserRequest",
	description = "Request payload for creating a new user"
)]
pub struct CreateUserRequest {
	#[validate(length(
		min = 3,
		max = 50,
		message = "Username must be between 3 and 50 characters"
	))]
	#[schema(example = "john_doe", min_length = 3, max_length = 50)]
	pub username: String,

	#[validate(email(message = "Invalid email format"))]
	#[schema(example = "john_doe@example.com", format = "email")]
	pub email: String,
	#[validate(length(
		min = 8,
		message = "Password must be at least 8 characters"
	))]
	#[schema(example = "strongPassword123", min_length = 8, write_only = true)]
	pub password: String,
	// #[validate(length(max = 500, message = "Bio cannot exceed 500 characters"))]
	// pub bio: Option<String>,

	// #[validate(url(message = "Image must be a valid URL"))]
	// pub image: Option<String>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateUserRequest {
	#[validate(length(
		min = 3,
		max = 50,
		message = "Username must be between 3 and 50 characters"
	))]
	pub username: Option<String>,

	#[validate(email(message = "Invalid email format"))]
	pub email: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
#[schema(
	title = "UserResponse",
	description = "Response after successfully requesting  user api"
)]
pub struct UserResponse {
	#[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
	pub id: Uuid,

	/// 用户名
	#[schema(example = "john_doe", min_length = 3, max_length = 50)]
	pub username: String,

	/// 邮箱地址
	#[schema(example = "user@example.com", format = "email")]
	pub email: String,

	// 个人简介
	// #[schema(example = "A software developer from Beijing")]
	// pub bio: Option<String>,

	// 头像图片URL
	// #[schema(example = "https://example.com/avatar.jpg")]
	// pub image: Option<String>,
	/// 用户状态
	#[schema(example = "active", default = "active")]
	pub status: UserStatus,

	/// 账户创建时间 - ISO 8601 格式
	#[schema(example = "2024-01-01T00:00:00Z")]
	pub created_at: String,

	/// 最后更新时间 - ISO 8601 格式
	#[schema(example = "2024-01-01T00:00:00Z")]
	pub updated_at: String,
}

impl From<SysUser> for UserResponse {
	fn from(user: SysUser) -> Self {
		Self {
			id: user.id,
			username: user.username,
			email: user.email,
			status: user.status,
			// bio: Some(user.bio),
			// image: Some(user.image),
			created_at: user.created_at.map(|t| t.to_rfc3339()).unwrap_or_default(),
			updated_at: user.updated_at.map(|t| t.to_rfc3339()).unwrap_or_default(),
		}
	}
}

/// 用户查询条件
#[derive(Debug, Deserialize, Default)]
pub struct UserQueryRequest {
	pub username: Option<String>,
	pub email: Option<String>,
	pub phone: Option<String>,
	pub role_id: Option<i64>,
}

#[cfg(test)]
mod validation_user_tests {
	use validator::Validate;

	use crate::schemas::sys_user_schemas::CreateUserRequest;

	#[test]
	fn test_login_request_validation() {
		let request = CreateUserRequest {
			username: "test".to_string(),
			email: "example@email.com".to_string(),
			password: "<PASSWORD>".to_string(),
			// bio: None,
			// image: None,
		};
		assert!(
			request.validate().is_ok(),
			"CreateUserRequest should be valid"
		);
	}
}

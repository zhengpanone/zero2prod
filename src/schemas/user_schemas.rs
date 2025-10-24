use crate::enums::common::UserStatus;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateUserRequest {
	#[validate(length(
		min = 3,
		max = 50,
		message = "Username must be between 3 and 50 characters"
	))]
	pub username: String,

	#[validate(email(message = "Invalid email format"))]
	pub email: String,
	#[validate(length(
		min = 8,
		message = "Password must be at least 8 characters"
	))]
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
pub struct UserResponse {
	pub id: Uuid,
	pub username: String,
	pub email: String,
	// pub bio: Option<String>,
	// pub image: Option<String>,
	pub status: UserStatus,
	pub created_at: String,
	pub updated_at: String,
}

impl From<crate::models::sys_user::SysUser> for UserResponse {
	fn from(user: crate::models::sys_user::SysUser) -> Self {
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

#[cfg(test)]
mod validation_user_tests {
	use validator::Validate;

	use crate::schemas::user_schemas::CreateUserRequest;

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

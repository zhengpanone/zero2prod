use chrono::{DateTime, Utc};
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

	#[validate(length(max = 500, message = "Bio cannot exceed 500 characters"))]
	pub bio: Option<String>,

	#[validate(url(message = "Image must be a valid URL"))]
	pub image: Option<String>,
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
	pub bio: Option<String>,
	pub image: Option<String>,
	pub created_at: DateTime<Utc>,
	pub updated_at: DateTime<Utc>,
}

impl From<crate::models::user::User> for UserResponse {
	fn from(user: crate::models::user::User) -> Self {
		Self {
			id: user.id,
			username: user.username,
			email: user.email,
			bio: Some(user.bio),
			image: Some(user.image),
			created_at: user.created_at,
			updated_at: user.updated_at,
		}
	}
}

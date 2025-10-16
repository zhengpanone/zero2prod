use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct LoginRequest {
	#[validate(email(message = "Invalid email format"))]
	pub email: String,

	#[validate(length(
		min = 6,
		message = "Password must be at least 6 characters"
	))]
	pub password: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct RegisterRequest {
	#[validate(email(message = "Invalid email format"))]
	pub email: String,

	#[validate(length(
		min = 2,
		max = 100,
		message = "Name must be between 2 and 100 characters"
	))]
	pub name: String,

	#[validate(length(
		min = 6,
		message = "Password must be at least 6 characters"
	))]
	pub password: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AuthResponse {
	pub token: String,
	pub user_id: Uuid,
	pub email: String,
	pub name: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct LoginResponse {
	pub token: String,
}

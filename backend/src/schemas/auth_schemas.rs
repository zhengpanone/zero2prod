use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct LoginRequest {
	// #[validate(email(message = "Invalid email format"))]
	// pub email: String,
	#[validate(length(
		min = 2,
		max = 100,
		message = "Name must be between 2 and 100 characters"
	))]
	pub username: String,

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
	pub username: String,

	#[validate(length(
		min = 6,
		message = "Password must be at least 6 characters"
	))]
	pub password: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AuthResponse {
	pub token: String,
	pub user_id: String,
	pub email: String,
	pub username: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct LoginResponse {
	pub access_token: String,
	pub refresh_token: String,
	pub user_id: String,
	pub email: String,
	pub username: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct RefreshRequest {
	pub refresh_token: String,
}

#[cfg(test)]
mod validation_auth_tests {
	use validator::Validate;

	use crate::schemas::auth_schemas::{LoginRequest, RegisterRequest};

	#[test]
	fn test_login_request_validation() {
		let mut request = LoginRequest {
			// email: "example@email.com".to_string(),
			username: String::from("admin"),
			password: String::from("password123"),
		};
		assert!(request.validate().is_ok(), "LoginRequest should be valid");

		request.password = String::from("passw");
		assert!(
			request.validate().is_err(),
			"LoginRequest should be invalid"
		);
	}
	#[test]
	fn test_register_request_validation() {
		let request = RegisterRequest {
			email: "example@email.com".to_string(),
			username: String::from("username"),
			password: String::from("password123"),
		};
		assert!(
			request.validate().is_ok(),
			"RegisterRequest should be valid"
		);
	}
}

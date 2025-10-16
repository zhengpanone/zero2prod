use axum::{
	http::StatusCode,
	response::{IntoResponse, Response},
	Json,
};

use serde::{Deserialize, Serialize};
use tracing::error;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ApiError {
	pub code: String,
	pub message: String,
	pub details: Option<serde_json::Value>,
}

#[derive(thiserror::Error, Debug)]
pub enum AppError {
	#[error("database error: {0}")]
	Database(#[from] sqlx::Error),

	#[error("authentication error: {0}")]
	Auth(String),

	#[error("Not found: {0}")]
	NotFound(String),

	#[error("Bad request: {0}")]
	BadRequest(String),

	#[error("Validation error: {0}")]
	Validation(String),

	#[error("Internal server error: {0}")]
	Internal(#[from] anyhow::Error),
}

impl IntoResponse for AppError {
	fn into_response(self) -> Response {
		let (status, code, message) = match self {
			AppError::Database(ref e) => {
				error!("Database error: {:?}", e);
				(
					StatusCode::INTERNAL_SERVER_ERROR,
					"DATABASE_ERROR",
					"A database error occurred".to_string(),
				)
			}
			AppError::Auth(ref msg) => {
				error!("Authentication error: {:?}", msg);
				(StatusCode::UNAUTHORIZED, "AUTH_ERROR", msg.to_string())
			}
			AppError::NotFound(ref msg) => {
				error!("Not found: {:?}", msg);
				(StatusCode::NOT_FOUND, "NOT_FOUND", msg.to_string())
			}
			AppError::BadRequest(ref e) => {
				error!("Bad request: {:?}", e);
				(StatusCode::BAD_REQUEST, "BAD_REQUEST", e.to_string())
			}
			AppError::Validation(ref msg) => {
				error!("Validation error: {:?}", msg);
				(
					StatusCode::UNPROCESSABLE_ENTITY,
					"VALIDATION_ERROR",
					msg.clone(),
				)
			}
			AppError::Internal(ref e) => {
				error!("Internal error: {:?}", e);
				(
					StatusCode::INTERNAL_SERVER_ERROR,
					"INTERNAL_ERROR",
					"A internal error occurred".to_string(),
				)
			}
		};

		let body = Json(ApiError {
			code: code.to_string(),
			message: message.to_string(),
			details: None,
		});
		(status, body).into_response()
	}
}

pub type Result<T> = std::result::Result<T, AppError>;

use axum::response::IntoResponse;
use http::StatusCode;
use jwt::AuthError;

pub mod jwt;
pub mod user;
pub mod counter;

pub enum ApiError {
    Auth(AuthError),
    Internal(anyhow::Error),
}

impl<E> From<E> for ApiError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self::Internal(err.into())
    }
}
impl From<AuthError> for ApiError {
    fn from(e: AuthError) -> Self {
        ApiError::Auth(e)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}

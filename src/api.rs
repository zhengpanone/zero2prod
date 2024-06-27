use axum::response::IntoResponse;
use http::StatusCode;
use jwt::AuthError;

pub mod counter;
pub mod counter_record;
pub mod jwt;
pub mod user;

pub enum ApiError {
    NotFound,
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
        match self {
            ApiError::NotFound => StatusCode::NOT_FOUND.into_response(),
            ApiError::Auth(err) => err.into_response(),
            ApiError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}

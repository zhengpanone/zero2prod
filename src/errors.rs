use axum::Json;
use axum::response::{IntoResponse, Response};
use bcrypt::BcryptError;
use argon2::password_hash::Error as Argon2Error;
use http::StatusCode;
use serde_json::json;
use tokio::task::JoinError;

// https://github.com/ndelvalle/rustapi/blob/master/Cargo.toml
#[derive(thiserror::Error, Debug)]
#[error("...")]
pub enum Error {
    #[error("{0}")]
    Authenticate(#[from] AuthenticateError),

    #[error("{0}")]
    BadRequest(#[from] BadRequest),

    #[error("{0}")]
    NotFound(#[from] NotFound),

    #[error("{0}")]
    RunSyncTask(#[from] JoinError),

    #[error("{0}")]
    HashPassword(#[source] HashPasswordError),
}


#[derive(Debug)]
pub enum HashPasswordError {
    Bcrypt(BcryptError),
    Argon2(Argon2Error),
}

impl std::fmt::Display for HashPasswordError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HashPasswordError::Bcrypt(err) => write!(f, "Bcrypt error: {}", err),
            HashPasswordError::Argon2(err) => write!(f, "Argon2 error: {}", err),
        }
    }
}


impl std::error::Error for HashPasswordError {}

impl Error {
    fn get_codes(&self) -> (StatusCode, u16) {
        match *self {
            // 4XX Errors
            Error::BadRequest(_) => (StatusCode::BAD_REQUEST, 40002),
            Error::NotFound(_) => (StatusCode::NOT_FOUND, 4003),
            Error::Authenticate(AuthenticateError::WrongCredentials) => {
                (StatusCode::UNAUTHORIZED, 4004)
            }
            Error::Authenticate(AuthenticateError::InvalidToken) => {
                (StatusCode::UNAUTHORIZED, 4005)
            }
            Error::Authenticate(AuthenticateError::Locked) => {
                (StatusCode::LOCKED, 4006)
            }

            // 5XX Errors
            Error::Authenticate(AuthenticateError::TokenCreation) => {
                (StatusCode::INTERNAL_SERVER_ERROR, 5001)
            }
            Error::RunSyncTask(_) => (StatusCode::INTERNAL_SERVER_ERROR, 5005),
            Error::HashPassword(_) => (StatusCode::INTERNAL_SERVER_ERROR, 5006),
        }
    }

    pub fn bad_request() -> Self {
        Error::BadRequest(BadRequest {})
    }

    pub fn not_found() -> Self {
        Error::NotFound(NotFound {})
    }
}


impl From<BcryptError> for HashPasswordError {
    fn from(err: BcryptError) -> Self {
        HashPasswordError::Bcrypt(err.into())
    }
}

impl From<Argon2Error> for HashPasswordError {
    fn from(err: Argon2Error) -> Self {
        HashPasswordError::Argon2(err.into())
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (status_code, code) = self.get_codes();
        let message = self.to_string();
        let body = Json(json!({"code":code, "message": message}));
        (status_code, body).into_response()
    }
}

#[derive(thiserror::Error, Debug)]
#[error("...")]
pub enum AuthenticateError {
    #[error("Wrong authentication credentials")]
    WrongCredentials,
    #[error("Failed to create authentication token")]
    TokenCreation,
    #[error("Invalid authentication credentials")]
    InvalidToken,
    #[error("User is locked")]
    Locked,
}

#[derive(thiserror::Error, Debug)]
#[error("Bad Request")]
pub struct BadRequest {}

#[derive(thiserror::Error, Debug)]
#[error("Not found")]
pub struct NotFound {}
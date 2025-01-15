use axum::Json;
use axum::response::{IntoResponse, Response};
use bcrypt::BcryptError;
use argon2::password_hash::Error as Argon2Error;
use http::StatusCode;
use serde_json::json;
use tokio::task::JoinError;

mod error_codes {
    pub const BAD_REQUEST_CODE: u16 = 40002;
    pub const NOT_FOUND_CODE: u16 = 4003;
    pub const WRONG_CREDENTIALS_CODE: u16 = 4004;
    pub const INTERNAL_ERROR_CODE: u16 = 5000;
    pub const TOKEN_CREATION_ERROR_CODE: u16 = 5001;
    pub const SYNC_TASK_FAILED_CODE: u16 = 5005;
    pub const BCRYPT_HASHING_FAILED_CODE: u16 = 5006;
    pub const ARGON2_HASHING_FAILED_CODE: u16 = 5007;
}

/**
 * 认证相关的错误类型
 */
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

// 密码哈希相关的错误类型
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

/**
 * 主错误类型
 */
// https://github.com/ndelvalle/rustapi/blob/master/Cargo.toml
#[derive(thiserror::Error, Debug)]
// #[non_exhaustive] 属性来确保添加新的错误时不会破坏现有的模式匹配
#[non_exhaustive]
pub enum Error {
    #[error("Authentication error: {0}")]
    Authenticate(#[from] AuthenticateError),

    #[error("Bad request: {0}")]
    BadRequest(#[from] BadRequest),

    #[error("Resource not found: {0}")]
    NotFound(#[from] NotFound),

    #[error("Failed to run blocking task: {0}")]
    RunSyncTask(#[from] JoinError),

    #[error("Password hashing error: {source}")]
    HashPassword {
        #[from]
        source: HashPasswordError,
    },
}

// 额外的错误类型
#[derive(thiserror::Error, Debug)]
#[error("Bad Request")]
pub struct BadRequest {}


#[derive(thiserror::Error, Debug)]
#[error("Not found")]
pub struct NotFound {}

impl Error {
    fn get_codes(&self) -> (StatusCode, u16, &'static str) {
        use error_codes::*;
        match *self {
            // 4XX Errors
            Error::BadRequest(_) => (StatusCode::BAD_REQUEST, BAD_REQUEST_CODE, "Invalid request data"),
            Error::NotFound(_) => (StatusCode::NOT_FOUND, NOT_FOUND_CODE, "Resource Not found"),
            Error::Authenticate(ref err) => self.get_authenticate_codes(err),

            Error::RunSyncTask(_) => (StatusCode::INTERNAL_SERVER_ERROR, SYNC_TASK_FAILED_CODE, "Sync Task Failed"),
            Error::HashPassword { ref source } => self.get_hash_password_codes(source),
        }
    }

    fn generate_details(&self) -> serde_json::Value {
        match &self {
            Error::HashPassword { source } => {
                // 解构 HashPasswordError 枚举
                match source {
                    HashPasswordError::Bcrypt(err) => json!({
                        "algorithm": "bcrypt", // 可根据实际情况设置
                        "error": self.to_string()
                    }),
                    HashPasswordError::Argon2(err) => json!({
                        "algorithm": "argon2",
                        "error": err.to_string()
                    }),
                }
            }
            Error::Authenticate(AuthenticateError::WrongCredentials) => json!({
                "message": "提供的凭证不正确。",
                "hint": "检查用户名和密码是否正确。"
            }),
            Error::Authenticate(AuthenticateError::InvalidToken) => json!({
                "message": "提供的令牌无效。",
                "hint": "确保令牌没有过期。"
            }),
            Error::Authenticate(AuthenticateError::Locked) => json!({
                "message": "账户因多次登录失败被锁定。",
                "hint": "请联系支持解锁账户。"
            }),
            _ => json!({}),
        }
    }


    fn get_authenticate_codes(&self, error: &AuthenticateError) -> (StatusCode, u16, &'static str) {
        use AuthenticateError::*;
        match error {
            WrongCredentials => (StatusCode::UNAUTHORIZED, error_codes::WRONG_CREDENTIALS_CODE, "Wrong credentials provided"),
            InvalidToken => (StatusCode::UNAUTHORIZED, error_codes::WRONG_CREDENTIALS_CODE, "Wrong Token provided"),
            Locked => (StatusCode::LOCKED, error_codes::WRONG_CREDENTIALS_CODE, "Account Locked"),
            TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, error_codes::TOKEN_CREATION_ERROR_CODE, "Server Internal")
        }
    }

    fn get_hash_password_codes(&self, source: &HashPasswordError) -> (StatusCode, u16, &'static str) {
        match source {
            HashPasswordError::Bcrypt(_) => (StatusCode::INTERNAL_SERVER_ERROR, error_codes::BCRYPT_HASHING_FAILED_CODE, "Bcrypt Hashing Failed"),
            HashPasswordError::Argon2(_) => (StatusCode::INTERNAL_SERVER_ERROR, error_codes::ARGON2_HASHING_FAILED_CODE, "Argon2 Hashing Failed"),
        }
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
        let (status_code, code, message) = self.get_codes();
        // 为错误添加详细信息
        let details = self.generate_details();
        let body = Json(json!({"code":code, "message": message, "details":details}));
        (status_code, body).into_response()
    }
}

#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::response::IntoResponse;
    use http::StatusCode;
    use serde_json::json;
    use crate::errors::{BadRequest, Error};


    // 辅助函数：将 Body 转换为字节数组
    async fn body_to_bytes(body: Body) -> Vec<u8> {
        let mut bytes = Vec::new();
        let mut stream = body;
        while let Some(chunk) = stream.next().await {
            bytes.extend_from_slice(&chunk.unwrap());
        }
        bytes
    }

    #[tokio::test]
    async fn test_bad_request_error() {
        // 模拟一个BadRequest错误
        let error = Error::BadRequest(BadRequest {});
        // 将错误转换为响应
        let response = error.into_response();
        let body_bytes = body_to_bytes(response.into_body()).await;

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        let expected = json!({
            "code": 40002,
            "message": "Invalid request data",
            "detail": {}
        });
        let actual = serde_json::from_slice(&body_bytes).unwrap();
        assert_eq!(actual, expected);
    }
}

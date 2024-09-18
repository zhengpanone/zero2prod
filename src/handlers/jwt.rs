use axum::{
    async_trait,
    extract::FromRequestParts,
    response::{IntoResponse, Response},
    Json, RequestPartsExt,
};
use http::{request::Parts, StatusCode};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use serde_json::json;

use std::time::{Duration, SystemTime, UNIX_EPOCH};

use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    exp: usize,
}
pub struct Uid(pub i32);

impl Claims {
    pub fn new(sub: String) -> Self {
        let exp = SystemTime::now() + Duration::from_secs(15 * 24 * 60 * 60);
        let exp = exp.duration_since(UNIX_EPOCH).unwrap().as_secs() as usize;
        Claims { sub, exp }
    }
}
#[derive(Debug)]
pub enum AuthError {
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
        };
        let body = Json(json!({
          "error": error_message,
        }));
        (status, body).into_response()
    }
}
// JWT_SECRET=secret cargo run -p example-jwt
#[async_trait]
impl<S> FromRequestParts<S> for Uid
where
    S: Send + Sync,
{
    type Rejection = AuthError;
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract the token from the authentication header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::InvalidToken)?;
        // Decode the token
        let token_data = decode::<Claims>(
            bearer.token(),
            &DecodingKey::from_secret(b"secret"),
            &Validation::default(),
        )
        .map_err(|_| AuthError::InvalidToken)?;
      let user_id = token_data.claims.sub.parse::<i32>().unwrap();
        Ok(Uid(user_id))
    }
}

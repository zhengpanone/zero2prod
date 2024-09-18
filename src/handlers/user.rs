use std::sync::Arc;

use axum::{extract::State, Json};
use http::StatusCode;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

use crate::{AppState, db::UserDO, handlers::jwt::Claims};
use crate::handlers::handlers::ApiError;

use super::jwt::AuthError;

#[derive(Deserialize)]
pub struct LoginPayload {
    code: String,
}
#[derive(Serialize)]
pub struct AuthBody {
    access_token: String,
    token_type: String,
}

impl AuthBody {
    fn new(access_token: String) -> Self {
        Self {
            access_token,
            token_type: "Bearer".to_string(),
        }
    }
}

pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<LoginPayload>,
) -> Result<Json<AuthBody>, ApiError> {
    let pool = &state.db_pool;
    let wx_user = wx_login(payload.code).await?;

    let user = sqlx::query_as::<_, UserDO>("select * from users where openid = ?")
        .bind(&wx_user.openid)
        .fetch_one(pool)
        .await;

    let user = match user {
        Ok(user) => user,
        Err(sqlx::Error::RowNotFound) => {
            sqlx::query("insert into users(openid,session_key) values(?, ?)")
                .bind(&wx_user.openid)
                .bind(&wx_user.session_key)
                .execute(pool)
                .await?;

            sqlx::query_as::<_, UserDO>("select * from users where openid = ?")
                .bind(&wx_user.openid)
                .fetch_one(pool)
                .await?
        }
        Err(e) => return Err(ApiError::from(e)),
    };
    let claims = Claims::new(user.id.to_string());
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(b"secret"),
    )
    .map_err(|_| AuthError::TokenCreation)?;
    let rsp = AuthBody::new(token);

    Ok(Json(rsp))
    // todo!()
}


#[derive(Deserialize, Default)]
pub struct WxUser {
    pub openid: String,
    pub session_key: String,
}
// TODO
pub async fn wx_login(code: String) -> Result<WxUser, ApiError> {
    
    Ok(WxUser::default())
}

pub async fn create_user(Json(payload): Json<CreateUser>) -> (StatusCode, Json<UserVO>) {
    let user = UserVO {
        id: 1337,
        username: payload.username,
    };
    (StatusCode::CREATED, Json(user))
}

#[derive(Deserialize)]
pub struct CreateUser {
    username: String,
}

#[derive(Serialize)]
pub struct UserVO {
    id: u64,
    username: String,
}

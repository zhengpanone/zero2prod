use axum::{extract::State, Json};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite};

use crate::{api::jwt::Claims, db::User};

use super::{jwt::AuthError, ApiError};

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
    State(pool): State<Pool<Sqlite>>,
    Json(payload): Json<LoginPayload>,
) -> Result<Json<AuthBody>, ApiError> {
    let wx_user = wx_login(payload.code).await?;

    let user = sqlx::query_as::<_, User>("select * from users where openid = ?")
        .bind(&wx_user.openid)
        .fetch_one(&pool)
        .await;

    let user = match user {
        Ok(user) => user,
        Err(sqlx::Error::RowNotFound) => {
            sqlx::query("insert into users(openid,session_key) values(?, ?)")
                .bind(&wx_user.openid)
                .bind(&wx_user.session_key)
                .execute(&pool)
                .await?;

            sqlx::query_as::<_, User>("select * from users where openid = ?")
                .bind(&wx_user.openid)
                .fetch_one(&pool)
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
async fn wx_login(code: String) -> Result<WxUser, ApiError> {
    Ok(WxUser::default())
}

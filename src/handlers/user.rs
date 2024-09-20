use std::sync::Arc;

use axum::{extract::State, Json};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

use crate::{AppState,  handlers::jwt::Claims};
use crate::db::user::{get_user_by_openid, insert_user};
use crate::errors::Error;
use crate::handlers::handlers::ApiError;
use crate::utils::custom_response::CustomResponse;

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

   let user =  get_user_by_openid(&pool,&wx_user.openid).await;
    let user = match user {
        Ok(user) => user,
        Err(sqlx::Error::RowNotFound) => {
            insert_user(&pool, &wx_user).await.expect("新增数据失败");
            get_user_by_openid(&pool,&wx_user.openid).await?
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

pub async fn create_user(Json(payload): Json<CreateUser>) -> Result<CustomResponse<UserVO>,Error> {
    // let user = UserVO {
    //     id: 1337,
    //     username: payload.username,
    // };
    //
    // (StatusCode::CREATED, Json(user))
    // Ok(res)
    todo!()

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

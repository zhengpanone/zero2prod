use std::sync::Arc;

use axum::{extract::State, Json};

use http::StatusCode;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

use crate::db::base::insert_selective;
use crate::{handlers::jwt::Claims, utils, AppState};

use crate::db::user::{get_user_by_openid, insert_user_wx_user};
use crate::handlers::handlers::ApiError;
use crate::models::user::UserDO;
use crate::utils::custom_response::ApiResponse;

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

    let user = get_user_by_openid(&pool, &wx_user.openid).await;
    let user = match user {
        Ok(user) => user,
        Err(sqlx::Error::RowNotFound) => {
            insert_user_wx_user(&pool, &wx_user)
                .await
                .expect("新增数据失败");
            get_user_by_openid(&pool, &wx_user.openid).await?
        }
        Err(e) => return Err(ApiError::from(e)),
    };
    let claims = Claims::new(user.id.unwrap().to_string());
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

pub async fn create_user(
    State(state): State<Arc<AppState>>,
    Json(body): Json<CreateUser>,
) -> Result<ApiResponse<()>, ApiError> {
    let pool = &state.db_pool;
    let password_hash =
        utils::encrypt::hash_password(body.password, utils::encrypt::HashAlgorithm::Bcrypt).await?;
    let user = UserDO::new(body.username, body.email, password_hash);
    insert_selective(pool, user)
        .await
        .expect("Failed to insert user");

    let response = ApiResponse::<()>::new(
        None,
        Some("User created successfully".into()),
        StatusCode::CREATED,
    );
    Ok(response)
}

#[derive(Deserialize)]
pub struct CreateUser {
    username: String,
    email: String,
    password: String,
}

#[derive(Serialize)]
pub struct UserVO {
    id: u64,
    username: String,
}

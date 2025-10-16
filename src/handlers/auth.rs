use axum::{extract::State, Json};
use utoipa::OpenApi;
use validator::Validate;

use crate::{
	error::{ApiError, AppError, Result},
	repositories::user_repository::UserRepository,
	schemas::auth_schemas::{AuthResponse, LoginRequest, RegisterRequest},
	state::AppState,
	utils::encrypt::verify_password,
};

/// 用户登录
#[utoipa::path(
    post,
    path = "/auth/login",
    tag = "Auth API",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "登录成功", body = AuthResponse),
        (status = 401, description = "认证失败", body = ApiError),
    )
)]
pub async fn login(
	State(state): State<AppState>,
	Json(req): Json<LoginRequest>,
) -> Result<Json<AuthResponse>> {
	req.validate()
		.map_err(|e| AppError::Validation(e.to_string()))?;
	let repository = UserRepository::new(state.db.clone());
	let user = repository.find_by_email(&req.email).await?.ok_or_else(|| {
		AppError::BadRequest("Invalid email or password".to_string())
	})?;

	let is_valid = verify_password(&req.password, &user.password_hash)?;
	if is_valid {
		return Err(AppError::BadRequest(
			"Invalid email or password".to_string(),
		));
	}

	// let token =

	todo!()
}

/// 用户注册
#[utoipa::path(
    post,
    path = "/auth/register",
    tag = "Auth API",
    request_body = RegisterRequest,
    responses(
        (status = 201, description = "注册成功", body = AuthResponse),
        (status = 400, description = "请求参数错误", body = ApiError),
    )
)]
pub async fn register(
	State(state): State<AppState>,
	Json(req): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>> {
	todo!()
}

#[derive(OpenApi)]
#[openapi(paths(register,login), tags((name = "Auth API", description = "Auth items")))]
pub struct AuthApiDoc;

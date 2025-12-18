use std::sync::Arc;

use crate::{
	errors::{ApiError, Result},
	schemas::sys_auth_schemas::{
		AuthResponse, LoginRequest, LoginResponse, RefreshRequest, RegisterRequest,
	},
	services::sys_auth_service::AuthService,
	state::AppState,
	utils::response::ApiResponse,
};
use axum::{extract::State, http::StatusCode, Json};
use utoipa::OpenApi;

/// 用户登录
#[utoipa::path(
    post,
    path = "/login",
    tag = "Auth API",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "登录成功", body = ApiResponse<LoginResponse>),
        (status = 401, description = "认证失败", body = ApiError),
    )
)]
pub async fn login(
	State(state): State<Arc<AppState>>,
	Json(req): Json<LoginRequest>,
) -> Result<(StatusCode, Json<ApiResponse<LoginResponse>>)> {
	let auth_service = AuthService::new(state.clone());
	let data = auth_service.login(req).await?;
	Ok(ApiResponse::ok_with_data(data))
}

/// 用户注册
#[utoipa::path(
    post,
    path = "/register",
    tag = "Auth API",
    request_body = RegisterRequest,
    responses(
        (status = 201, description = "注册成功", body = AuthResponse),
        (status = 400, description = "请求参数错误", body = ApiError),
    )
)]
pub async fn register(
	State(state): State<Arc<AppState>>,
	Json(req): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<ApiResponse<AuthResponse>>)> {
	let auth_service = AuthService::new(state.clone());
	let data = auth_service.register(req).await?;

	Ok(ApiResponse::ok_with_code_data(StatusCode::CREATED, data))
}

/// 用户退出
#[utoipa::path(
    post,
    path = "/logout",
    tag = "Auth API",
    request_body = RefreshRequest,
    responses(
        (status = 201, description = "注销成功", body = AuthResponse),
        (status = 400, description = "请求参数错误", body = ApiError),
    )
)]
pub async fn logout(
	State(state): State<Arc<AppState>>,
	Json(req): Json<RefreshRequest>,
) -> Result<(StatusCode, Json<ApiResponse<()>>)> {
	let auth_service = AuthService::new(state.clone());
	let data = auth_service.logout(req).await?;
	Ok(ApiResponse::message(data))
}

#[derive(OpenApi)]
#[openapi(paths(register,login), tags((name = "Auth API", description = "Auth items")))]
pub struct AuthApiDoc;

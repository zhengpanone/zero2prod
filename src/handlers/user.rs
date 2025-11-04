use std::sync::Arc;

use crate::{
	errors::{ApiError, Result},
	models::sys_user::SysUser,
	schemas::{
		common_schemas::IdsRequest,
		user_schemas::{CreateUserRequest, UpdateUserRequest, UserResponse},
	},
	services::user_service::UserService,
	state::AppState,
	utils::response::{ApiResponse, Page},
};
use axum::{
	extract::{Path, State},
	http::StatusCode,
	Json,
};
use utoipa::OpenApi;
use uuid::Uuid;

const TAG_NAME: &str = "User API";

/// 创建用户
#[utoipa::path(
    post,
    path = "/create",
    tag = TAG_NAME,
    request_body=CreateUserRequest,
responses(
    (status=201,description="创建用户",body=ApiResponse<UserResponse>),
    (status=400,description="请求参数错误",body=ApiError),
    (status=422,description="验证失败",body=ApiError),
    (status=500,description="服务器错误",body=ApiError))
)]
pub async fn create_user(
	State(state): State<Arc<AppState>>,
	Json(req): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<ApiResponse<UserResponse>>)> {
	let user_service = UserService::new(state.clone());
	let user = user_service.create_user(req).await?;
	let user_response = UserResponse::from(user);
	Ok(ApiResponse::created(user_response))
}

/// 删除用户 （204 无内容）
#[utoipa::path(
    delete,
    path = "/delete",
    tag = TAG_NAME,
    request_body=IdsRequest,
    responses(
        (status = 204, description = "用户删除成功"),
        (status = 404, description = "用户不存在", body = ApiError),
        (status = 500, description = "内部服务器错误", body = ApiError)
    )
)]
pub async fn delete_user(
	State(state): State<Arc<AppState>>,
	Json(req): Json<IdsRequest>,
) -> Result<(StatusCode, Json<ApiResponse<String>>)> {
	let user_service = UserService::new(state.clone());
	user_service.delete_user(req).await?;
	Ok(ApiResponse::message("删除用户成功"))
}

/// 更新用户
#[utoipa::path(
    put,
    path = "/update/{id}",
    tag = TAG_NAME,
    request_body=UpdateUserRequest,
    params(
        ("id" = Uuid, Path, description = "用户ID")
    ),
    responses(
        (status = 204, description = "用户更新成功"),
        (status = 404, description = "用户不存在", body = ApiError),
        (status = 500, description = "内部服务器错误", body = ApiError)
    )
)]
pub async fn update_user(
	State(state): State<Arc<AppState>>,
	Path(id): Path<Uuid>,
	Json(req): Json<UpdateUserRequest>,
) -> Result<(StatusCode, Json<UserResponse>)> {
	let user_service = UserService::new(state.clone());
	let user = user_service.update_user(id, req).await?;
	let user_response = UserResponse::from(user);
	Ok((StatusCode::OK, Json(user_response)))
}

/// 获取用户列表
#[utoipa::path(
    get,
    path="/list",
    tag=TAG_NAME,
    responses((status=200,description="成功获取用户列表",body=[SysUser]),
    (status=500,description="服务器内部错误",body=ApiError))
)]
pub async fn list_users(
	State(state): State<Arc<AppState>>,
) -> Result<(StatusCode, Json<ApiResponse<Page<UserResponse>>>)> {
	let user_service = UserService::new(state.clone());
	let user_list = user_service.list_users().await?;

	let items = user_list.into_iter().map(Into::into).collect();
	let total = 0;
	let page_num = 1;
	let page_size = 20;
	let total_page = 0;
	Ok(ApiResponse::page(
		items, total, page_num, page_size, total_page,
	))
}

#[derive(OpenApi)]
#[openapi(
    paths(list_users, create_user,delete_user,update_user),
    tags((name = "User API", description = "User management"))
)]
pub struct UserApiDoc;

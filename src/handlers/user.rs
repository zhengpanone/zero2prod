use std::sync::Arc;

use crate::{
	errors::{ApiError, Result},
	models::user::User,
	schemas::{
		common_schemas::IdsRequest,
		user_schemas::{CreateUserRequest, UpdateUserRequest, UserResponse},
	},
	services::user_service::UserService,
	state::AppState,
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
    (status=201,description="创建用户",body=UserResponse),
    (status=400,description="请求参数错误",body=ApiError),
    (status=422,description="验证失败",body=ApiError),
    (status=500,description="服务器错误",body=ApiError))
)]
pub async fn create_user(
	State(state): State<AppState>,
	Json(req): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<UserResponse>)> {
	let state_clone = Arc::new(state.clone());
	let user_service = UserService::new(state_clone);

	user_service.get_user_detail(Uuid::new_v4()).await?;

	let user = user_service.create_user(req).await?;

	let user_response = UserResponse::from(user);
	Ok((StatusCode::CREATED, Json(user_response)))
}

/// 删除用户
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
	State(state): State<AppState>,
	Json(req): Json<IdsRequest>,
) -> Result<StatusCode> {
	let state_clone = Arc::new(state.clone());
	let user_service = UserService::new(state_clone);
	user_service.delete_user(req).await?;

	Ok(StatusCode::NO_CONTENT)
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
	State(state): State<AppState>,
	Path(id): Path<Uuid>,
	Json(req): Json<UpdateUserRequest>,
) -> Result<(StatusCode, Json<UserResponse>)> {
	let state_clone = Arc::new(state.clone());
	let user_service = UserService::new(state_clone);
	let user = user_service.update_user(id, req).await?;
	let user_response = UserResponse::from(user);
	Ok((StatusCode::OK, Json(user_response)))
}

/// 获取用户列表
#[utoipa::path(
    get,
    path="/list",
    tag=TAG_NAME,
    responses((status=200,description="成功获取用户列表",body=[User]),
    (status=500,description="服务器内部错误",body=ApiError))
)]
pub async fn list_users(State(state): State<AppState>) -> Result<Json<Vec<User>>> {
	let state_clone = Arc::new(state.clone());
	let user_service = UserService::new(state_clone);
	let user_list = user_service.list_users().await?;
	Ok(Json(user_list))
}

#[derive(OpenApi)]
#[openapi(
    paths(list_users, create_user,delete_user,update_user),
    tags((name = "User API", description = "User management"))
)]
pub struct UserApiDoc;

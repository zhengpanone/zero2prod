use axum::{extract::State, Extension, Json};
use std::sync::Arc;
use utoipa::OpenApi;

use crate::errors::ApiError;
use crate::schemas::sys_dict_schemas::SysDictTypeResponse;
use crate::schemas::{
	common_schemas::IdsRequest,
	sys_dict_schemas::{CreateDictTypeRequest, UpdateDictTypeRequest},
};
use crate::utils::response::ApiResponse;
use crate::{errors::Result, state::AppState};
use crate::middleware::auth::AuthUser;
use crate::services::sys_dict_service::SysDictService;

const TAG_NAME: &str = "Dict API";

/// 新增字典类型
#[utoipa::path(post, path = "/create", tag = TAG_NAME,
	request_body = CreateDictTypeRequest,
	responses(
	(status = 201,description="创建字段类型",body=ApiResponse<SysDictTypeResponse>),
	(status=400,description="请求参数错误",body=ApiError),
    (status=422,description="验证失败",body=ApiError),
    (status=500,description="服务器错误",body=ApiError))
)]
pub async fn create_dict_type(
	State(state): State<Arc<AppState>>,
	Json(req): Json<CreateDictTypeRequest>,
	Extension(auth_user): Extension<Arc<AuthUser>>,
) -> Result<Json<SysDictTypeResponse>> {
	let sys_dict_service = SysDictService::new(state.clone());
	let dict_type = sys_dict_service.create_dict_type(req,auth_user).await?;
	todo!()
}

/// 删除字典类型
#[utoipa::path(delete, path = "/delete", tag = TAG_NAME, request_body = IdsRequest)]
pub async fn delete_dict_type() {
	todo!()
}

/// 更新字典类型
#[utoipa::path(put, path = "/update", tag = TAG_NAME, request_body = UpdateDictTypeRequest)]
pub async fn update_dict_type() {
	todo!()
}

/// 获取字典类型列表
#[utoipa::path(get, path = "/list", tag = TAG_NAME)]
pub async fn list_dict_type() {
	todo!()
}

///字典管理
#[derive(OpenApi)]
#[openapi(
    paths(create_dict_type, delete_dict_type, update_dict_type, list_dict_type),
tags((name = "Dict API", description = "Dict management"))
)]
pub struct DictApiDoc;

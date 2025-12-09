use std::sync::Arc;

use axum::{
	extract::{Path, State},
	http::StatusCode,
	Json,
};
use utoipa::OpenApi;
use uuid::Uuid;

use crate::{
	errors::{ApiError, Result},
	schemas::{
		common_schemas::IdsRequest,
		sys_role_schemas::{CreateRoleRequest, RoleResponse, UpdateRoleRequest},
	},
	services::sys_role_service::SysRoleService,
	state::AppState,
	utils::response::ApiResponse,
};

const TAG_NAME: &str = "Role API";
/// 创建角色
#[utoipa::path(
    post,
    path = "/create",
    tag = TAG_NAME,
    request_body = CreateRoleRequest,
    responses(
        (status = 201, description = "创建角色", body = ApiResponse<RoleResponse>),
        (status = 400, description = "请求参数错误", body = ApiError),
        (status = 422, description = "验证失败", body = ApiError),
        (status = 500, description = "服务器错误", body = ApiError)
    )
)]
pub async fn create_role(
	State(state): State<Arc<AppState>>,
	Json(req): Json<CreateRoleRequest>,
) -> Result<(StatusCode, Json<ApiResponse<RoleResponse>>)> {
	let role_service = SysRoleService::new(state.clone());
	let role = role_service.create_role(req).await?;
	let role_response = RoleResponse::from(role);
	Ok(ApiResponse::ok_with_code_data(
		StatusCode::CREATED,
		role_response,
	))
}

/// 删除角色 （204 无内容）
#[utoipa::path(
    delete,
    path = "/delete",
    tag = TAG_NAME,
    request_body = IdsRequest,
    responses(
        (status = 204, description = "角色删除成功"),
        (status = 404, description = "角色不存在", body = ApiError),
        (status = 500, description = "内部服务器错误", body = ApiError)
    )
)]
pub async fn delete_role(
	State(state): State<Arc<AppState>>,
	Json(req): Json<IdsRequest>,
) {
	todo!()
}

/// 更新角色
#[utoipa::path(put,
    path = "/update",
    tag = TAG_NAME,
    request_body = UpdateRoleRequest,
    params(
        ("id"=Uuid,Path,  description = "角色ID", )
    ),
    responses(
        (status = 200, description = "角色更新成功", body = ApiResponse<RoleResponse>),
        (status = 400, description = "请求参数错误", body = ApiError),
        (status = 404, description = "角色不存在", body = ApiError),
        (status = 422, description = "验证失败", body = ApiError),
        (status = 500, description = "服务器错误", body = ApiError)
    )

)]
pub async fn update_role(
	State(state): State<Arc<AppState>>,
	Path(id): Path<Uuid>,
	Json(req): Json<UpdateRoleRequest>,
) {
	todo!()
}

/// 获取角色列表
#[utoipa::path(get, path = "/list", tag = TAG_NAME,
responses(
    (status = 200, description = "角色列表", body = ApiResponse<Vec<RoleResponse>>),
    (status = 500, description = "服务器错误", body = ApiError)
))]
pub async fn list_role(State(state): State<Arc<AppState>>) {
	todo!()
}

#[derive(OpenApi)]
#[openapi(
    paths(create_role, delete_role, update_role, list_role),

tags((name = "Role API", description = "Role management"))
)]
pub struct RoleApiDoc;

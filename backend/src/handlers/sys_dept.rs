use utoipa::OpenApi;

use crate::schemas::{
	common_schemas::IdsRequest,
	sys_dept_schemas::{CreateDeptRequest, UpdateDeptRequest},
};

const TAG_NAME: &str = "Dept API";

/// 新增部门
#[utoipa::path(post, path = "/create",
tag = TAG_NAME, request_body = CreateDeptRequest)]
pub async fn create_dept() {
	todo!()
}

/// 删除部门
#[utoipa::path(delete, path = "/delete",tag = TAG_NAME, request_body = IdsRequest)]
pub async fn delete_dept() {
	todo!()
}

/// 更新部门
#[utoipa::path(put, path = "/update", tag = TAG_NAME, request_body = UpdateDeptRequest)]
pub async fn update_dept() {
	todo!()
}

/// 获取部门列表
#[utoipa::path(get, path = "/list", tag = TAG_NAME)]
pub async fn list_dept() {
	todo!()
}

#[derive(OpenApi)]
#[openapi(
    paths(create_dept, delete_dept, update_dept, list_dept),
tags((name = "Dept API", description = "Dept management"))
)]
pub struct DeptApiDoc;

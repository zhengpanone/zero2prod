use utoipa::OpenApi;

use crate::schemas::{
	common_schemas::IdsRequest,
	sys_dict_schemas::{CreateDictTypeRequest, UpdateDictTypeRequest},
};

const TAG_NAME: &str = "Dict API";

/// 新增字典类型
#[utoipa::path(post, path = "/create", tag = TAG_NAME, request_body = CreateDictTypeRequest)]
pub async fn create_dict_type() {
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

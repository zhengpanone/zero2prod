use utoipa::OpenApi;

use crate::schemas::{
	common_schemas::IdsRequest,
	sys_menu_schemas::{CreateMenuRequest, UpdateMenuRequest},
};

const TAG_NAME: &str = "Menu API";
/// 新增菜单
#[utoipa::path(post, path = "/create", tag = TAG_NAME,
request_body = CreateMenuRequest)]
pub async fn create_menu() {
	todo!()
}

/// 删除菜单
#[utoipa::path(delete, path = "/delete", tag = TAG_NAME,
request_body = IdsRequest)]
pub async fn delete_menu() {
	todo!()
}

/// 更新菜单
#[utoipa::path(put, path = "/update", tag = TAG_NAME,request_body=UpdateMenuRequest)]
pub async fn update_menu() {
	todo!()
}
/// 获取菜单列表
#[utoipa::path(get, path = "/list", tag = TAG_NAME)]
pub async fn list_menu() {
	todo!()
}

#[derive(OpenApi)]
#[openapi(
    paths(create_menu, delete_menu, update_menu, list_menu),
tags((name = "Menu API", description = "Menu management"))
)]
pub struct MenuApiDoc;

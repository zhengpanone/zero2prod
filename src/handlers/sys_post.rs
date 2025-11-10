use utoipa::OpenApi;

use crate::schemas::{
	common_schemas::IdsRequest,
	sys_post_schemas::{CreatePostRequest, UpdatePostRequest},
};

const TAG_NAME: &str = "Post API";

/// 新增岗位
#[utoipa::path(
    post,
    path = "/create",
    tag = TAG_NAME,
    request_body = CreatePostRequest
)]
pub async fn create_post() {
	todo!()
}

/// 删除岗位
#[utoipa::path(
    delete,
    path = "/delete",
    tag = TAG_NAME,
    request_body = IdsRequest
)]
pub async fn delete_post() {
	todo!()
}

/// 更新岗位
#[utoipa::path(
    put,
    path = "/update",
    tag = TAG_NAME,
    request_body = UpdatePostRequest
)]
pub async fn update_post() {
	todo!()
}

/// 查询岗位列表
#[utoipa::path(
    get,
    path = "/list",
    tag = TAG_NAME
)]
pub async fn list_post() {
	todo!()
}

#[derive(OpenApi)]
#[openapi(
    paths(create_post, delete_post, update_post, list_post),
tags((name = "Post API", description = "Post management"))
)]
pub struct PostApiDoc;

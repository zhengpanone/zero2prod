use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, ToSchema)]
#[schema(title = "CreatePostRequest", description = "创建岗位请求体")]
pub struct CreatePostRequest {
	/// 岗位名称
	#[validate(length(min = 2, max = 30))]
	#[schema(example = "销售", min_length = 2, max_length = 30)]
	pub post_name: String,
	/// 岗位Key
	#[validate(length(min = 2, max = 100))]
	#[schema(example = "sale", min_length = 2, max_length = 100)]
	pub post_key: String,
	/// 排序
	#[validate(range(min = 0, max = 100))]
	#[schema(example = 1)]
	pub order_num: i32,
	/// 状态
	#[validate(length(min = 1, max = 100))]
	#[schema(example = "0")]
	pub status: String,
	/// 备注
	#[validate(length(min = 1, max = 100))]
	#[schema(example = "备注")]
	pub remark: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
#[schema(title = "UpdatePostRequest", description = "更新岗位请求体")]
pub struct UpdatePostRequest {
	/// 岗位名称
	#[validate(length(min = 2, max = 30))]
	#[schema(example = "销售", min_length = 2, max_length = 30)]
	pub post_name: String,
	/// 岗位Key
	#[validate(length(min = 2, max = 100))]
	#[schema(example = "sale", min_length = 2, max_length = 100)]
	pub post_key: String,
	/// 排序
	#[validate(range(min = 0, max = 100))]
	#[schema(example = 1)]
	pub order_num: i32,
	/// 状态
	#[validate(length(min = 1, max = 100))]
	#[schema(example = "0")]
	pub status: String,
	/// 备注
	#[validate(length(min = 1, max = 100))]
	#[schema(example = "备注")]
	pub remark: String,
}

#[derive(Debug, Serialize, Validate, ToSchema)]
#[schema(title = "PostResponse", description = "岗位响应体")]
pub struct PostResponse {
	/// 岗位名称
	#[validate(length(min = 2, max = 30))]
	#[schema(example = "销售", min_length = 2, max_length = 30)]
	pub post_name: String,
	/// 岗位Key
	#[validate(length(min = 2, max = 100))]
	#[schema(example = "sale", min_length = 2, max_length = 100)]
	pub post_key: String,
	/// 排序
	#[validate(range(min = 0, max = 100))]
	#[schema(example = 1)]
	pub order_num: i32,
	/// 备注
	#[validate(length(min = 1, max = 100))]
	#[schema(example = "备注")]
	pub remark: String,
}

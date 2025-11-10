use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, ToSchema)]
#[schema(title = "CreateRoleRequest", description = "创建角色请求体")]
pub struct CreateRoleRequest {
	/// 角色名称
	#[validate(length(min = 1, max = 30))]
	#[schema(example = "管理员", min_length = 1, max_length = 30)]
	pub role_name: String,
	/// 角色Key
	#[validate(length(min = 1, max = 100))]
	#[schema(example = "admin", min_length = 1, max_length = 100)]
	pub role_key: String,
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
pub struct UpdateRoleRequest {
	/// 角色名称
	#[validate(length(min = 1, max = 30))]
	#[schema(example = "管理员", min_length = 1, max_length = 30)]
	pub role_name: String,
	/// 角色Key
	#[validate(length(min = 1, max = 100))]
	#[schema(example = "admin", min_length = 1, max_length = 100)]
	pub role_key: String,
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
#[schema(title = "RoleResponse", description = "角色响应体")]
pub struct RoleResponse {
	/// 角色名称
	#[validate(length(min = 1, max = 30))]
	#[schema(example = "管理员", min_length = 1, max_length = 30)]
	pub role_name: String,
	/// 角色Key
	#[validate(length(min = 1, max = 100))]
	#[schema(example = "admin", min_length = 1, max_length = 100)]
	pub role_key: String,
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

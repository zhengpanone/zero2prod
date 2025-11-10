use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, ToSchema)]
#[schema(title = "CreateMenuRequest", description = "创建菜单请求体")]
pub struct CreateMenuRequest {
	/// 菜单名称
	#[validate(length(min = 1, max = 30))]
	#[schema(example = "系统管理", min_length = 1, max_length = 30)]
	pub menu_name: String,
	/// 父级菜单ID
	#[validate(length(min = 1, max = 100))]
	#[schema(example = "0", min_length = 1, max_length = 100)]
	pub parent_id: Option<String>,
	/// 排序
	#[validate(range(min = 0, max = 100))]
	#[schema(example = 1)]
	pub order_num: i32,
	/// 菜单路径
	#[validate(length(min = 1, max = 100))]
	#[schema(example = "system", min_length = 1, max_length = 100)]
	pub path: String,
	/// 组件路径
	#[validate(length(min = 1, max = 100))]
	#[schema(example = "system", min_length = 1, max_length = 100)]
	pub component: String,
	///
	pub is_frame: String,
	/// 是否缓存
	#[schema(example = true)]
	pub is_cache: bool,
	/// 菜单类型
	#[validate(length(min = 1, max = 100))]
	#[schema(example = "0")]
	pub menu_type: String,
	/// 是否可见
	#[validate(length(min = 1, max = 100))]
	#[schema(example = "0")]
	pub visible: String,
	/// 状态
	#[validate(length(min = 1, max = 100))]
	#[schema(example = "0")]
	pub status: String,
	/// 权限标识
	#[validate(length(min = 1, max = 100))]
	#[schema(example = "system:user:list")]
	pub perms: String,
	/// 图标
	#[validate(length(min = 1, max = 100))]
	#[schema(example = "system:user:list")]
	pub icon: String,
	/// 备注
	#[validate(length(min = 1, max = 100))]
	#[schema(example = "system:user:list")]
	pub remark: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
#[schema(title = "UpdateMenuRequest", description = "更新菜单请求体")]
pub struct UpdateMenuRequest {
	/// 菜单名称
	#[validate(length(min = 1, max = 30))]
	#[schema(example = "系统管理", min_length = 1, max_length = 30)]
	pub menu_name: String,
	/// 父级菜单ID
	#[validate(length(min = 1, max = 100))]
	#[schema(example = "0", min_length = 1, max_length = 100)]
	pub parent_id: Option<String>,
	/// 排序
	#[validate(range(min = 0, max = 100))]
	#[schema(example = 1)]
	pub order_num: i32,
	/// 菜单路径
	#[validate(length(min = 1, max = 100))]
	#[schema(example = "system", min_length = 1, max_length = 100)]
	pub path: String,
	/// 组件路径
	#[validate(length(min = 1, max = 100))]
	#[schema(example = "system", min_length = 1, max_length = 100)]
	pub component: String,
	///
	pub is_frame: String,
	/// 是否缓存
	#[schema(example = true)]
	pub is_cache: bool,
	/// 菜单类型
	#[validate(length(min = 1, max = 100))]
	#[schema(example = "0")]
	pub menu_type: String,
	/// 是否可见
	#[validate(length(min = 1, max = 100))]
	#[schema(example = "0")]
	pub visible: String,
	/// 状态
	#[validate(length(min = 1, max = 100))]
	#[schema(example = "0")]
	pub status: String,
	/// 权限标识
	#[validate(length(min = 1, max = 100))]
	#[schema(example = "system:user:list")]
	pub perms: String,
	/// 图标
	#[validate(length(min = 1, max = 100))]
	#[schema(example = "system:user:list")]
	pub icon: String,
	/// 备注
	#[validate(length(min = 1, max = 100))]
	#[schema(example = "system:user:list")]
	pub remark: String,
}

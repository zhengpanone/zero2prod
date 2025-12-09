use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use crate::{
	enums::common::RoleStatus,
	errors::{AppError, Result},
	models::sys_role::SysRole,
};

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
	pub role_code: String,
	#[schema(example = "admin role", min_length = 1, max_length = 100)]
	pub description: Option<String>,

	pub is_default: Option<String>,

	pub is_protected: Option<String>,
	/// 排序
	#[validate(range(min = 0, max = 100))]
	#[schema(example = 1)]
	pub order_num: Option<i32>,
	/// 状态
	#[validate(length(min = 1, max = 100))]
	#[schema(example = "0")]
	pub status: Option<String>,
	/// 备注
	#[validate(length(min = 1, max = 100))]
	#[schema(example = "备注")]
	pub remark: Option<String>,

	// 创建时间
	pub created_at: Option<DateTime<Utc>>,
	// 创建人
	pub created_by: Option<String>,
}

impl TryFrom<CreateRoleRequest> for SysRole {
	type Error = AppError;
	fn try_from(req: CreateRoleRequest) -> Result<Self> {
		let now = Utc::now();

		let status = RoleStatus::from_option_str(req.status.as_deref());

		let role = SysRole {
			id: Uuid::new_v4().to_string(),

			name: req.role_name,
			code: req.role_code,

			// CreateRoleRequest 没有 description 字段，设为空
			description: "".to_string(),
			status,
			order_num: req.order_num.unwrap_or(1),
			remark: req.remark.unwrap_or_default(),

			// 默认值字段
			is_default: false,
			is_protected: false,
			is_deleted: Some(false),

			// 时间相关
			created_at: now,
			updated_at: now,

			// 你后续可以从登录用户信息里替换这些
			created_by: "system".to_string(),
			updated_by: "system".to_string(),
			// 软删除通常用 MIN_UTC 或 now，但推荐 MIN_UTC
			deleted_at: None,
		};
		Ok(role)
	}
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
	pub role_code: String,
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

impl From<SysRole> for RoleResponse {
	fn from(role: SysRole) -> Self {
		Self {
			role_name: role.name,
			role_code: role.code,
			order_num: role.order_num,
			status: role.status.as_str().to_string(),
			remark: role.remark,
		}
	}
}

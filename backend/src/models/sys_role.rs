use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::enums::common::RoleStatus;

/// 内置角色说明：
///
/// - **admin**
///   - is_default: false
///   - is_protected: true
///   - 超级管理员，保护，绝对不能删
///
/// - **user**
///   - is_default: true
///   - is_protected: false
///   - 普通用户，默认给新用户
///
/// - **guest**
///   - is_default: true
///   - is_protected: false
///   - 游客角色
///
/// - **auditor**
///   - is_default: false
///   - is_protected: false
///   - 审计角色
///
/// - **service**
///   - is_default: false
///   - is_protected: true
///   - 系统内部服务角色
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct SysRole {
	// 角色ID
	pub id: String,
	// 角色名称
	pub name: String,

	pub code: String,

	pub description: String,

	// 角色状态
	#[sqlx(default)]
	pub status: RoleStatus,
	// 是否默认角色 是否用于自动分配、默认初始化角色，通常可删
	pub is_default: bool,
	// 是否保护角色 是否为系统核心角色，强保护，不允许删
	pub is_protected: bool,

	// 排序值
	pub order_num: i32,

	// 角色备注
	pub remark: String,

	// 创建时间
	pub created_at: DateTime<Utc>,
	// 创建人
	pub created_by: String,

	// 更新时间
	pub updated_at: DateTime<Utc>,
	// 更新人
	pub updated_by: String,

	// 是否删除
	#[sqlx(default)]
	pub is_deleted: Option<bool>,
	// 删除时间
	#[sqlx(default)]
	pub deleted_at: Option<DateTime<Utc>>,
}

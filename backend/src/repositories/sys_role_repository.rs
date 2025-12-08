use crate::{errors::Result, models::sys_role::SysRole};
use sqlx::{PgPool, Row};

#[derive(Debug)]
pub struct SysRoleRepository {
	pub pool: PgPool,
}

impl SysRoleRepository {
	pub fn new(pool: PgPool) -> Self {
		Self { pool }
	}

	/// 检查角色名称或代码是否已存在。
	///
	/// 返回：
	/// - `Ok(true)`  表示存在（名称或代码任一匹配）
	/// - `Ok(false)` 表示不存在
	///
	/// # Errors
	///
	/// 如果查询发生错误，会返回数据库错误。
	///
	/// # Examples
	///
	/// ```ignore
	/// let exists = repo.exists_by_code_name("admin", "ADMIN").await?;
	/// if exists {
	///     return Err(AppError::BadRequest(format!(
	///         "Role with name {} or code {} already exists",
	///         req.name,
	///         req.code,
	///     )));
	/// }
	/// ```
	pub async fn exists_by_code_name(
		&self,
		name: String,
		code: String,
	) -> Result<bool> {
		let row = sqlx::query_scalar::<_, Option<String>>(
			r#"
			SELECT id FROM sys_role WHERE name = $1 OR code = $2 LIMIT 1
			"#,
		)
		.bind(name)
		.bind(code)
		.fetch_one(&self.pool)
		.await?;
		Ok(true)
	}

	pub async fn find_page_with_count(
		&self,
		page_num: i64,
		page_size: i64,
	) -> Result<(Vec<SysRole>, i64)> {
		let offset = (page_num - 1) * page_size;
		// 使用 CTE 在一次查询中获取数据和总数
		let rows = sqlx::query(
			r#"
            WITH role_data AS (
                SELECT id, name, description, created_at, updated_at
                FROM sys_role
                ORDER BY created_at DESC
                LIMIT $1 OFFSET $2
            ),
            total_count AS (
                SELECT COUNT(*) as count FROM sys_role
            )
            SELECT
                role_data.*,
                total_count.count as total
            FROM role_data
            CROSS JOIN total_count
            "#,
		)
		.bind(page_size)
		.bind(offset)
		.fetch_all(&self.pool)
		.await?;

		if rows.is_empty() {
			// 如果没有数据，还需要查询总数
			let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM sys_role")
				.fetch_one(&self.pool)
				.await?;
			return Ok((vec![], total));
		}

		// 从第一行获取总数
		let total: i64 = rows[0].get("total");

		// 解析所有角色数据
		let users: Vec<SysRole> = rows
			.iter()
			.map(|row| SysRole {
				id: row.get("id"),
				name: row.get("name"),
				code: row.get("code"),
				description: row.get("description"),
				status: row.get("status"),
				order_num: row.get("order_num"),
				remark: row.get("remark"),
				is_default: row.get("is_default"),
				is_protected: row.get("is_protected"),
				created_at: row.get("created_at"),
				created_by: row.get("created_by"),
				updated_at: row.get("updated_at"),
				updated_by: row.get("updated_by"),
				is_deleted: row.get("is_deleted"),
				deleted_at: row.get("deleted_at"),
			})
			.collect();

		Ok((users, total))
	}
}

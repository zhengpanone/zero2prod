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
			let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM sys_user")
				.fetch_one(&self.pool)
				.await?;
			return Ok((vec![], total));
		}

		// 从第一行获取总数
		let total: i64 = rows[0].get("total");

		// 解析所有用户数据
		let users: Vec<SysRole> = rows
			.iter()
			.map(|row| SysRole {
				id: row.get("id"),
				name: row.get("name"),
				status: row.get("status"),
				remark: row.get("remark"),
				description: row.get("description"),
				created_at: row.get("created_at"),
				created_by: row.get("create_by"),
				updated_at: row.get("updated_at"),
				updated_by: row.get("update_by"),
				is_deleted: row.get("is_deleted"),
				deleted_at: row.get("deleted_at"),
			})
			.collect();

		Ok((users, total))
	}
}

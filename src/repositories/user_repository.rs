use sqlx::PgPool;
use uuid::Uuid;

use crate::{error::Result, models::user::User};

pub struct UserRepository {
	pool: PgPool,
}

impl UserRepository {
	pub fn new(pool: PgPool) -> Self {
		Self { pool }
	}

	pub async fn find_all(&self) -> Result<Vec<User>> {
		let users = sqlx::query_as::<_, User>(
            "SELECT id, email, name, role, password_hash, created_at, updated_at FROM users ORDER BY created_at DESC"
        )
        .fetch_all(&self.pool)
        .await?;

		Ok(users)
	}

	pub async fn find_by_id(&self, id: Uuid) -> Result<Option<User>> {
		let user = sqlx::query_as::<_, User>(
            "SELECT id, email, name, created_at, updated_at FROM users WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

		Ok(user)
	}

	pub async fn find_by_email(&self, email: &str) -> Result<Option<User>> {
		let user = sqlx::query_as::<_,User>("SELECT id, email, name, role, password_hash, created_at, updated_at FROM users WHERE email = $1").bind(email)
		.fetch_optional(&self.pool).await?;
		Ok(user)
	}

	pub async fn create(&self, email: &str, name: &str) -> Result<User> {
		let user = sqlx::query_as::<_, User>(
            "INSERT INTO users (email, name) VALUES ($1, $2) RETURNING id, email, name, created_at, updated_at"
        )
        .bind(email)
        .bind(name)
        .fetch_one(&self.pool)
        .await?;

		Ok(user)
	}

	pub async fn update(
		&self,
		id: Uuid,
		email: Option<&str>,
		name: Option<&str>,
	) -> Result<User> {
		let user = sqlx::query_as::<_, User>(
			r#"
            UPDATE users
            SET
                email = COALESCE($2, email),
                name = COALESCE($3, name),
                updated_at = NOW()
            WHERE id = $1
            RETURNING id, email, name, created_at, updated_at
            "#,
		)
		.bind(id)
		.bind(email)
		.bind(name)
		.fetch_one(&self.pool)
		.await?;

		Ok(user)
	}

	pub async fn delete(&self, ids: &[Uuid]) -> Result<bool> {
		if ids.is_empty() {
			return Ok(false);
		}
		// // 构建动态参数占位符，例如 $1, $2, ...
		// let placeholders = (1..=ids.len())
		// 	.map(|i| format!("${}", i))
		// 	.collect::<Vec<_>>()
		// 	.join(", ");
		// let query = format!("DELETE FROM users WHERE id in ({})", placeholders);
		// let mut q = sqlx::query(&query);
		// for id in ids {
		// 	q = q.bind(id);
		// }
		// let result = q.execute(&self.pool).await?;

		// PostgreSQL 支持直接传切片
		let result = sqlx::query!("DELETE FROM users WHERE id = ANY($1)", ids)
			.execute(&self.pool)
			.await?;

		Ok(result.rows_affected() > 0)
	}

	pub async fn exists_by_email(&self, email: &str) -> Result<bool> {
		let exists: bool = sqlx::query_scalar(
			"SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)",
		)
		.bind(email)
		.fetch_one(&self.pool)
		.await?;

		Ok(exists)
	}
}

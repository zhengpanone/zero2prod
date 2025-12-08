use crate::{errors::Result, models::sys_user::SysUser};
use sqlx::PgPool;

#[derive(Debug)]
pub struct UserRepository {
	pub pool: PgPool,
}

impl UserRepository {
	pub fn new(pool: PgPool) -> Self {
		Self { pool }
	}

	pub async fn find_all(&self) -> Result<Vec<SysUser>> {
		let users: Vec<SysUser> = sqlx::query_as::<_, SysUser>(
            "SELECT id, email, username, password_hash, status, created_at, updated_at FROM sys_user ORDER BY created_at DESC"
        )
        .fetch_all(&self.pool)
        .await?;

		Ok(users)
	}
	/// 分页查询用户列表
	pub async fn find_page(
		&self,
		page_num: i64,
		page_size: i64,
	) -> Result<Vec<SysUser>> {
		let offset = (page_num - 1) * page_size;
		let users: Vec<SysUser> = sqlx::query_as::<_, SysUser>(
			r#"
			SELECT id, email, username, password_hash, status, created_at, updated_at
			FROM sys_user ORDER BY created_at DESC
			LIMIT $1 OFFSET $2
			"#,
		)
		.bind(page_size)
		.bind(offset)
		.fetch_all(&self.pool)
		.await?;

		Ok(users)
	}

	/// 查询用户总数
	pub async fn count(&self) -> Result<i64> {
		let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM sys_user")
			.fetch_one(&self.pool)
			.await?;

		Ok(count)
	}

	pub async fn find_by_id(&self, id: &str) -> Result<Option<SysUser>> {
		let user = sqlx::query_as::<_, SysUser>(
            "SELECT id, email, username, password_hash, status, created_at, updated_at FROM sys_user WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

		Ok(user)
	}

	pub async fn find_by_email(&self, email: &str) -> Result<Option<SysUser>> {
		let user = sqlx::query_as::<_,SysUser>("SELECT id, email, username,password_hash, status, created_at, updated_at FROM sys_user WHERE email = $1").bind(email)
		.fetch_optional(&self.pool).await?;
		Ok(user)
	}

	pub async fn find_by_username(&self, username: &str) -> Result<Option<SysUser>> {
		let user = sqlx::query_as::<_,SysUser>("SELECT id, email, username,password_hash, status, created_at, updated_at FROM sys_user WHERE username = $1").bind(username)
		.fetch_optional(&self.pool).await?;
		Ok(user)
	}

	pub async fn find_exist_superadmin(&self) -> Result<bool> {
		let user = sqlx::query_as::<_, SysUser>(
			"SELECT 1 FROM sys_user WHERE role_type = 1",
		)
		.fetch_optional(&self.pool)
		.await?;
		Ok(true)
	}

	pub async fn create(
		&self,
		email: &str,
		username: &str,
		password_hash: &str,
	) -> Result<SysUser> {
		let mut tx = self.pool.begin().await?;

		let query =
			"INSERT INTO sys_user (email, username, password_hash) VALUES ($1, $2, $3)
				 RETURNING id, email, username, password_hash, status, created_id,created_by, created_at, updated_id, updated_by, updated_at";

		let user = sqlx::query_as::<_, SysUser>(query)
			.bind(email)
			.bind(username)
			.bind(password_hash)
			.fetch_one(&mut *tx)
			.await?;
		tx.commit().await?;
		Ok(user)
	}

	/// 更新用户
	/// 最优雅的方式: 使用 Executor trait 来实现
	/// 这样可以避免在事务中使用时，需要传入 &mut Transaction<'_, Postgres> 的麻烦
	pub async fn update(
		&self,
		id: &str,
		email: Option<&str>,
		username: Option<&str>,
	) -> Result<SysUser> {
		let query = r#"
            UPDATE sys_user
            SET
                email = COALESCE($2, email),
                username = COALESCE($3, username),
                updated_at = NOW()
            WHERE id = $1
            RETURNING id, email, username, password_hash, created_at, updated_at
            "#;
		let user = sqlx::query_as::<_, SysUser>(query)
			.bind(id)
			.bind(email)
			.bind(username)
			.fetch_one(&self.pool)
			.await?;

		Ok(user)
	}

	pub async fn delete(&self, ids: &[String]) -> Result<bool> {
		if ids.is_empty() {
			return Ok(false);
		}
		// // 1. 构建动态参数占位符，例如 $1, $2, ...
		// let placeholders = (1..=ids.len())
		// 	.map(|i| format!("${}", i))
		// 	.collect::<Vec<_>>()
		// 	.join(", ");
		// let query = format!("DELETE FROM sys_user WHERE id in ({})", placeholders);
		// let mut q = sqlx::query(&query);
		// for id in ids {
		// 	q = q.bind(id);
		// }
		// let result = q.execute(&self.pool).await?;

		// 2. PostgreSQL 支持直接传切片
		let result = sqlx::query!("DELETE FROM sys_user WHERE id = ANY($1)", ids)
			.execute(&self.pool)
			.await?;

		Ok(result.rows_affected() > 0)
	}

	pub async fn exists_by_email(&self, email: &str) -> Result<bool> {
		let exists: bool = sqlx::query_scalar(
			"SELECT EXISTS(SELECT 1 FROM sys_user WHERE email = $1)",
		)
		.bind(email)
		.fetch_one(&self.pool)
		.await?;

		Ok(exists)
	}
}

#[cfg(test)]
mod user_repository_tests {

	use sqlx::{postgres::PgPoolOptions, PgPool};

	/// 测试辅助函数：创建测试数据库连接池
	async fn create_test_pool() -> PgPool {
		let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
			"postgresql://postgres:postgres@localhost:15432/gmall".to_string()
		});

		PgPoolOptions::new()
			.max_connections(5)
			.connect(&database_url)
			.await
			.expect("Failed to connect to test database")
	}

	// 测试辅助函数：清理测试数据
	async fn cleanup_test_data(pool: &PgPool) {
		sqlx::query("DELETE FROM sys_user WHERE email LIKE '%@example.com'")
			.execute(pool)
			.await
			.unwrap();
	}

	/// 获取事务，用于隔离测试
	async fn begin_test_transaction(
		pool: &PgPool,
	) -> sqlx::Transaction<'_, sqlx::Postgres> {
		pool.begin().await.expect("Failed to start transaction")
	}
	#[tokio::test]
	async fn clean_up_data_before() {
		let pool: sqlx::Pool<sqlx::Postgres> = create_test_pool().await;
		cleanup_test_data(&pool).await;
	}

	// #[tokio::test]
	// async fn test_find_all() {
	// 	let pool = create_test_pool().await;
	// 	let repo = UserRepository::new(pool.clone());

	// 	// 启动事务
	// 	let mut tx = begin_test_transaction(&pool).await;

	// 	// 在事务内执行

	// 	// 插入一些测试数据
	// 	let user1 = repo
	// 		.create(tx, "test1@example.com", "Test User 1", "123456")
	// 		.await
	// 		.unwrap();
	// 	let user2 = repo
	// 		.create(tx, "test2@example.com", "Test User 2", "123456")
	// 		.await
	// 		.unwrap();

	// 	// 测试查找所有用户
	// 	let users = repo.find_all(&mut *tx).await.unwrap();

	// 	// 至少应该包含我们刚创建的两个用户
	// 	assert!(users.len() >= 2);
	// 	assert!(users.iter().any(|u| u.id == user1.id));
	// 	assert!(users.iter().any(|u| u.id == user2.id));

	// 	tx.rollback().await.unwrap();

	// 	// cleanup_test_data(&pool).await;
	// }

	// #[tokio::test]
	// async fn test_find_by_id() {
	// 	let pool = create_test_pool().await;
	// 	let repo = UserRepository::new(pool.clone());

	// 	// 启动事务
	// 	let mut tx = begin_test_transaction(&pool).await;

	// 	// 创建测试用户
	// 	let created_user = repo
	// 		.create(
	// 			Some(&mut tx),
	// 			"test_find@example.com",
	// 			"Find Test User",
	// 			"123456",
	// 		)
	// 		.await
	// 		.unwrap();

	// 	// 测试通过ID查找用户
	// 	let found_user = repo.find_by_id(&mut *tx, created_user.id).await.unwrap();
	// 	assert!(found_user.is_some());
	// 	let user = found_user.unwrap();
	// 	assert_eq!(user.email, "test_find@example.com");
	// 	assert_eq!(user.username, "Find Test User");

	// 	// 测试查找不存在的用户
	// 	let non_existent_id = Uuid::new_v4();
	// 	let not_found = repo.find_by_id(&mut *tx, non_existent_id).await.unwrap();
	// 	assert!(not_found.is_none());

	// 	tx.rollback().await.unwrap();
	// }

	// #[tokio::test]
	// async fn test_find_by_email() {
	// 	let pool = create_test_pool().await;
	// 	let repo = UserRepository::new(pool.clone());

	// 	// 启动事务
	// 	let mut tx = begin_test_transaction(&pool).await;

	// 	// 创建测试用户
	// 	let created_user = repo
	// 		.create(
	// 			Some(&mut tx),
	// 			"test_email@example.com",
	// 			"Email Test User",
	// 			"123456",
	// 		)
	// 		.await
	// 		.unwrap();

	// 	// 测试通过邮箱查找用户
	// 	let found_user = repo
	// 		.find_by_email(&mut *tx, "test_email@example.com")
	// 		.await
	// 		.unwrap();
	// 	assert!(found_user.is_some());
	// 	let user = found_user.unwrap();
	// 	assert_eq!(user.id, created_user.id);
	// 	assert_eq!(user.email, "test_email@example.com");

	// 	// 测试查找不存在的邮箱
	// 	let not_found = repo
	// 		.find_by_email(&mut *tx, "nonexistent@example.com")
	// 		.await
	// 		.unwrap();
	// 	assert!(not_found.is_none());

	// 	tx.rollback().await.unwrap();
	// }

	// #[tokio::test]
	// async fn test_create_user() {
	// 	let pool = create_test_pool().await;
	// 	let repo = UserRepository::new(pool.clone());

	// 	// 启动事务
	// 	let mut tx = begin_test_transaction(&pool).await;

	// 	// 测试正常创建用户
	// 	let user = repo
	// 		.create(
	// 			Some(&mut tx),
	// 			"test_create@example.com",
	// 			"Create Test User",
	// 			&hash_password("123456").unwrap(),
	// 		)
	// 		.await
	// 		.unwrap();

	// 	assert_eq!(user.email, "test_create@example.com");
	// 	assert_eq!(user.username, "Create Test User");
	// 	assert!(user.created_at <= user.updated_at);

	// 	// 验证用户确实被保存到数据库
	// 	let found_user = repo.find_by_id(&mut *tx, user.id).await.unwrap();
	// 	assert!(found_user.is_some());
	// 	assert_eq!(found_user.unwrap().email, "test_create@example.com");

	// 	tx.rollback().await.unwrap();
	// }

	// #[tokio::test]
	// async fn test_create_user_with_transaction() {
	// 	let pool = create_test_pool().await;
	// 	let repo = UserRepository::new(pool.clone());

	// 	// cleanup_test_data(&pool).await;

	// 	// 测试在事务中创建用户
	// 	let mut tx = pool.begin().await.unwrap();

	// 	let user = repo
	// 		.create(
	// 			Some(&mut tx),
	// 			"test_tx@example.com",
	// 			"Transaction Test User",
	// 			"123456",
	// 		)
	// 		.await
	// 		.unwrap();

	// 	// 提交前验证用户是否在事务中可见
	// 	let user_in_tx = sqlx::query_as::<_, User>(
	//         "SELECT id, email, username, password_hash, created_at, updated_at FROM sys_user WHERE id = $1"
	//     )
	//     .bind(user.id)
	//     .fetch_one(&mut *tx)
	//     .await
	//     .unwrap();

	// 	assert_eq!(user_in_tx.email, "test_tx@example.com");

	// 	// 提交事务
	// 	// tx.commit().await.unwrap();

	// 	// 验证提交后用户是否在数据库中
	// 	let found_user = repo.find_by_id(&mut *tx, user.id).await.unwrap();
	// 	assert!(found_user.is_some());

	// 	tx.rollback().await.unwrap();
	// }

	// #[tokio::test]
	// async fn test_update_user() {
	// 	let pool = create_test_pool().await;
	// 	let repo = UserRepository::new(pool.clone());

	// 	// 测试在事务中创建用户
	// 	let mut tx = pool.begin().await.unwrap();

	// 	// 创建测试用户
	// 	let user = repo
	// 		.create(
	// 			Some(&mut tx),
	// 			"test_update1@example.com",
	// 			"Original Name",
	// 			"123456",
	// 		)
	// 		.await
	// 		.unwrap();

	// 	// 测试更新用户信息
	// 	let updated_user = repo
	// 		.update(
	// 			&mut *tx,
	// 			user.id,
	// 			Some("updated11@example.com"),
	// 			Some("Updated Name"),
	// 		)
	// 		.await
	// 		.unwrap();

	// 	assert_eq!(updated_user.email, "updated11@example.com");
	// 	assert_eq!(updated_user.username, "Updated Name");

	// 	// 验证更新确实保存到数据库
	// 	let found_user = repo.find_by_id(&mut *tx, user.id).await.unwrap().unwrap();
	// 	assert_eq!(found_user.email, "updated11@example.com");
	// 	assert_eq!(found_user.username, "Updated Name");

	// 	tx.rollback().await.unwrap();
	// }

	// #[tokio::test]
	// async fn test_update_user_partial() {
	// 	let pool = create_test_pool().await;
	// 	let repo = UserRepository::new(pool.clone());

	// 	// 测试在事务中创建用户
	// 	let mut tx = pool.begin().await.unwrap();

	// 	// 创建测试用户
	// 	let user = repo
	// 		.create(
	// 			Some(&mut tx),
	// 			"test_partial@example.com",
	// 			"Original Name",
	// 			"123456",
	// 		)
	// 		.await
	// 		.unwrap();

	// 	// 测试部分更新（只更新名称）
	// 	let updated_user = repo
	// 		.update(&mut *tx, user.id, None, Some("Only Name Updated"))
	// 		.await
	// 		.unwrap();

	// 	assert_eq!(updated_user.email, "test_partial@example.com"); // 邮箱保持不变
	// 	assert_eq!(updated_user.username, "Only Name Updated"); // 名称更新

	// 	tx.rollback().await.unwrap();
	// }

	// #[tokio::test]
	// async fn test_delete_user() {
	// 	let pool = create_test_pool().await;
	// 	let repo = UserRepository::new(pool.clone());

	// 	// 测试在事务中创建用户
	// 	let mut tx = pool.begin().await.unwrap();

	// 	// 创建测试用户
	// 	let user1 = repo
	// 		.create(
	// 			Some(&mut tx),
	// 			"test_delete1@example.com",
	// 			"Delete Test 1",
	// 			"123456",
	// 		)
	// 		.await
	// 		.unwrap();
	// 	let user2 = repo
	// 		.create(
	// 			Some(&mut tx),
	// 			"test_delete2@example.com",
	// 			"Delete Test 2",
	// 			"123456",
	// 		)
	// 		.await
	// 		.unwrap();

	// 	// 测试删除单个用户
	// 	let deleted = repo.delete(&mut *tx, &[user1.id]).await.unwrap();
	// 	assert!(deleted);

	// 	// 验证用户1已被删除
	// 	let found_user1 = repo.find_by_id(&mut *tx, user1.id).await.unwrap();
	// 	assert!(found_user1.is_none());

	// 	// 验证用户2仍然存在
	// 	let found_user2 = repo.find_by_id(&mut *tx, user2.id).await.unwrap();
	// 	assert!(found_user2.is_some());

	// 	tx.rollback().await.unwrap();
	// }

	// #[tokio::test]
	// async fn test_delete_multiple_users() {
	// 	let pool = create_test_pool().await;
	// 	let repo = UserRepository::new(pool.clone());

	// 	// 测试在事务中创建用户
	// 	let mut tx = pool.begin().await.unwrap();

	// 	// 创建多个测试用户
	// 	let user1 = repo
	// 		.create(
	// 			Some(&mut tx),
	// 			"test_multi1@example.com",
	// 			"Multi Delete 1",
	// 			"123456",
	// 		)
	// 		.await
	// 		.unwrap();
	// 	let user2 = repo
	// 		.create(
	// 			Some(&mut tx),
	// 			"test_multi2@example.com",
	// 			"Multi Delete 2",
	// 			"123456",
	// 		)
	// 		.await
	// 		.unwrap();
	// 	let user3 = repo
	// 		.create(
	// 			Some(&mut tx),
	// 			"test_multi3@example.com",
	// 			"Multi Delete 3",
	// 			"123456",
	// 		)
	// 		.await
	// 		.unwrap();

	// 	// 测试批量删除
	// 	let ids_to_delete = vec![user1.id, user2.id];
	// 	let deleted = repo.delete(&mut *tx, &ids_to_delete).await.unwrap();
	// 	assert!(deleted);

	// 	// 验证前两个用户已被删除
	// 	assert!(repo.find_by_id(&mut *tx, user1.id).await.unwrap().is_none());
	// 	assert!(repo.find_by_id(&mut *tx, user2.id).await.unwrap().is_none());

	// 	// 验证第三个用户仍然存在
	// 	assert!(repo.find_by_id(&mut *tx, user3.id).await.unwrap().is_some());

	// 	tx.rollback().await.unwrap();
	// }

	// #[tokio::test]
	// async fn test_delete_empty_ids() {
	// 	let pool = create_test_pool().await;
	// 	let repo = UserRepository::new(pool.clone());
	// 	// 测试在事务中创建用户
	// 	let mut tx = pool.begin().await.unwrap();

	// 	// 测试空ID列表
	// 	let deleted = repo.delete(&mut *tx, &[]).await.unwrap();
	// 	assert!(!deleted); // 应该返回false，因为没有删除任何记录
	// 	tx.rollback().await.unwrap();
	// }

	// #[tokio::test]
	// async fn test_delete_with_transaction() {
	// 	let pool = create_test_pool().await;
	// 	let repo = UserRepository::new(pool.clone());

	// 	// 测试在事务中创建用户
	// 	let mut tx = pool.begin().await.unwrap();

	// 	// 创建测试用户
	// 	let user = repo
	// 		.create(
	// 			Some(&mut tx),
	// 			"test_delete_tx@example.com",
	// 			"Delete TX User",
	// 			"123456",
	// 		)
	// 		.await
	// 		.unwrap();

	// 	// 验证用户
	// 	let found_user = repo.find_by_id(&mut *tx, user.id).await.unwrap();
	// 	assert!(found_user.is_some());

	// 	let deleted = repo.delete(&mut *tx, &[user.id]).await.unwrap();
	// 	assert!(deleted);

	// 	// 回滚事务
	// 	tx.rollback().await.unwrap();
	// }

	// #[tokio::test]
	// async fn test_exists_by_email() {
	// 	let pool = create_test_pool().await;
	// 	let repo = UserRepository::new(pool.clone());

	// 	// 测试在事务中创建用户
	// 	let mut tx = pool.begin().await.unwrap();

	// 	// 创建测试用户
	// 	repo.create(
	// 		Some(&mut tx),
	// 		"test_exists@example.com",
	// 		"Exists Test User",
	// 		"123456",
	// 	)
	// 	.await
	// 	.unwrap();

	// 	// 测试存在的邮箱
	// 	let exists = repo
	// 		.exists_by_email(&mut *tx, "test_exists@example.com")
	// 		.await
	// 		.unwrap();
	// 	assert!(exists);

	// 	// 测试不存在的邮箱
	// 	let not_exists = repo
	// 		.exists_by_email(&mut *tx, "nonexistent@example.com")
	// 		.await
	// 		.unwrap();
	// 	assert!(!not_exists);

	// 	tx.rollback().await.unwrap();
	// }

	// #[tokio::test]
	// async fn test_update_nonexistent_user() {
	// 	let pool = create_test_pool().await;
	// 	let repo = UserRepository::new(pool.clone());
	// 	// 测试在事务中创建用户
	// 	let mut tx = pool.begin().await.unwrap();

	// 	// 测试更新不存在的用户（应该返回错误）
	// 	let non_existent_id = Uuid::new_v4();
	// 	let result = repo
	// 		.update(
	// 			&mut *tx,
	// 			non_existent_id,
	// 			Some("new@example.com"),
	// 			Some("New Name"),
	// 		)
	// 		.await;

	// 	assert!(result.is_err());
	// 	tx.rollback().await.unwrap();
	// }

	// #[tokio::test]
	// async fn test_transaction_rollback() {
	// 	let pool = create_test_pool().await;
	// 	let repository = UserRepository::new(pool.clone());

	// 	let result: Result<(), AppError> = async {
	// 		let mut tx = pool.begin().await?;
	// 		// 创建用户
	// 		let email = format!("tx_test_{}@example.com", Uuid::new_v4());
	// 		let password_hash = hash_password("password").unwrap();

	// 		let user = repository
	// 			.create(Some(&mut tx), &email, "Test User", "123456")
	// 			.await;
	// 		tx.rollback().await?;
	// 		Ok(())
	// 	}
	// 	.await;

	// 	assert!(result.is_ok(), "Transaction should rollback successfully");

	// 	let count: i64 = sqlx::query_scalar(
	// 		"SELECT COUNT(*) FROM sys_user where email like 'tx_test_%'",
	// 	)
	// 	.fetch_one(&pool)
	// 	.await
	// 	.expect("Count query should succeed");
	// 	assert_eq!(count, 0, "No users should exist after rollback");
	// }
}

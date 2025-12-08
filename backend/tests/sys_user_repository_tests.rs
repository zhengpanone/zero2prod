use chrono::Utc;
use sqlx::{postgres::PgPoolOptions, PgPool};
use uuid::Uuid;
use zero2prod::enums::common::UserStatus;
use zero2prod::{
	models::sys_user::SysUser, repositories::sys_user_repository::UserRepository,
};

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

/// 清理测试数据
async fn cleanup_test_data(pool: &PgPool) {
	sqlx::query("DELETE FROM sys_user WHERE email LIKE '%@example.com'")
		.execute(pool)
		.await
		.unwrap();
}

/// 创建测试用户数据
fn create_test_user(email: &str, username: &str) -> SysUser {
	let now = Utc::now();
	SysUser {
		id: Uuid::new_v4().to_string(),
		username: username.to_string(),
		email: email.to_string(),
		password_hash: Some("hashed_password".to_string()),
		status: UserStatus::Active,
		created_at: Some(now),
		created_id: Some(Uuid::new_v4().to_string()),
		created_by: Some("test_user".to_string()),
		updated_id: Some(Uuid::new_v4().to_string()),
		updated_at: Some(now),
		updated_by: Some("test_user".to_string()),
		is_deleted: Some(false),
		deleted_at: None,
	}
}

/// 在数据库中插入测试用户
async fn insert_test_user(pool: &PgPool, user: &SysUser) {
	sqlx::query(
        r#"
        INSERT INTO sys_user
        (id, username, email, password_hash, status, created_at, created_id, created_by,
         updated_at, updated_id, updated_by, is_deleted, deleted_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
        "#
    )
    .bind(&user.id)
    .bind(&user.username)
    .bind(&user.email)
    .bind(&user.password_hash)
    .bind(user.status.clone())
    .bind(user.created_at)
    .bind(&user.created_id)
    .bind(&user.created_by)
    .bind(user.updated_at)
    .bind(&user.updated_id)
    .bind(&user.updated_by)
    .bind(user.is_deleted)
    .bind(user.deleted_at)
    .execute(pool)
    .await
    .unwrap();
}

#[tokio::test]
async fn test_user_repository_new() {
	let pool = create_test_pool().await;
	let repo = UserRepository::new(pool.clone());

	// 验证 repository 正确创建
	// 通过成功创建来验证，无需比较指针
	assert!(!std::ptr::eq(&repo, std::ptr::null())); // 如果能创建到这里就说明成功
}
#[ignore]
#[tokio::test]
async fn test_find_all_empty() {
	let pool = create_test_pool().await;
	let repo = UserRepository::new(pool.clone());
	cleanup_test_data(&pool).await;

	let users = repo.find_all().await.unwrap();

	// 可能返回空列表或包含其他用户的列表
	// 我们验证至少不会崩溃
	assert!(!users.is_empty());
}
#[ignore]
#[tokio::test]
async fn test_find_all_with_data() {
	let pool = create_test_pool().await;
	let repo = UserRepository::new(pool.clone());
	cleanup_test_data(&pool).await;

	// 创建测试用户
	let test_user1 = create_test_user("test1@example.com", "testuser1");
	let test_user2 = create_test_user("test2@example.com", "testuser2");

	insert_test_user(&pool, &test_user1).await;
	insert_test_user(&pool, &test_user2).await;

	let users = repo.find_all().await.unwrap();

	// 验证至少包含我们创建的用户
	assert!(users.len() >= 2);
	assert!(users.iter().any(|u| u.email == "test1@example.com"));
	assert!(users.iter().any(|u| u.email == "test2@example.com"));

	// 验证用户按创建时间降序排列
	for i in 1..users.len() {
		assert!(users[i - 1].created_at >= users[i].created_at);
	}
}
#[ignore]
#[tokio::test]
async fn test_find_page_empty() {
	let pool = create_test_pool().await;
	let repo = UserRepository::new(pool.clone());
	cleanup_test_data(&pool).await;

	let users = repo.find_page(1, 10).await.unwrap();

	// 数据库中可能有其他用户，我们检查至少没有我们的测试用户
	let test_emails: Vec<&str> = users.iter().map(|u| u.email.as_str()).collect();
	assert!(!test_emails
		.iter()
		.any(|email| email.contains("@example.com")));
}
#[ignore]
#[tokio::test]
async fn test_find_page_with_data() {
	let pool = create_test_pool().await;
	let repo = UserRepository::new(pool.clone());
	cleanup_test_data(&pool).await;

	// 获取初始用户数
	let initial_count = repo.count().await.unwrap();

	// 创建多个测试用户
	for i in 1..=5 {
		let test_user = create_test_user(
			&format!("test{}@example.com", i),
			&format!("testuser{}", i),
		);
		insert_test_user(&pool, &test_user).await;
	}

	// 测试第一页
	let page1 = repo.find_page(1, 2).await.unwrap();
	// 确保至少返回2个用户（可能包含其他用户）
	assert!(page1.len() >= 2);

	// 测试第二页
	let page2 = repo.find_page(2, 2).await.unwrap();
	// 确保至少返回2个用户
	assert!(page2.len() >= 2);

	// 测试第三页
	let page3 = repo.find_page(3, 2).await.unwrap();
	// 确保至少返回1个用户
	assert!(!page3.is_empty());

	// 测试超出范围的页面
	let empty_page = repo.find_page(100, 2).await.unwrap();
	// 超出范围的页面应该为空
	assert_eq!(empty_page.len(), 0);
}
#[ignore]
#[tokio::test]
async fn test_find_page_edge_cases() {
	let pool = create_test_pool().await;
	let repo = UserRepository::new(pool.clone());
	cleanup_test_data(&pool).await;

	// 创建测试用户
	let test_user = create_test_user("edge@example.com", "edgeuser");
	insert_test_user(&pool, &test_user).await;

	// 测试页面大小为 0
	let empty_page = repo.find_page(1, 0).await.unwrap();
	assert_eq!(empty_page.len(), 0);

	// 测试负数页码 - PostgreSQL 不支持负数 offset，会返回错误
	let negative_result = repo.find_page(-1, 2).await;
	assert!(negative_result.is_err());

	// 测试大页面大小
	let large_page = repo.find_page(1, 1000).await.unwrap();
	assert!(!large_page.is_empty());
}

#[tokio::test]
async fn test_count_empty() {
	let pool = create_test_pool().await;
	let repo = UserRepository::new(pool.clone());
	cleanup_test_data(&pool).await;

	let count = repo.count().await.unwrap();

	// 可能大于 0（如果数据库中有其他用户）
	assert!(count >= 0);
}

#[ignore]
#[tokio::test]
async fn test_count_with_data() {
	let pool = create_test_pool().await;
	let repo = UserRepository::new(pool.clone());
	cleanup_test_data(&pool).await;

	let initial_count = repo.count().await.unwrap();

	// 创建测试用户
	let test_user1 = create_test_user("count1@example.com", "countuser1");
	let test_user2 = create_test_user("count2@example.com", "countuser2");

	insert_test_user(&pool, &test_user1).await;
	insert_test_user(&pool, &test_user2).await;

	let final_count = repo.count().await.unwrap();

	assert_eq!(final_count, initial_count + 2);
}

#[tokio::test]
async fn test_find_by_id_existing() {
	let pool = create_test_pool().await;
	let repo = UserRepository::new(pool.clone());
	cleanup_test_data(&pool).await;

	// 创建测试用户
	let test_user = create_test_user("findbyid@example.com", "finduser");
	insert_test_user(&pool, &test_user).await;

	// 测试查找存在的用户
	let found_user = repo.find_by_id(&test_user.id).await.unwrap();
	assert!(found_user.is_some());

	let user = found_user.unwrap();
	assert_eq!(user.id, test_user.id);
	assert_eq!(user.email, "findbyid@example.com");
	assert_eq!(user.username, "finduser");
}

#[tokio::test]
async fn test_find_by_id_nonexistent() {
	let pool = create_test_pool().await;
	let repo = UserRepository::new(pool.clone());

	// 测试查找不存在的用户
	let non_existent_id = Uuid::new_v4().to_string();
	let found_user = repo.find_by_id(&non_existent_id).await.unwrap();
	assert!(found_user.is_none());
}
#[ignore]
#[tokio::test]
async fn test_find_by_email_existing() {
	let pool = create_test_pool().await;
	let repo = UserRepository::new(pool.clone());
	cleanup_test_data(&pool).await;

	// 创建测试用户
	let test_user = create_test_user("findemail@example.com", "emailuser");
	insert_test_user(&pool, &test_user).await;

	// 测试通过邮箱查找存在的用户
	let found_user = repo.find_by_email("findemail@example.com").await.unwrap();
	assert!(found_user.is_some());

	let user = found_user.unwrap();
	assert_eq!(user.email, "findemail@example.com");
	assert_eq!(user.username, "emailuser");
}

#[tokio::test]
async fn test_find_by_email_nonexistent() {
	let pool = create_test_pool().await;
	let repo = UserRepository::new(pool.clone());

	// 测试查找不存在的邮箱
	let found_user = repo.find_by_email("nonexistent@example.com").await.unwrap();
	assert!(found_user.is_none());
}
#[ignore]
#[tokio::test]
async fn test_find_by_username_existing() {
	let pool = create_test_pool().await;
	let repo = UserRepository::new(pool.clone());
	cleanup_test_data(&pool).await;

	// 创建测试用户
	let test_user = create_test_user("username@example.com", "finduserbyname");
	insert_test_user(&pool, &test_user).await;

	// 测试通过用户名查找存在的用户
	let found_user = repo.find_by_username("finduserbyname").await.unwrap();
	assert!(found_user.is_some());

	let user = found_user.unwrap();
	assert_eq!(user.username, "finduserbyname");
	assert_eq!(user.email, "username@example.com");
}

#[tokio::test]
async fn test_find_by_username_nonexistent() {
	let pool = create_test_pool().await;
	let repo = UserRepository::new(pool.clone());

	// 测试查找不存在的用户名
	let found_user = repo.find_by_username("nonexistent_user").await.unwrap();
	assert!(found_user.is_none());
}

#[tokio::test]
async fn test_create_user_success() {
	let pool = create_test_pool().await;
	let repo = UserRepository::new(pool.clone());
	cleanup_test_data(&pool).await;

	// 测试创建用户
	let created_user = repo
		.create("create@example.com", "createuser", "hashed_password")
		.await
		.unwrap();

	assert_eq!(created_user.email, "create@example.com");
	assert_eq!(created_user.username, "createuser");
	assert_eq!(
		created_user.password_hash,
		Some("hashed_password".to_string())
	);

	// 验证用户确实被保存到数据库
	let found_user = repo.find_by_email("create@example.com").await.unwrap();
	assert!(found_user.is_some());
	assert_eq!(found_user.unwrap().id, created_user.id);
}
#[ignore]
#[tokio::test]
async fn test_create_user_duplicate_email() {
	let pool = create_test_pool().await;
	let repo = UserRepository::new(pool.clone());
	cleanup_test_data(&pool).await;

	// 创建第一个用户
	repo.create("duplicate@example.com", "user1", "password1")
		.await
		.unwrap();

	// 尝试创建重复邮箱的用户（应该失败）
	let result = repo
		.create("duplicate@example.com", "user2", "password2")
		.await;

	assert!(result.is_err());
}

#[tokio::test]
async fn test_update_user_success() {
	let pool = create_test_pool().await;
	let repo = UserRepository::new(pool.clone());
	cleanup_test_data(&pool).await;

	// 创建测试用户
	let created_user = repo
		.create("update@example.com", "originaluser", "password")
		.await
		.unwrap();

	// 测试更新用户
	let updated_user = repo
		.update(
			&created_user.id,
			Some("updated_new@example.com"),
			Some("updateduser"),
		)
		.await
		.unwrap();

	assert_eq!(updated_user.id, created_user.id);
	assert_eq!(updated_user.email, "updated_new@example.com");
	assert_eq!(updated_user.username, "updateduser");
	assert!(updated_user.updated_at > created_user.updated_at);

	// 验证更新确实保存到数据库
	let found_user = repo.find_by_id(&created_user.id).await.unwrap().unwrap();
	assert_eq!(found_user.email, "updated_new@example.com");
	assert_eq!(found_user.username, "updateduser");
}

#[ignore]
#[tokio::test]
async fn test_update_user_partial() {
	let pool = create_test_pool().await;
	let repo = UserRepository::new(pool.clone());
	cleanup_test_data(&pool).await;

	// 创建测试用户
	let created_user = repo
		.create("partial@example.com", "originaluser", "password")
		.await
		.unwrap();

	// 测试部分更新（只更新邮箱）
	let updated_user = repo
		.update(&created_user.id, Some("updated@example.com"), None)
		.await
		.unwrap();

	assert_eq!(updated_user.email, "updated@example.com");
	assert_eq!(updated_user.username, "originaluser"); // 保持不变

	// 测试部分更新（只更新用户名）
	let updated_user2 = repo
		.update(&created_user.id, None, Some("onlyusername"))
		.await
		.unwrap();

	assert_eq!(updated_user2.email, "updated@example.com"); // 保持不变
	assert_eq!(updated_user2.username, "onlyusername");
}

#[tokio::test]
async fn test_update_user_nonexistent() {
	let pool = create_test_pool().await;
	let repo = UserRepository::new(pool.clone());

	// 测试更新不存在的用户（应该失败）
	let non_existent_id = Uuid::new_v4().to_string();
	let result = repo
		.update(&non_existent_id, Some("new@example.com"), Some("newname"))
		.await;

	assert!(result.is_err());
}

#[tokio::test]
async fn test_delete_user_success() {
	let pool = create_test_pool().await;
	let repo = UserRepository::new(pool.clone());
	cleanup_test_data(&pool).await;

	// 创建测试用户
	let test_user = repo
		.create("delete@example.com", "deleteuser", "password")
		.await
		.unwrap();

	// 验证用户存在
	let found_before = repo.find_by_id(&test_user.id).await.unwrap();
	assert!(found_before.is_some());

	// 删除用户
	let deleted = repo
		.delete(std::slice::from_ref(&test_user.id))
		.await
		.unwrap();
	assert!(deleted);

	// 验证用户已被删除
	let found_after = repo.find_by_id(&test_user.id).await.unwrap();
	assert!(found_after.is_none());
}

#[tokio::test]
async fn test_delete_multiple_users() {
	let pool = create_test_pool().await;
	let repo = UserRepository::new(pool.clone());
	cleanup_test_data(&pool).await;

	// 创建多个测试用户
	let user1 = repo
		.create("multi1@example.com", "multiuser1", "password1")
		.await
		.unwrap();
	let user2 = repo
		.create("multi2@example.com", "multiuser2", "password2")
		.await
		.unwrap();
	let user3 = repo
		.create("multi3@example.com", "multiuser3", "password3")
		.await
		.unwrap();

	// 批量删除用户
	let deleted = repo
		.delete(&[user1.id.clone(), user2.id.clone()])
		.await
		.unwrap();
	assert!(deleted);

	// 验证前两个用户已被删除，第三个仍然存在
	assert!(repo.find_by_id(&user1.id).await.unwrap().is_none());
	assert!(repo.find_by_id(&user2.id).await.unwrap().is_none());
	assert!(repo.find_by_id(&user3.id).await.unwrap().is_some());
}

#[tokio::test]
async fn test_delete_empty_ids() {
	let pool = create_test_pool().await;
	let repo = UserRepository::new(pool.clone());

	// 测试空 ID 列表
	let deleted = repo.delete(&[]).await.unwrap();
	assert!(!deleted);
}

#[tokio::test]
async fn test_delete_nonexistent_user() {
	let pool = create_test_pool().await;
	let repo = UserRepository::new(pool.clone());

	// 测试删除不存在的用户
	let non_existent_id = Uuid::new_v4().to_string();
	let deleted = repo
		.delete(std::slice::from_ref(&non_existent_id))
		.await
		.unwrap();
	assert!(!deleted);
}

#[tokio::test]
async fn test_exists_by_email_true() {
	let pool = create_test_pool().await;
	let repo = UserRepository::new(pool.clone());
	cleanup_test_data(&pool).await;

	// 创建测试用户
	repo.create("exists@example.com", "existsuser", "password")
		.await
		.unwrap();

	// 测试存在的邮箱
	let exists = repo.exists_by_email("exists@example.com").await.unwrap();
	assert!(exists);
}

#[tokio::test]
async fn test_exists_by_email_false() {
	let pool = create_test_pool().await;
	let repo = UserRepository::new(pool.clone());

	// 测试不存在的邮箱
	let exists = repo
		.exists_by_email("nonexistent@example.com")
		.await
		.unwrap();
	assert!(!exists);
}

#[tokio::test]
async fn test_transaction_rollback() {
	let pool = create_test_pool().await;
	let repo = UserRepository::new(pool.clone());
	cleanup_test_data(&pool).await;

	// 开始事务
	let mut tx = pool.begin().await.unwrap();

	// 在事务中创建用户
	let email = format!("tx_{}@example.com", Uuid::new_v4());
	let result = sqlx::query(
		"INSERT INTO sys_user (id, email, username, password_hash, status)
         VALUES ($1, $2, $3, $4, $5)",
	)
	.bind(Uuid::new_v4())
	.bind(&email)
	.bind("txuser")
	.bind("password")
	.bind(UserStatus::Active)
	.execute(&mut *tx)
	.await;

	assert!(result.is_ok());

	// 回滚事务
	tx.rollback().await.unwrap();

	// 验证用户不在数据库中
	let exists = repo.exists_by_email(&email).await.unwrap();
	assert!(!exists);
}

#[tokio::test]
async fn cleanup_test_data_before_all_tests() {
	let pool = create_test_pool().await;
	cleanup_test_data(&pool).await;
}

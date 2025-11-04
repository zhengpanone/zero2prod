use std::sync::Arc;

use uuid::Uuid;

use crate::{
	errors::{AppError, Result},
	models::sys_user::SysUser,
	repositories::user_repository::UserRepository,
	schemas::{
		common_schemas::IdsRequest,
		user_schemas::{CreateUserRequest, UpdateUserRequest},
	},
	state::AppState,
	utils::encrypt::hash_password,
};

pub struct UserService {
	repository: UserRepository,
	state: Arc<AppState>,
}

impl UserService {
	pub fn new(state: Arc<AppState>) -> Self {
		let repository = UserRepository::new(state.db.clone());
		Self { repository, state }
	}

	pub async fn list_users(&self) -> Result<Vec<SysUser>> {
		self.repository.find_all().await
	}

	pub async fn get_user_detail(&self, id: Uuid) -> Result<SysUser> {
		self.repository.find_by_id(id).await?.ok_or_else(|| {
			AppError::NotFound(format!("User with id {} not found", id))
		})
	}

	pub async fn create_user(&self, req: CreateUserRequest) -> Result<SysUser> {
		// 检查邮箱是否已存在
		if self.repository.exists_by_email(&req.email).await? {
			return Err(AppError::BadRequest(format!(
				"User with email {} already exists",
				req.email
			)));
		}
		let password_hash = hash_password(&req.password)?;
		self.repository
			.create(&req.email, &req.username, &password_hash)
			.await
	}

	pub async fn delete_user(&self, ids: IdsRequest) -> Result<()> {
		// 将字符串 ID 转换为 Uuid，更安全地处理解析错误
		let ids: Vec<Uuid> = ids
			.ids
			.iter()
			.map(|id| {
				id.parse::<Uuid>().map_err(|e| {
					AppError::BadRequest(format!("Invalid UUID: {}", e))
				})
			})
			.collect::<Result<Vec<Uuid>>>()?;
		let deleted = self.repository.delete(&ids).await?;
		if !deleted {
			return Err(AppError::NotFound(format!(
				"User with id {:?} not found",
				&ids
			)));
		}
		Ok(())
	}

	pub async fn update_user(
		&self,
		id: Uuid,
		req: UpdateUserRequest,
	) -> Result<SysUser> {
		let pool = &self.repository.pool;
		// Rust 提供 as_deref() 方法，可以把 Option<String> 转成 Option<&str>
		let user = self
			.repository
			.update(id, req.email.as_deref(), req.username.as_deref())
			.await?;
		Ok(user)
	}

	pub async fn find_by_username(&self, username: &str) -> Result<Option<SysUser>> {
		let user = self.repository.find_by_username(username).await?;
		Ok(user)
	}

	pub async fn find_by_email(&self, email: &str) -> Result<Option<SysUser>> {
		let user = self.repository.find_by_email(email).await?;
		Ok(user)
	}
}

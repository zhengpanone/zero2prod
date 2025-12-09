use std::sync::Arc;

use crate::{
	errors::{AppError, Result},
	models::sys_role::SysRole,
	repositories::sys_role_repository::SysRoleRepository,
	schemas::{
		common_schemas::IdsRequest,
		sys_role_schemas::{CreateRoleRequest, UpdateRoleRequest},
	},
	state::AppState,
};

pub struct SysRoleService {
	state: Arc<AppState>,
	repository: SysRoleRepository,
}

impl SysRoleService {
	pub fn new(state: Arc<AppState>) -> Self {
		let repository = SysRoleRepository::new(state.db.clone());
		Self { repository, state }
	}

	pub async fn list_roles(&self) -> Result<Vec<SysRole>> {
		todo!()
	}

	pub async fn get_role_detail(&self, id: String) -> Result<SysRole> {
		todo!()
	}

	pub async fn create_role(&self, req: CreateRoleRequest) -> Result<SysRole> {
		if self
			.repository
			.exists_by_code_name(&req.role_name, &req.role_code)
			.await?
		{
			return Err(AppError::BadRequest(format!(
				"Role with name: {} or code: {} already exists",
				&req.role_name, &req.role_name
			)));
		}
		// Ok(SysRole::from(req))
		self.repository.create(req).await
	}

	pub async fn update_role(
		&self,
		id: String,
		req: UpdateRoleRequest,
	) -> Result<SysRole> {
		todo!()
	}

	pub async fn delete_role(&self, ids: IdsRequest) -> Result<()> {
		todo!()
	}
}

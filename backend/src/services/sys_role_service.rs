use std::sync::Arc;

use crate::{
	errors::Result,
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
		todo!()
		// if self.repository.exists_by_code_name(&req.name).await? {
		// 	return Err(AppError::BadRequest(format!(
		// 		"Role with name {} already exists",
		// 		req.name
		// 	)));
		// }
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

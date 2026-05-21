use std::sync::Arc;

use crate::schemas::sys_dict_schemas::{CreateDictTypeRequest, SysDictTypeResponse};
use crate::{repositories::sys_dict_repository::SysDictRepository, state::AppState};
use crate::errors::Result;
use crate::middleware::auth::AuthUser;

pub struct SysDictService {
	state: Arc<AppState>,
	repository: SysDictRepository,
}
impl SysDictService {
	pub fn new(state: Arc<AppState>) -> Self {
		let repository = SysDictRepository::new(state.db.clone());
		Self { state, repository }
	}
	pub async fn create_dict_type(
		&self,
		req: CreateDictTypeRequest,
		auth_user: Arc<AuthUser>,
	) -> Result<SysDictTypeResponse> {
		// self.repository.create()
		todo!()
	}
}

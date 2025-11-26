use std::sync::Arc;

use crate::{repositories::sys_dept_repository::SysDeptRepository, state::AppState};

pub struct SysDeptService {
	state: Arc<AppState>,
	repository: SysDeptRepository,
}

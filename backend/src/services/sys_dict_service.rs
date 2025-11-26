use std::sync::Arc;

use crate::{repositories::sys_dict_repository::SysDictRepository, state::AppState};

pub struct SysDictService {
	state: Arc<AppState>,
	repository: SysDictRepository,
}

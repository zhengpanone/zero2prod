use std::sync::Arc;

use crate::{repositories::sys_post_repository::SysPostRepository, state::AppState};

pub struct SysPostService {
	state: Arc<AppState>,
	repository: SysPostRepository,
}

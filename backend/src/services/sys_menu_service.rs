use std::sync::Arc;

use crate::{repositories::sys_menu_repository::SysMenuRepository, state::AppState};

pub struct SysMenuService {
	state: Arc<AppState>,
	repository: SysMenuRepository,
}

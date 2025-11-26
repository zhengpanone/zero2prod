use sqlx::PgPool;

#[derive(Debug)]
pub struct SysMenuRepository {
	pub pool: PgPool,
}

impl SysMenuRepository {
	pub fn new(pool: PgPool) -> Self {
		Self { pool }
	}
}

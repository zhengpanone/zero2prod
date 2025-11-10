use sqlx::PgPool;

#[derive(Debug)]
pub struct SysPostRepository {
	pub pool: PgPool,
}

impl SysPostRepository {
	pub fn new(pool: PgPool) -> Self {
		Self { pool }
	}
}

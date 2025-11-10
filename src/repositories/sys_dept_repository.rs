use sqlx::PgPool;

#[derive(Debug)]
pub struct SysDeptRepository {
	pub pool: PgPool,
}

impl SysDeptRepository {
	pub fn new(pool: PgPool) -> Self {
		Self { pool }
	}
}

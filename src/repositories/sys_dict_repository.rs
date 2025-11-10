use sqlx::PgPool;

#[derive(Debug)]
pub struct SysDictRepository {
	pub pool: PgPool,
}

impl SysDictRepository {
	pub fn new(pool: PgPool) -> Self {
		Self { pool }
	}
}

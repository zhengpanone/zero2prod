use sqlx::{postgres::PgPoolOptions, PgPool, Pool, Postgres};

use crate::config::database::DatabaseConfig;

pub type Db = Pool<Postgres>;

pub async fn init_db_pool(
	db_config: &DatabaseConfig,
) -> Result<PgPool, sqlx::Error> {
	PgPoolOptions::new()
		.max_connections(db_config.max_connections)
		.connect(&db_config.url)
		.await
}

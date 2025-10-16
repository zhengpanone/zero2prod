use sqlx::{postgres::PgPoolOptions, PgPool};

use crate::config::database::DatabaseConfig;

pub async fn init_db_pool(db_config: &DatabaseConfig) -> anyhow::Result<PgPool> {
	let pool = PgPoolOptions::new()
		.max_connections(db_config.max_connections)
		.connect(&db_config.url)
		.await?;
	Ok(pool)
}

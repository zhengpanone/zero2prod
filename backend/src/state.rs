use anyhow::Error;
use sqlx::PgPool;
use tracing::info;

use crate::{config::Config, db::init_db_pool};
/// 应用状态
#[derive(Clone)]
pub struct AppState {
	pub db: PgPool,
	pub config: Config,
}

impl AppState {
	pub async fn new(config: Config) -> Result<Self, Error> {
		let db_pool = init_db_pool(&config.database).await?;
		info!("Database connection established");

		// 运行数据库迁移
		sqlx::migrate!("./migrations").run(&db_pool).await?;
		Ok(Self {
			db: db_pool,
			config,
		})
	}
}

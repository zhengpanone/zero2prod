pub mod app;
pub mod database;
pub mod jwt;
pub mod server;

// use crate::config::Settings;
// use crate::db::init_db_pool;
// use once_cell::sync::Lazy;
// use sqlx::PgPool;

// pub static SETTINGS: Lazy<Settings> =
// 	Lazy::new(|| Settings::from_env().expect("Failed to load .env configuration"));

// pub static DB_POOL: Lazy<PgPool> = Lazy::new(|| {
// 	tokio::runtime::Runtime::new()
// 		.unwrap()
// 		.block_on(init_db_pool(&SETTINGS.database_url))
// 		.expect("Failed to init DB pool")
// });

use std::env;

use anyhow::{Context, Ok, Result};
use serde::Deserialize;

use crate::config::{
	app::{AppConfig, Environment},
	database::{DatabaseConfig, RedisConfig},
	jwt::JwtConfig,
	server::ServerConfig,
};

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
	pub server: ServerConfig,
	pub database: DatabaseConfig,
	pub redis: RedisConfig,
	// pub rabbitmq: RabbitMQConfig,
	pub jwt: JwtConfig,
	pub app: AppConfig,
	// pub monitoring: MonitoringConfig,
}

impl Config {
	pub fn from_env() -> Result<Self> {
		let server = ServerConfig {
			port: env::var("PORT")
				.unwrap_or_else(|_| "3000".to_string())
				.parse::<u16>()
				.context("Invalid PORT")?,
			host: env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
		};

		let database = DatabaseConfig {
			url: env::var("DATABASE_URL").context("DATABASE_URL must be set")?,
			max_connections: env::var("DATABASE_MAX_CONNECTIONS")
				.unwrap_or_else(|_| "5".to_string())
				.parse::<u32>()
				.context("Invalid DATABASE_MAX_CONNECTIONS")?,
		};

		let redis = RedisConfig {
			url: env::var("REDIS_URL")
				.unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string()),
		};

		let jwt = JwtConfig {
			secret: std::env::var("JWT_SECRET").unwrap_or_else(|_| {
				"your-secret-key-change-in-production".to_string()
			}),
			access_ttl_min: env::var("ACCESS_TOKEN_TTL_MIN")
				.ok()
				.and_then(|s| s.parse().ok())
				.unwrap_or(15),
			refresh_ttl_days: env::var("REFRESH_TOKEN_TTL_DAYS")
				.ok()
				.and_then(|s| s.parse().ok())
				.unwrap_or(7),
			expiration_hours: std::env::var("JWT_EXPIRATION_HOURS")
				.unwrap_or_else(|_| "24".to_string())
				.parse()
				.context("Invalid JWT_EXPIRATION_HOURS")?,
		};

		let app = AppConfig {
			name: env::var("APP_NAME").unwrap_or_else(|_| "zero2prod".to_string()),
			environment: env::var("ENVIRONMENT")
				.unwrap_or_else(|_| "development".to_string())
				.parse::<Environment>()
				.unwrap_or(Environment::Development),
			log_level: env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string()),
		};

		Ok(Config {
			server,
			database,
			redis,
			jwt,
			app,
		})
	}
}

#[cfg(test)]
mod config_tests {
	use std::env;

	use crate::config::Config;

	/// 测试配置加载
	#[test]
	#[ignore] // 忽略此测试，因为它依赖环境变量
	fn test_config_from_env() {
		// 设置必需的环境变量
		env::set_var(
			"DATABASE_URL",
			"postgres://user:password@localhost:5432/gmall",
		);
		env::set_var("PORT", "3000");

		let config = Config::from_env().unwrap();
		assert_eq!(config.server.port, 3000);
		assert_eq!(
			config.database.url,
			"postgres://user:password@localhost:5432/gmall"
		);

		// 清理
		env::remove_var("DATABASE_URL");
		env::remove_var("PORT");
	}
}

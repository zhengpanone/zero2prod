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

use std::{env, str::FromStr};

use anyhow::{Context, Ok, Result};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct JwtConfig {
	pub secret: String,
	pub expiration_hours: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
	pub name: String,
	pub environment: Environment,
	pub log_level: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
	pub port: u16,
	pub host: String,
}
#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
	pub url: String,
	pub max_connections: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RedisConfig {
	pub url: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Environment {
	Development,
	Staging,
	Production,
}
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

impl FromStr for Environment {
	type Err = anyhow::Error;

	fn from_str(s: &str) -> Result<Self> {
		match s.to_lowercase().as_str() {
			"development" | "dev" => Ok(Environment::Development),
			"staging" => Ok(Environment::Staging),
			"production" | "prod" => Ok(Environment::Production),
			_ => Err(anyhow::Error::msg("Invalid environment")),
		}
	}
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
			expiration_hours: std::env::var("JWT_EXPIRATION_HOURS")
				.unwrap_or_else(|_| "24".to_string())
				.parse()
				.context("Invalid JWT_EXPIRATION_HOURS")?,
		};

		let app = AppConfig {
			name: env::var("APP_NAME")
				.unwrap_or_else(|_| "zero2prod".to_string()),
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

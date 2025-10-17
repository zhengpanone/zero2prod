use std::str::FromStr;

use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
	pub name: String,
	pub environment: Environment,
	pub log_level: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Environment {
	Development,
	Staging,
	Production,
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
#[cfg(test)]
mod config_tests {
	use crate::config::app::Environment;

	/// 测试环境类型解析
	#[test]
	fn test_environment_parsing() {
		use std::str::FromStr;
		assert_eq!(
			Environment::from_str("development").unwrap(),
			Environment::Development
		);
		assert_eq!(
			Environment::from_str("dev").unwrap(),
			Environment::Development
		);

		assert_eq!(
			Environment::from_str("staging").unwrap(),
			Environment::Staging
		);

		assert_eq!(
			Environment::from_str("production").unwrap(),
			Environment::Production
		);

		// 无效的环境应该返回错误
		assert!(Environment::from_str("invalid").is_err());
	}
}

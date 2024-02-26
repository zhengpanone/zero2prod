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

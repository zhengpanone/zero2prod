use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct JwtConfig {
	pub secret: String,
	pub access_ttl_min: i64,
	pub refresh_ttl_days: i64,
	pub expiration_hours: i64,
}

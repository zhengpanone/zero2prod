use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct JwtConfig {
	pub secret: String,
	pub expiration_hours: i64,
}

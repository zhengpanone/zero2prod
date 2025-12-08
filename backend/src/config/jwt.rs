use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct JwtConfig {
	/// 对称签名密钥（HS256）
	pub secret: String,
	/// access token 生存时长（分钟）
	pub access_ttl_min: i64,
	/// refresh token 生存时长（天）
	pub refresh_ttl_days: i64,
	/// 可选 issuer / audience
	pub issuer: Option<String>,

	pub audience: Option<String>,

	pub expiration_hours: i64,
}

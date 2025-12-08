use crate::config::jwt::JwtConfig;
use chrono::{Duration, Utc};
use jsonwebtoken::{
	decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation,
};
use serde::{Deserialize, Serialize};

/// JWT Claims 结构体
///
/// 说明：
/// - `sub` : subject，通常放 user id（字符串形式，例如 UUID）。
/// - `exp` : 到期时间，秒（自 epoch），JWT 标准字段（NumericDate）。
/// - `iat` : 签发时间，秒（自 epoch）。
/// - `jti` : 可选的 JWT ID，用于 refresh token 的唯一标识（服务器可据此做撤销/黑名单）。
#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct Claims {
	/// user id（字符串形式，例如 UUID）
	pub sub: String,
	/// expiration time（秒，自 epoch）
	pub exp: i64,
	/// issued at（秒，自 epoch）
	pub iat: i64,
	/// jti：用于 refresh token 或需要 server-side 撤销/追踪的 token
	pub jti: Option<String>,
}

/// 为 access token 签名（短期有效）
///
/// # 参数
/// - `user_id`：用户 ID 的字符串表示（不应是临时引用）
/// - `jwt_config`：JWT 配置（包含 secret、过期时间等）
///
/// # 返回
/// - 成功返回 compact JWT 字符串
/// - 失败返回 `anyhow::Error`
///
/// # 说明
/// - access token 通常不包含 `jti`（除非你也想对 access token 做 server-side 撤销）
pub fn sign_access(user_id: &str, jwt_config: &JwtConfig) -> anyhow::Result<String> {
	let iat = Utc::now();
	let exp = iat + Duration::minutes(jwt_config.access_ttl_min);
	let claims = Claims {
		sub: user_id.to_string(),
		iat: iat.timestamp(),
		exp: exp.timestamp(),
		jti: None,
	};
	// 使用 HS256 对称签名
	Ok(encode(
		&Header::new(Algorithm::HS256),
		&claims,
		&EncodingKey::from_secret(jwt_config.secret.as_bytes()),
	)?)
}

/// 为 refresh token 签名（长期有效）
///
/// # 参数
/// - `user_id`：用户 ID 的字符串表示
/// - `jti`：refresh token 的唯一 id（建议由调用方生成并持久化）
/// - `jwt_config`：JWT 配置
///
/// # 返回
/// - 成功返回 compact JWT 字符串
/// - 失败返回 `anyhow::Error`
///
/// # 说明
/// - 此函数不负责生成 `jti`（让调用方生成并持久化更灵活，调用方可立即将 jti 写入 DB）
pub fn sign_refresh(
	user_id: &str,
	jti: &str,
	jwt_config: &JwtConfig,
) -> anyhow::Result<String> {
	let iat = Utc::now();
	let exp = iat + Duration::days(jwt_config.refresh_ttl_days);
	let claims = Claims {
		sub: user_id.to_string(),
		iat: iat.timestamp(),
		exp: exp.timestamp(),
		jti: Some(jti.to_string()),
	};
	Ok(encode(
		&Header::new(Algorithm::HS256),
		&claims,
		&EncodingKey::from_secret(jwt_config.secret.as_bytes()),
	)?)
}

/// 验证并解析 token（包含过期校验）
///
/// # 参数
/// - `token`：待验证的 JWT 字符串
/// - `jwt_config`：JWT 配置（包含 secret）
///
/// # 返回
/// - 返回解析出的 `Claims`
/// - 如果 token 无效或过期，会返回错误（jsonwebtoken 的错误会被传递上来）
pub fn verify(token: &str, jwt_config: &JwtConfig) -> anyhow::Result<Claims> {
	let data = decode::<Claims>(
		token,
		&DecodingKey::from_secret(jwt_config.secret.as_bytes()),
		&Validation::new(Algorithm::HS256),
	)?;
	Ok(data.claims)
}

/// 便捷的生成 token 函数（与上面一致，但用于测试或简单场景）
///
/// 注意：这里的 user_id 使用 `&str` 更加通用（你传 `&String` 也可以）
pub fn generate_token(
	user_id: &String,
	secret: &str,
) -> Result<String, jsonwebtoken::errors::Error> {
	let now = Utc::now();
	let exp = (now + Duration::hours(24)).timestamp();
	let iat = now.timestamp();

	let claims = Claims {
		sub: user_id.to_string(),
		exp,
		iat,
		jti: None,
	};
	encode(
		&Header::default(),
		&claims,
		&EncodingKey::from_secret(secret.as_ref()),
	)
}

pub fn validate_token(
	token: &str,
	secret: &str,
) -> Result<Claims, jsonwebtoken::errors::Error> {
	decode::<Claims>(
		token,
		&DecodingKey::from_secret(secret.as_ref()),
		&Validation::default(),
	)
	.map(|data| data.claims)
}

#[cfg(test)]
mod tests {
	use super::*;
	use jsonwebtoken::{encode, EncodingKey, Header};
	use uuid::Uuid;

	const SECRET: &str = "my_secret_key";

	// fn test_generate_token() {
	// 			let jwt = JwtKeys::new(
	// 		&config.jwt.secret,
	// 		config.jwt.access_ttl_min,
	// 		config.jwt.refresh_ttl_days,
	// 	);
	// }

	#[test]
	fn test_generate_and_validate_token() {
		let user_id = Uuid::new_v4().to_string();
		// 生成 token
		let token =
			generate_token(&user_id, SECRET).expect("Failed to generate token");

		// token 应该是非空字符串
		assert!(!token.is_empty(), "Token should not be empty");

		// 验证 token
		let claims =
			validate_token(&token, SECRET).expect("Failed to validate token");

		// 校验解析出的内容
		assert_eq!(claims.sub, user_id.to_string());
		assert!(claims.exp > claims.iat);
	}
	#[test]
	fn test_invalid_secret_should_fail() {
		let user_id = Uuid::new_v4().to_string();

		// 生成 token
		let token =
			generate_token(&user_id, SECRET).expect("Failed to generate token");

		// 使用错误的密钥验证 token
		let result = validate_token(&token, "wrong_secret");
		assert!(
			result.is_err(),
			"Token validation should failed with wrong secret"
		);
	}
	#[test]
	fn test_expired_token_should_fail() {
		use chrono::{Duration, Utc};

		let user_id = Uuid::new_v4();
		let expired_claims = Claims {
			sub: user_id.to_string(),
			exp: (Utc::now() - Duration::hours(1)).timestamp(), // 已过期
			iat: Utc::now().timestamp(),
			jti: None,
		};
		let token = encode(
			&Header::default(),
			&expired_claims,
			&EncodingKey::from_secret(SECRET.as_ref()),
		)
		.unwrap();

		let result = validate_token(&token, SECRET);
		assert!(result.is_err(), "Expired token should fail validation");
	}
}

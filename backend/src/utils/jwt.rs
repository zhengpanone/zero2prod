use crate::{
	config::jwt::JwtConfig,
	errors::{AppError, Result},
	models::sys_user::SysUser,
};
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
	pub email: String,
	pub username: String,
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
pub fn sign_access(user: &SysUser, jwt_config: &JwtConfig) -> Result<String> {
	let iat = Utc::now();
	let exp = iat + Duration::minutes(jwt_config.access_ttl_min);
	let claims = Claims {
		sub: user.id.clone(),
		email: user.email.clone(),
		username: user.username.clone(),
		iat: iat.timestamp(),
		exp: exp.timestamp(),
		jti: None,
	};
	// 使用 HS256 对称签名
	encode(
		&Header::new(Algorithm::HS256),
		&claims,
		&EncodingKey::from_secret(jwt_config.secret.as_bytes()),
	)
	.map_err(|e| {
		// 把 jsonwebtoken 错误包装成 AppError::Internal
		AppError::JWTError(format!("failed to encode access token: {}", e))
	})
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
	user: &SysUser,
	jti: &str,
	jwt_config: &JwtConfig,
) -> Result<String> {
	let iat = Utc::now();
	let exp = iat + Duration::days(jwt_config.refresh_ttl_days);
	let claims = Claims {
		sub: user.id.clone(),
		email: user.email.clone(),
		username: user.username.clone(),
		iat: iat.timestamp(),
		exp: exp.timestamp(),
		jti: Some(jti.to_string()),
	};

	encode(
		&Header::new(Algorithm::HS256),
		&claims,
		&EncodingKey::from_secret(jwt_config.secret.as_bytes()),
	)
	.map_err(|e| {
		// 把 jsonwebtoken 错误包装成 AppError::Internal
		AppError::JWTError(format!("failed to encode refresh token: {}", e))
	})
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
pub fn verify(token: &str, jwt_config: &JwtConfig) -> Result<Claims> {
	decode::<Claims>(
		token,
		&DecodingKey::from_secret(jwt_config.secret.as_bytes()),
		&Validation::new(Algorithm::HS256),
	)
	.map(|data| data.claims)
	.map_err(|e| AppError::JWTError(format!("failed to decode token: {}", e)))
}

/// 便捷的生成 token 函数（与上面一致，但用于测试或简单场景）
///
/// 注意：这里的 user_id 使用 `&str` 更加通用（你传 `&String` 也可以）
pub fn generate_token(user: &SysUser, secret: &str) -> Result<String> {
	let now = Utc::now();
	let exp = (now + Duration::hours(24)).timestamp();
	let iat = now.timestamp();

	let claims = Claims {
		sub: user.id.clone(),
		email: user.email.clone(),
		username: user.username.clone(),
		exp,
		iat,
		jti: None,
	};
	encode(
		&Header::default(),
		&claims,
		&EncodingKey::from_secret(secret.as_ref()),
	)
	.map_err(|e| {
		// 把 jsonwebtoken 错误包装成 AppError::Internal
		AppError::JWTError(format!("failed to encode token: {}", e))
	})
}

pub fn validate_token(token: &str, secret: &str) -> Result<Claims> {
	decode::<Claims>(
		token,
		&DecodingKey::from_secret(secret.as_ref()),
		&Validation::default(),
	)
	.map(|data| data.claims)
	.map_err(|e| {
		// 把 jsonwebtoken 错误包装成 AppError::Internal
		AppError::JWTError(format!("failed to decode token: {}", e))
	})
}

#[cfg(test)]
mod tests {
	use super::*;
	use jsonwebtoken::{encode, EncodingKey, Header};
	use uuid::Uuid;

	const SECRET: &str = "my_secret_key";

	// NOTE: 为了让测试自包含，我们构造一个最小 SysUser 用于签发 token。
	fn make_test_user() -> SysUser {
		SysUser {
			id: Uuid::new_v4().to_string(),
			username: "test_user".to_string(),
			email: "test@example.com".to_string(),
			password_hash: None,
			status: Default::default(),
			created_at: None,
			created_id: None,
			created_by: None,
			updated_id: None,
			updated_at: None,
			updated_by: None,
			is_deleted: None,
			deleted_at: None,
		}
	}

	// fn test_generate_token() {
	// 			let jwt = JwtKeys::new(
	// 		&config.jwt.secret,
	// 		config.jwt.access_ttl_min,
	// 		config.jwt.refresh_ttl_days,
	// 	);
	// }

	#[test]
	fn test_generate_and_validate_token() {
		let user = make_test_user();
		// 生成 token
		let token = generate_token(&user, SECRET).expect("Failed to generate token");

		// token 应该是非空字符串
		assert!(!token.is_empty(), "Token should not be empty");

		// 验证 token
		let claims =
			validate_token(&token, SECRET).expect("Failed to validate token");

		// 校验解析出的内容
		assert_eq!(claims.sub, user.id.clone());
		assert!(claims.exp > claims.iat);
	}
	#[test]
	fn test_invalid_secret_should_fail() {
		let user = make_test_user();

		// 生成 token
		let token = generate_token(&user, SECRET).expect("Failed to generate token");

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

		let user = make_test_user();
		let expired_claims = Claims {
			sub: user.id.clone(),
			email: user.email.clone(),
			username: user.username.clone(),
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

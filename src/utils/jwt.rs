use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct Claims {
	pub sub: String, // user_id
	pub exp: usize,  // expiration
	pub iat: usize,  // issued_at
}

pub fn generate_token(
	user_id: &Uuid,
	secret: &str,
) -> Result<String, jsonwebtoken::errors::Error> {
	let now = Utc::now();
	let exp = (now + Duration::hours(24)).timestamp() as usize;
	let iat = now.timestamp() as usize;

	let claims = Claims {
		sub: user_id.to_string(),
		exp: exp,
		iat: iat,
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

	#[test]
	fn test_generate_and_validate_token() {
		let user_id = Uuid::new_v4();
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
		let user_id = Uuid::new_v4();

		// 生成 token
		let token =
			generate_token(&user_id, SECRET).expect("Failed to generate token");

		// 使用错误的密钥验证 token
		let result = validate_token(&token, "wrong_secret");
		assert!(
			result.is_err(),
			"Token validation should faild with wrong secret"
		);
	}
	#[test]
	fn test_expired_token_should_fail() {
		use chrono::{Duration, Utc};

		let user_id = Uuid::new_v4();
		let expired_claims = Claims {
			sub: user_id.to_string(),
			exp: (Utc::now() - Duration::hours(1)).timestamp() as usize, // 已过期
			iat: Utc::now().timestamp() as usize,
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

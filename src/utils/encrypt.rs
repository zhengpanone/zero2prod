use argon2::{
	password_hash::{
		rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString,
	},
	Argon2,
};

use crate::error::{AppError, Result};

pub fn hash_password(password: &str) -> Result<String> {
	// 随机生成盐
	let salt = SaltString::generate(&mut OsRng);
	// 使用默认配置创建 Argon2 哈希器
	let argon2 = Argon2::default();

	argon2
		.hash_password(password.as_bytes(), &salt)
		.map(|hash| hash.to_string())
		.map_err(|e| {
			AppError::Internal(anyhow::anyhow!("Failed to hash password: {}", e))
		})
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool> {
	let parsed_hash = PasswordHash::new(hash).map_err(|e| {
		AppError::Internal(anyhow::anyhow!("Invalid hash format: {}", e))
	})?;

	Ok(Argon2::default()
		.verify_password(password.as_bytes(), &parsed_hash)
		.is_ok())
}

#[cfg(test)]
mod tests {

	use crate::utils::encrypt::{hash_password, verify_password};

	#[test]
	fn test_password_hashing() {
		let password = "test_password_123";
		// 测试密码哈希
		let hash = hash_password(password).unwrap();

		// 测试密码验证 - 正确密码
		let is_valid = verify_password(password, &hash).unwrap();
		assert!(is_valid, "Password should be valid");

		// 测试密码验证-错误密码
		let is_invalid = verify_password("wrong_password", &hash).unwrap();
		assert!(!is_invalid, "Password should not be valid");
	}
	#[test]
	fn test_gener_uuid() {
		println!("{}", uuid::Uuid::new_v4());
	}
}

use anyhow::Result;
use chrono::{Duration, Utc};
use jsonwebtoken::{
	decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
	pub sub: String,
	pub exp: usize,
}

pub fn generate_token(secret: &str, sub: &str) -> Result<String> {
	let expiration = Utc::now()
		.checked_add_signed(Duration::hours(24))
		.expect("valid timestamp")
		.timestamp() as usize;
	let claims = Claims {
		sub: sub.to_string(),
		exp: expiration,
	};
	let token = encode(
		&Header::default(),
		&claims,
		&EncodingKey::from_secret(secret.as_ref()),
	)?;
	Ok(token)
}

pub fn verify_token(secret: &str, token: &str) -> Result<Claims> {
	let data = decode::<Claims>(
		token,
		&DecodingKey::from_secret(secret.as_ref()),
		&Validation::new(Algorithm::HS256),
	)?;
	Ok(data.claims)
}

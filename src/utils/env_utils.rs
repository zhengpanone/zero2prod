use std::env;

pub fn env_str(key: &str, default_value: &str) -> String {
	env::var(key).unwrap_or_else(|_| default_value.to_string())
}

pub fn env_int<T: std::str::FromStr>(key: &str, default_value: T) -> T {
	env::var(key)
		.ok()
		.and_then(|v| v.parse().ok())
		.unwrap_or(default_value)
}

pub fn env_bool(key: &str, default_value: bool) -> bool {
	env::var(key)
		.ok()
		.and_then(|v| v.parse::<bool>().ok())
		.unwrap_or(true)
}

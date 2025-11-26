use std::env;

use serde::Deserialize;

use crate::utils::env_utils::env_bool;

/// 日志配置
#[derive(Debug, Clone, Deserialize)]
pub struct LogConfig {
	/// 默认日志级别
	pub default_level: String,
	/// 是否启用彩色输出
	pub enable_color: bool,
	/// 是否打印文件名
	pub show_file: bool,
	/// 是否打印行号
	pub show_line_number: bool,
	/// 是否打印线程信息
	pub show_thread: bool,
	/// 特定模块的日志级别
	pub module_levels: Vec<(String, String)>,
}

impl Default for LogConfig {
	fn default() -> Self {
		Self {
			default_level: env::var("LOG_LEVEL")
				.unwrap_or_else(|_| "info".to_string()),
			enable_color: env_bool("ENABLE_COLOR", true),
			show_file: env::var("SHOW_FILE")
				.map(|v| v == "true" || v == "1")
				.unwrap_or(true),
			show_line_number: env::var("SHOW_LINE_NUMBER")
				.map(|v| v == "true" || v == "1")
				.unwrap_or(true),
			show_thread: env::var("SHOW_THREAD")
				.map(|v| v == "true" || v == "1")
				.unwrap_or(true),
			module_levels: vec![
				("hyper".to_string(), "warn".to_string()),
				("sqlx".to_string(), "warn".to_string()),
				("tower".to_string(), "warn".to_string()),
			],
		}
	}
}

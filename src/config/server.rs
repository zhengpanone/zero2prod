use serde::Deserialize;
/// 服务相关配置
#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
	pub port: u16,
	pub host: String,
}

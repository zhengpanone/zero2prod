// use tracing_subscriber::{
// 	layer::SubscriberExt,
// 	util::SubscriberInitExt, // 必须导入这个 trait
// 	EnvFilter,
// };

// // 方案1 初始化日志订阅器
// pub fn init() {
// 	// 初始化日志订阅器
// 	// let subscriber = FmtSubscriber::builder()
// 	// 	.with_max_level(tracing::Level::INFO)
// 	// 	.finish();
// 	// tracing::subscriber::set_global_default(subscriber)
// 	// 	.expect("setting default subscriber failed");

// 	// 注册一个全局的日志记录器

// 	// 1. 构建环境过滤器
// 	// .unwrap_or_else(|_| EnvFilter::new("info"));
// 	let env_filter = match EnvFilter::try_from_default_env() {
// 		Ok(filter) => filter,
// 		Err(_) => {
// 			// 如果没有设置环境变量，使用默认值或自定义值
// 			EnvFilter::new("info")
// 		}
// 	};
// 	// 2. 配置日志格式
// 	let fmt_layer = tracing_subscriber::fmt::layer()
// 		.with_file(true) //打印文件名
// 		.with_line_number(true) //打印行号
// 		.with_thread_ids(true) //打印线程ID
// 		.with_thread_names(true) //打印线程名称
// 		.with_target(false) //不打印target
// 		.compact(); // 使用紧凑格式，可选
// 			  // 3. 注册日志订阅器

// 	// 注册全局订阅者
// 	tracing_subscriber::registry()
// 		.with(env_filter)
// 		.with(fmt_layer)
// 		.init();

// 	tracing::info!("日志系统初始化完成");
// }

// 方案2 初始化日志订阅器
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use crate::config::logger::LogConfig;

/// 使用配置初始化日志系统
pub fn init_with_config(
	config: LogConfig,
) -> Result<(), Box<dyn std::error::Error>> {
	// 构建环境过滤器
	let mut env_filter = match EnvFilter::try_from_default_env() {
		Ok(filter) => filter,
		Err(_) => EnvFilter::new(&config.default_level),
	};

	// 添加模块特定的日志级别
	for (module, level) in &config.module_levels {
		env_filter =
			env_filter.add_directive(format!("{}={}", module, level).parse()?);
	}

	// 配置日志格式层
	let fmt_layer = tracing_subscriber::fmt::layer()
		.with_file(config.show_file) // 打印文件名
		.with_line_number(config.show_line_number) // 打印行号
		.with_thread_ids(config.show_thread) // 打印线程ID
		.with_thread_names(config.show_thread) // 打印线程名称
		.with_target(false) // 不打印target
		.with_ansi(config.enable_color) // 彩色输出
		.compact(); // 使用紧凑格式

	// 注册全局订阅者
	tracing_subscriber::registry()
		.with(env_filter)
		.with(fmt_layer)
		.init();

	info!(
		default_level = %config.default_level,
		"日志系统初始化完成"
	);

	Ok(())
}

#[cfg(test)]
mod tests {

	/// 为测试环境初始化日志
	#[test]
	pub fn init_test() {
		let _ = tracing_subscriber::fmt()
			.with_env_filter("warn") // 测试环境只显示警告和错误
			.with_test_writer() // 用于测试的输出
			.try_init();
	}
}

// // 基本使用
// fn main() {
//     logging::init();

//     tracing::info!("这是一个信息日志");
//     tracing::warn!("这是一个警告日志");
//     tracing::error!("这是一个错误日志");
// }

// // 高级配置使用
// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let config = LogConfig {
//         default_level: "debug".to_string(),
//         module_levels: vec![
//             ("my_app".to_string(), "debug".to_string()),
//             ("sqlx".to_string(), "info".to_string()),
//             ("hyper".to_string(), "warn".to_string()),
//         ],
//         ..Default::default()
//     };

//     logging::init_with_config(config)?;

//     tracing::debug!("调试信息");
//     Ok(())
// }

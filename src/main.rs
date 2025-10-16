use crate::{config::config::Config, state::AppState};
use dotenvy::dotenv;
use std::net::SocketAddr;
use tokio::signal;
use tracing::{info, warn};
use tracing_subscriber::FmtSubscriber;
mod config;
mod db;
mod error;
mod handlers;
mod middleware;
mod models;
mod repositories;
mod routers;
mod schemas;
mod services;
mod state;
mod utils;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	// 加载环境变量
	dotenv().ok();
	// 加载配置
	let config = Config::from_env()?;

	// 初始化日志订阅器
	let subscriber = FmtSubscriber::builder()
		.with_max_level(tracing::Level::INFO)
		.finish();
	tracing::subscriber::set_global_default(subscriber)
		.expect("setting default subscriber failed");

	info!("Starting server...");

	// 创建应用状态
	let state = AppState::new(config.clone()).await?;

	// 构建路由
	let app = routers::create_router(state);

	let addr = SocketAddr::from(([0, 0, 0, 0], config.server.port));

	let listener = tokio::net::TcpListener::bind(addr).await?;

	info!(
		"Starting web server at http://127.0.0.1:{}",
		config.server.port
	);

	axum::serve(listener, app)
		.with_graceful_shutdown(shutdown_signal())
		.await?;

	Ok(())
}

/// 捕获系统信号，触发优雅关闭
async fn shutdown_signal() {
	info!("🔌 Waiting for shutdown signal (Ctrl+C or SIGTERM)...");
	// Ctrl+C 信号（跨平台支持）
	let ctrl_c = async {
		signal::ctrl_c()
			.await
			.expect("failed to install Ctrl+C handler");
		info!("🧹 Received Ctrl+C");
	};
	// Unix 平台支持 SIGTERM
	#[cfg(unix)]
	let terminate = async {
		// signal::unix::signal(signal::unix::SignalKind::terminate())
		// 	.expect("failed to install signal handler")
		// 	.recv()
		// 	.await;
		match signal::unix::signal(signal::unix::SignalKind::terminate()) {
			Ok(mut term) => {
				term.recv().await;
				info!("🧹 Received SIGTERM");
			}
			Err(e) => {
				warn!("⚠️ Failed to install SIGTERM handler: {e}");
			}
		}
	};
	// Windows 不支持 SIGTERM，pending 替代
	#[cfg(not(unix))]
	let terminate = std::future::pending::<()>();

	// 等待任意信号触发
	tokio::select! {
		_=ctrl_c=>{},
		_=terminate=>{},
	}

	info!("🚦 Shutdown signal received, starting graceful shutdown...");
}

use crate::{
	config::Config, init::logger::init_log_with_config, middleware::rate_limiter,
	services::auth_service::AuthService, state::AppState,
};

use clap::Parser;
use dotenvy::dotenv;
use std::{net::SocketAddr, sync::Arc};
use tokio::signal;
use tracing::{error, info, warn};
mod command;
mod config;
mod db;
mod enums;
mod errors;
mod handlers;
mod init;
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
	let config = Config::from_env().expect("Failed to load config");

	let _ = init_log_with_config(config.clone().logger);
	info!("Starting server...");

	// 创建应用状态
	let state = Arc::new(AppState::new(config.clone()).await?);

	let cli = crate::command::Cli::parse();
	match cli.command {
		Some(crate::command::Commands::CreateAdmin {
			username,
			password,
			email,
		}) => {
			let auth_service = AuthService::new(state.clone());
			info!("create superadmin start");
			let exist = auth_service.exist_superadmin().await?;
			if !exist {
				auth_service
					.create_superadmin(username, password, email)
					.await?;
			}
			info!("create superadmin end");
		}
		_ => {
			if false {
				error!("No superadmin found.");
				error!("Please run: zero2prod create-admin -u admin -p admin123");
				return Ok(());
			}
		}
	}

	// 创建限流器
	let rate_limiter = rate_limiter::create_rate_limiter(100);

	// 构建路由（Router<Arc<AppState>>）
	let app = routers::create_router(state.clone(), rate_limiter);

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

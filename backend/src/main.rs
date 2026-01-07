use crate::{
	config::Config, init::logger::init_log_with_config, middleware::rate_limiter,
	services::sys_auth_service::AuthService, state::AppState,
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

#[cfg(test)]
mod tests {

	use axum::{
		body::Body,
		extract::connect_info::MockConnectInfo,
		http::{Request, StatusCode},
	};
	use tower::ServiceExt;

	#[tokio::test]
	// oneshot返回的OneShot对象只能调用一次，
	async fn hello_oneshot_test() {
		use std::sync::Arc;

		use axum::body::to_bytes;
		use axum::extract::Request;
		use axum::{body::Body, http::StatusCode};
		use dotenvy::dotenv;
		use tower::ServiceExt;
		use tracing::info;

		use crate::{
			config::Config, init::logger::init_log_with_config,
			middleware::rate_limiter, routers, state::AppState,
		};

		// 加载环境变量
		dotenv().ok();
		// 加载配置
		let config = Config::from_env().expect("Failed to load config");

		let _ = init_log_with_config(config.clone().logger);
		info!("Starting server...");

		// 创建限流器
		let rate_limiter = rate_limiter::create_rate_limiter(100);

		// 创建应用状态
		let state = Arc::new(AppState::new(config.clone()).await.unwrap());
		let app = routers::create_router(state.clone(), rate_limiter);

		let response = app
			.oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
			.await
			.unwrap();
		// 检查状态码是否为 200
		assert_eq!(response.status(), StatusCode::OK);

		// usize::MAX 表示无限制
		let body_bytes = to_bytes(response.into_body(), 1024 * 1024).await.unwrap();
		let body = String::from_utf8(body_bytes.to_vec()).unwrap();
		assert_eq!(body, "Hello Axum!");
		assert_eq!(&body_bytes[..], b"Hello Axum!");
	}

	// into_service()和 .ready().call()
	// 更底层的 Tower Service trait 模式
	// 需要显式处理可变性
	// 可以多次复用同一个 Service
	// 更灵活，但代码更冗长
	#[tokio::test]
	async fn hello_request_test() {
		use std::sync::Arc;

		use axum::body::Body;
		use axum::extract::Request;
		use dotenvy::dotenv;
		use tower::ServiceExt;
		use tracing::info;

		use crate::{
			config::Config, init::logger::init_log_with_config,
			middleware::rate_limiter, routers, state::AppState,
		};
		use tower::Service;

		// 加载环境变量
		dotenv().ok();
		// 加载配置
		let config = Config::from_env().expect("Failed to load config");

		let _ = init_log_with_config(config.clone().logger);
		info!("Starting server...");

		// 创建限流器
		let rate_limiter = rate_limiter::create_rate_limiter(100);

		// 创建应用状态
		let state = Arc::new(AppState::new(config.clone()).await.unwrap());
		let app = routers::create_router(state.clone(), rate_limiter);
		// 将 Router 转换为 Service
		let mut service = app.into_service();

		let request: Request<Body> =
			Request::builder().uri("/").body(Body::empty()).unwrap();

		let response = service.ready().await.unwrap().call(request).await.unwrap();

		assert_eq!(response.status(), StatusCode::OK);

		// 创建请求构建器
		let request_builder = Request::builder().uri("/");
		// 测试多个请求
		for _ in 0..3 {
			// 使用构建器创建新请求
			let request = create_request();
			let response =
				service.ready().await.unwrap().call(request).await.unwrap();
			assert_eq!(response.status(), 200);
		}
	}

	fn create_request() -> Request<Body> {
		Request::builder().uri("/").body(Body::empty()).unwrap()
	}

	#[tokio::test]
	async fn with_into_make_service_with_connect_info() {
		use axum::body::Body;
		use axum::extract::Request;
		use dotenvy::dotenv;
		use std::net::SocketAddr;
		use std::sync::Arc;
		use tower::Service;
		use tracing::info;

		use crate::{
			config::Config, init::logger::init_log_with_config,
			middleware::rate_limiter, routers, state::AppState,
		};

		// 加载环境变量
		dotenv().ok();
		// 加载配置
		let config = Config::from_env().expect("Failed to load config");

		let _ = init_log_with_config(config.clone().logger);
		info!("Starting server...");

		// 创建限流器
		let rate_limiter = rate_limiter::create_rate_limiter(100);

		// 创建应用状态
		let state = Arc::new(AppState::new(config.clone()).await.unwrap());
		let router = routers::create_router(state.clone(), rate_limiter);

		// 添加 MockConnectInfo 层
		let router =
			router.layer(MockConnectInfo(SocketAddr::from(([127, 0, 0, 1], 3000))));

		// 将 Router 转换为 Service
		let mut service = router.into_service();

		let request = Request::builder().uri("/").body(Body::empty()).unwrap();

		let response = service.ready().await.unwrap().call(request).await.unwrap();
		assert_eq!(response.status(), StatusCode::OK);
	}
}

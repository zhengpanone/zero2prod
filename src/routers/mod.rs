use crate::{
	handlers::ApiDoc,
	middleware::{self as app_middleware, rate_limiter, request_id},
	state::AppState,
};
use axum::{middleware, Router};
use tower_http::{
	compression::CompressionLayer,
	cors::CorsLayer,
	trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
};
use tracing::Level;
use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable as RedocServable};
use utoipa_scalar::Scalar;
use utoipa_scalar::Servable as ScalarServable;
use utoipa_swagger_ui::SwaggerUi;

pub mod auth;
pub mod public;
pub mod user;

pub fn public_routes(state: &AppState) -> Router<AppState> {
	Router::new().merge(public::routes(state.clone()))
	// .route("/", get(|| async { "Welcome to the public API" }))
}

/// API 路由聚合
/// 这里使用引用 `&AppState` 避免移动所有权
pub fn api_routes(state: &AppState) -> Router<AppState> {
	let api_router = Router::new()
		.nest("/auth", auth::routes())
		.nest("/user", user::routes(state.clone()));
	// .nest("/posts", posts::routes())
	// .nest("/comments", comments::routes())
	// .nest("/orders", orders::routes())

	// 给整个 API 加上统一前缀 /api
	Router::new().nest("/api", api_router)
}

/// ADMIN 路由聚合
/// 这里使用引用 `&AppState` 避免移动所有权
pub fn admin_routes(state: &AppState) -> Router<AppState> {
	let admin_router = Router::new()
		.merge(public::routes(state.clone()))
		.nest("/users", user::admin_routes(state.clone()));

	// 给整个 API 加上统一前缀 /api
	Router::new().nest("/admin", admin_router)
}

/// 创建主路由
pub fn create_router(
	state: AppState,
	rate_limiter: rate_limiter::SharedRateLimiter,
) -> Router {
	// 生成 OpenAPI 文档实例
	let api = ApiDoc::openapi();

	// 组合所有路由
	Router::new()
		// Swagger UI
		.merge(
			SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api.clone()),
		)
		// Redoc
		.merge(Redoc::with_url("/redoc", api.clone()))
		.merge(RapiDoc::new("/api-docs/openapi.json").path("/rapidoc"))
		// .merge(RapiDoc::with_openapi("/api-docs/openapi2.json", api).path("/rapidoc"))
		.merge(Scalar::with_url("/scalar", api))
		// 业务 API
		.merge(public_routes(&state))
		.merge(api_routes(&state)) // 传引用，不移动 state
		.merge(admin_routes(&state))
		// 全局中间件
		.layer(
			TraceLayer::new_for_http()
				.make_span_with(DefaultMakeSpan::new().level(Level::INFO))
				.on_response(DefaultOnResponse::new().level(Level::INFO)),
		)
		.layer(request_id::RequestIdLayer)
		.layer(middleware::from_fn(move |req, next| {
			let limiter = rate_limiter.clone();
			app_middleware::rate_limiter::rate_limit_middleware(limiter, req, next)
		}))
		.layer(CompressionLayer::new())
		.layer(CorsLayer::permissive())
		.with_state(state) // 最终注入原始 state
}

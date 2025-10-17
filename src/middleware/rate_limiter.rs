use std::{num::NonZeroU32, sync::Arc};

use axum::{
	body::Body,
	extract::Request,
	http::StatusCode,
	middleware::Next,
	response::{IntoResponse, Response},
};
use governor::{
	clock::DefaultClock,
	state::{InMemoryState, NotKeyed},
	Quota, RateLimiter,
};

/// 可在多线程间共享的全局限流器类型
pub type SharedRateLimiter = Arc<RateLimiter<NotKeyed, InMemoryState, DefaultClock>>;

/// 初始化一个全局限流器
pub fn create_rate_limiter(request_per_second: u32) -> SharedRateLimiter {
	// 每秒允许5次请求
	let quota = Quota::per_second(NonZeroU32::new(request_per_second).unwrap());
	Arc::new(RateLimiter::direct(quota))
}

pub async fn rate_limit_middleware(
	limiter: SharedRateLimiter,
	req: Request,
	next: Next,
) -> Result<Response<Body>, impl IntoResponse> {
	match limiter.check() {
		Ok(_) => Ok(next.run(req).await),
		Err(_) => Err((
			StatusCode::TOO_MANY_REQUESTS,
			"Rate limit exceeded. Please try again later",
		)),
	}
}

#[cfg(test)]
mod tests {

	use axum::{
		body::Body, extract::Request, http::StatusCode, middleware::from_fn,
		routing::get, Router,
	};

	use tower::ServiceExt;

	use crate::middleware::rate_limiter::create_rate_limiter;

	#[tokio::test]
	async fn test_rate_limiter_100_per_sec() {
		let limit = create_rate_limiter(100);

		// 创建一个简单 handler
		async fn handler() -> &'static str {
			"ok"
		}

		let app = Router::new().route("/test", get(handler)).layer(from_fn(
			move |req, next| super::rate_limit_middleware(limit.clone(), req, next),
		));
		// 模拟 105 次请求，前 100 次应成功，超过的返回 429
		let mut success_count = 0;
		let mut rate_limited_count = 0;
		for _ in 0..105 {
			let response = app
				.clone()
				.oneshot(
					Request::builder().uri("/test").body(Body::empty()).unwrap(),
				)
				.await
				.unwrap();

			if response.status() == StatusCode::OK {
				success_count += 1;
			} else if response.status() == StatusCode::TOO_MANY_REQUESTS {
				rate_limited_count += 1;
			}
		}

		println!(
			"Success:{}, Rate Limited:{}",
			success_count, rate_limited_count
		);
		assert!(success_count == 100);
		assert!(rate_limited_count == 5);
	}
}

use axum::{
	body::Body, extract::Request, http::HeaderValue, middleware::Next,
	response::Response,
};
use tower::{Layer, Service};
use uuid::Uuid;

const X_REQUEST_ID: &str = "x-request-id";

#[derive(Clone)]
pub struct RequestIdLayer;

impl<S> Layer<S> for RequestIdLayer {
	type Service = RequestIdMiddleware<S>;

	fn layer(&self, inner: S) -> Self::Service {
		RequestIdMiddleware { inner }
	}
}

#[derive(Clone)]
pub struct RequestIdMiddleware<S> {
	inner: S,
}

impl<S> Service<Request> for RequestIdMiddleware<S>
where
	S: Service<Request, Response = Response<Body>> + Clone + Send + 'static,
	S::Future: Send + 'static,
{
	type Response = S::Response;
	type Error = S::Error;
	type Future = std::pin::Pin<
		Box<
			dyn std::future::Future<Output = Result<Self::Response, Self::Error>>
				+ Send,
		>,
	>;

	fn poll_ready(
		&mut self,
		cx: &mut std::task::Context<'_>,
	) -> std::task::Poll<Result<(), Self::Error>> {
		self.inner.poll_ready(cx)
	}

	fn call(&mut self, mut req: Request) -> Self::Future {
		let request_id = req
			.headers()
			.get(X_REQUEST_ID)
			.and_then(|v| v.to_str().ok())
			.map(|s| s.to_string())
			.unwrap_or_else(|| Uuid::new_v4().to_string());

		req.extensions_mut().insert(request_id.clone());

		let mut inner = self.inner.clone();

		Box::pin(async move {
			let mut response = inner.call(req).await?;
			response
				.headers_mut()
				.insert(X_REQUEST_ID, HeaderValue::from_str(&request_id).unwrap());
			Ok(response)
		})
	}
}

pub async fn add_request_id(mut req: Request, next: Next) -> Response<Body> {
	let request_id = req
		.headers()
		.get(X_REQUEST_ID)
		.and_then(|v| v.to_str().ok())
		.map(|s| s.to_string())
		.unwrap_or_else(|| Uuid::new_v4().to_string());

	req.extensions_mut().insert(request_id.clone());

	let mut response = next.run(req).await;
	response
		.headers_mut()
		.insert(X_REQUEST_ID, HeaderValue::from_str(&request_id).unwrap());
	response
}

use axum::body::Body;

async fn body_to_bytes(body: Body) -> Result<(Vec<u8>, Body), axum::Error> {
	use futures::StreamExt; // for next()

	let mut bytes = Vec::new();
	let mut stream = body.into_data_stream();

	// 自 futures::StreamExt trait。只要正确导入，它就能工作。
	while let Some(chunk) = stream.next().await {
		bytes.extend_from_slice(&chunk.unwrap());
	}
	// 克隆数据，重新建一个Body
	let body = Body::from(bytes.clone());
	Ok((bytes, body))
}

async fn body_to_bytes_use_http_body(body: Body) -> Result<Vec<u8>, axum::Error> {
	use http_body_util::BodyExt; // for collect()

	let bytes = body.collect().await?.to_bytes();
	Ok(bytes.to_vec())
}

#[cfg(test)]
mod tests {
	use axum::{http::StatusCode, response::IntoResponse};

	use crate::{errors::AppError, utils::http::body_to_bytes};
	use serde_json::{json, Value};

	#[tokio::test] // ✅ 用 tokio::test 而不是 #[test]
	async fn test_bad_request_error() {
		// 模拟一个BadRequest错误
		// 1 直接使用字符串字面量
		let err = AppError::BadRequest("参数不合法".to_string());
		// 2. 使用String::from()
		// let err = AppError::BadRequest(String::from("参数不合法"));
		// 3. 使用format!()
		// let err = AppError::BadRequest(format!("参数不合法"));

		// 将错误转换为Response
		let response = err.into_response();
		// 先拷贝状态码
		let status = response.status();

		// 读取响应体
		let (bytes, body) = body_to_bytes(response.into_body()).await.unwrap();

		// 打印或断言
		println!("{}", String::from_utf8_lossy(&bytes));

		assert_eq!(status, StatusCode::BAD_REQUEST);

		let actual: Value = serde_json::from_slice(&bytes).unwrap();

		let expected = json!({
			"code": "BAD_REQUEST",
			"message": "参数不合法",
			"details": null,
		});
		assert_eq!(actual, expected);
	}
}

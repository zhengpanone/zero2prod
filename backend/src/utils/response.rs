use axum::{
	http::StatusCode,
	response::{IntoResponse, Response},
	Json,
};
use serde::Serialize;
use utoipa::ToSchema;

/// 通用响应与分页 VO
#[derive(Serialize, ToSchema)]
pub struct ApiResponse<T>
where
	T: Serialize + ToSchema,
{
	/// 0表示成功，其他表示失败
	/// 响应码
	pub code: i32,
	/// 一般为OK
	/// 响应消息
	pub message: String,
	/// 正常数据或分页数据
	/// 响应数据
	pub data: Option<T>,
}

impl<T: Serialize> ApiResponse<T>
where
	T: Serialize + ToSchema,
{
	#[inline]
	pub fn ok_with_data(data: T) -> (StatusCode, Json<Self>) {
		(
			StatusCode::OK,
			Json(Self {
				code: 0,
				message: "OK".into(),
				data: Some(data),
			}),
		)
	}
	#[inline]
	pub fn ok_with_code_data(code: StatusCode, data: T) -> (StatusCode, Json<Self>) {
		(
			code,
			Json(Self {
				code: 0,
				message: "Ok".into(),
				data: Some(data),
			}),
		)
	}
	/// 无数据的成功
	#[inline]
	pub fn ok() -> (StatusCode, Json<Self>) {
		(
			StatusCode::OK,
			Json(Self {
				code: 0,
				message: "Ok".into(),
				data: None,
			}),
		)
	}
	/// 仅提示信息的成功
	#[inline]
	pub fn message(message: impl Into<String>) -> (StatusCode, Json<Self>) {
		(
			StatusCode::OK,
			Json(Self {
				code: 0,
				message: message.into(),
				data: None,
			}),
		)
	}
}

impl<T> IntoResponse for ApiResponse<T>
where
	T: Serialize + ToSchema,
{
	fn into_response(self) -> Response {
		(StatusCode::OK, Json(self)).into_response()
	}
}

/// 通用分页体
#[derive(Serialize, Clone, ToSchema)]
pub struct Page<T>
where
	T: Serialize + ToSchema,
{
	pub items: Vec<T>,
	pub total: u64,
	pub page_num: u64,
	pub page_size: u64,
	pub total_page: u64,
}

impl<T> ApiResponse<Page<T>>
where
	T: Serialize + ToSchema,
{
	#[inline]
	pub fn page(
		items: Vec<T>,
		total: u64,
		page_num: u64,
		page_size: u64,
		total_page: u64,
	) -> (StatusCode, Json<Self>) {
		let body = Page {
			items,
			total,
			page_num,
			page_size,
			total_page,
		};
		(
			StatusCode::OK,
			Json(ApiResponse {
				code: 0,
				message: "OK".into(),
				data: Some(body),
			}),
		)
	}
}

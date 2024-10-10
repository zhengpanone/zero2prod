use axum::response::{IntoResponse, IntoResponseParts, ResponseParts};
use http::{header, HeaderValue, StatusCode};
use serde::Serialize;
use bytes::{BufMut, BytesMut};
use tracing::log::error;
use crate::errors::Error;


pub type ApiResponseResult<T> = Result<ApiResponse<T>, Error>;
#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub data: Option<T>,
    pub message: Option<String>,
    #[serde(serialize_with = "serialize_status_code")]
    pub status_code: StatusCode,
    pub pagination: Option<ResponsePagination>,
}

// 自定义序列化函数
fn serialize_status_code<S>(status_code: &StatusCode, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_u16(status_code.as_u16())
}

pub struct ApiResponseBuilder<T: Serialize> {
    pub data: Option<T>,
    pub status_code: StatusCode,
    pub pagination: Option<ResponsePagination>,
}

#[derive(Debug, Serialize)]
pub struct ResponsePagination {
    pub count: u64,
    pub offset: u64,
    pub limit: u32,
}


impl<T> Default for ApiResponseBuilder<T>
where
    T: Serialize,
{
    fn default() -> Self {
        Self {
            data: None,
            status_code: StatusCode::OK,
            pagination: None,
        }
    }
}

impl<T> ApiResponseBuilder<T>
where
    T: Serialize,
{
    pub fn new() -> Self {
        Self::default()
    }
    pub fn body(mut self, data: T) -> Self {
        self.data = Some(data);
        self
    }
    pub fn status_code(mut self, status_code: StatusCode) -> Self {
        self.status_code = status_code;
        self
    }

    pub fn pagination(mut self, pagination: ResponsePagination) -> Self {
        self.pagination = Some(pagination);
        self
    }
    pub fn build(self) -> ApiResponse<T> {
        ApiResponse {
            data: self.data,
            message: None,
            status_code: self.status_code,
            pagination: self.pagination,
        }
    }
}

impl<T:Serialize> IntoResponse for ApiResponse<T>
{
    fn into_response(self) -> axum::response::Response {
        let data = match self.data {
            Some(body) => body,
            None => return (self.status_code).into_response(),
        };
        let mut bytes = BytesMut::new().writer();
        if let Err(err) = serde_json::to_writer(&mut bytes, &data) {
            error!("Error serializing response body as JSON: {:?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }

        let bytes = bytes.into_inner().freeze();
        let headers = [(
            header::CONTENT_TYPE,
            HeaderValue::from_static(mime::APPLICATION_JSON.as_ref()),
        )];

        match self.pagination {
            Some(pagination) => (self.status_code, headers, bytes).into_response(),
            None => (self.status_code, headers, bytes).into_response(),
        }
    }
}

impl IntoResponseParts for ResponsePagination {
    type Error = (StatusCode, String);

    fn into_response_parts(self, mut res: ResponseParts) -> Result<ResponseParts, Self::Error> {
        res.headers_mut()
            .insert("x-pagination-count", self.count.into());
        res.headers_mut().insert("x-pagination-offset", self.offset.into());
        res.headers_mut().insert("x-pagination-limit", self.limit.into());

        Ok(res)
    }
}


impl<T: Serialize> ApiResponse<T> {
    pub fn new(data: Option<T>, message: Option<String>, status_code: StatusCode) -> Self {
        Self {
            data,
            message,
            status_code,
            pagination: None,
        }
    }
}
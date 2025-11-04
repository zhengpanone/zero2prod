use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct PageRequest {
	#[serde(default = "default_page_num")]
	pub page_num: i64,
	#[serde(default = "default_page_size")]
	pub page_size: i64,
	#[serde(default = "default_sort_field")]
	pub sort_field: String,
	#[serde(default = "default_sort_order")]
	pub sort_order: String,
}

fn default_page_num() -> i64 {
	1
}

fn default_page_size() -> i64 {
	20
}

fn default_sort_field() -> String {
	"created_at".to_string()
}

impl PageRequest {
	pub fn offset(&self) -> i64 {
		(self.page_num - 1) * self.page_size
	}

	pub fn limit(&self) -> i64 {
		self.page_size
	}
}

fn default_sort_order() -> String {
	"desc".to_string()
}

/// 完整的查询参数（分页+查询条件）
#[derive(Debug, Clone, Deserialize)]
pub struct QueryRequest<T> {
	#[serde(flatten)]
	pub page: PageRequest,
	#[serde(flatten)]
	pub condition: T,
}

impl<T> QueryRequest<T> {
	pub fn new(page: PageRequest, condition: T) -> Self {
		Self { page, condition }
	}
}

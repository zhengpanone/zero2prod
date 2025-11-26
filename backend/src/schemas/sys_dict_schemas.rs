use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateDictTypeRequest {}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateDictTypeRequest {}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateDictDataRequest {}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateDictDataRequest {}

use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateDeptRequest {}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateDeptRequest {}

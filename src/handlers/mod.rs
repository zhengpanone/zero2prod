use utoipa::OpenApi;

pub mod auth;
pub mod health;
pub mod metrics;
pub mod user;

// OpenAPI 文档定义
// 合并多个 OpenApi 文档
#[derive(OpenApi)]
#[openapi(
        nest(
            // you can nest sub apis here
            (path = "/api/user", api = user::UserApiDoc),
            (path = "/auth", api = auth::AuthApiDoc,),
        )
    )]
pub struct ApiDoc;

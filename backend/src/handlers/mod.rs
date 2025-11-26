use utoipa::OpenApi;

pub mod auth;
pub mod health;
pub mod metrics;
pub mod sys_dept;
pub mod sys_dict;
pub mod sys_menu;
pub mod sys_post;
pub mod sys_role;
pub mod sys_user;

// OpenAPI 文档定义
// 合并多个 OpenApi 文档
#[derive(OpenApi)]
#[openapi(
        nest(
            // you can nest sub apis here
            (path = "/api/user", api = sys_user::UserApiDoc),
            (path = "/api/role", api = sys_role::RoleApiDoc),
            (path = "/api/post", api = sys_post::PostApiDoc),
            (path = "/api/menu", api = sys_menu::MenuApiDoc),
            (path = "/api/dept", api = sys_dept::DeptApiDoc),
            (path = "/api/dict", api = sys_dict::DictApiDoc),
            (path = "/api/auth", api = auth::AuthApiDoc,),
        )
    )]
pub struct ApiDoc;

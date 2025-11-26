use utoipa::{
	openapi::security::{HttpAuthScheme, SecurityScheme},
	Modify, OpenApi,
};

use crate::models::{post::Post, user::User};
use crate::routes::{post::list_posts, user::list_users};

pub struct SecurityAddon;

impl Modify for SecurityAddon {
	fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
		openapi.components = Some(
			utoipa::openapi::ComponentsBuilder::new()
				.security_scheme(
					"bearer_auth",
					SecurityScheme::Http(
						utoipa::openapi::security::HttpBuilder::new()
							.scheme(HttpAuthScheme::Bearer)
							.bearer_format("JWT")
							.build(),
					),
				)
				.build(),
		);
	}
}

#[derive(OpenApi)]
#[openapi(
info(title = "Axum Starter API", version = "1.0.0", description = "Demo with utoipa & JWT"),
servers((url = "http://127.0.0.1:3000")),
modifiers(&SecurityAddon),
paths(list_users, list_posts),
components(schemas(User, Post)),
security(("bearer_auth" = []))
)]
pub struct ApiDoc;

use std::sync::Arc;

use sqlx::{Executor, Postgres};
use validator::Validate;

use crate::{
	errors::{AppError, Result},
	schemas::{
		auth_schemas::{AuthResponse, RegisterRequest},
		user_schemas::CreateUserRequest,
	},
	services::user_service::UserService,
	state::AppState,
	utils::{encrypt::hash_password, jwt::generate_token},
};

pub struct AuthService {
	user_service: UserService,
	state: Arc<AppState>,
}

impl AuthService {
	pub fn new(state: Arc<AppState>) -> Self {
		let user_service = UserService::new(Arc::clone(&state));
		Self {
			user_service,
			state,
		}
	}
	pub async fn login<'e, E>(&self, executor: E)
	where
		E: Executor<'e, Database = Postgres>,
	{
	}

	pub async fn register(&self, register: RegisterRequest) -> Result<AuthResponse> {
		register
			.validate()
			.map_err(|e| AppError::Validation(e.to_string()))?;
		// 判断用户是否重复
		let user = self
			.user_service
			.find_by_username(&register.username)
			.await?;
		if user.is_some() {
			return Err(AppError::BadRequest("用户名已存在".to_string()));
		}
		// 创建用户

		let user = self
			.user_service
			.create_user(CreateUserRequest {
				username: register.username.to_string(),
				email: register.username.to_string(),
				password: hash_password(&register.password).unwrap(),
			})
			.await?;
		let token = generate_token(&user.id, &self.state.config.jwt.secret).unwrap();
		let auth_response = AuthResponse {
			user_id: user.id,
			email: user.email,
			username: user.username,
			token,
		};

		Ok(auth_response)
	}
}

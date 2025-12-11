use std::sync::Arc;

use chrono::{Duration, Utc};
use uuid::Uuid;
use validator::Validate;

use crate::{
	errors::{AppError, AuthError, Result},
	schemas::{
		auth_schemas::{
			AuthResponse, LoginRequest, LoginResponse, RefreshRequest,
			RegisterRequest,
		},
		sys_user_schemas::CreateUserRequest,
	},
	services::sys_user_service::UserService,
	state::AppState,
	utils::{
		encrypt::{hash_password, verify_password},
		jwt_utils::{generate_token, sign_access, sign_refresh, verify_token},
	},
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
	pub async fn login(&self, login_request: LoginRequest) -> Result<LoginResponse> {
		login_request
			.validate()
			.map_err(|e| AppError::Validation(e.to_string()))?;

		let user = self
			.user_service
			.find_by_username(&login_request.username)
			.await?
			.ok_or_else(|| {
				AppError::BadRequest("Invalid username or password".to_string())
			})?;
		// TODO 校验邮箱

		let is_valid = verify_password(
			&login_request.password,
			user.password_hash.as_deref().unwrap_or(""),
		)?;
		if !is_valid {
			return Err(AppError::BadRequest(
				"Invalid username or password".to_string(),
			));
		}
		let jti = Uuid::new_v4().to_string();

		let access = sign_access(&user, &self.state.config.jwt)?;

		let refresh = sign_refresh(&user, &jti, &self.state.config.jwt)?;

		sqlx::query!(
		"INSERT INTO refresh_tokens (jti, user_id, expires_at) VALUES ($1,$2,$3)",
		jti,
		user.id,
		Utc::now() + Duration::days(self.state.config.jwt.refresh_ttl_days)
	)
		.execute(&self.state.db)
		.await
		.map_err(AppError::Database)?;

		let login_response = LoginResponse {
			user_id: user.id,
			email: user.email,
			username: user.username,
			access_token: access,
			refresh_token: refresh,
		};
		Ok(login_response)
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
				email: register.email.to_string(),
				password: hash_password(&register.password).unwrap(),
			})
			.await?;
		let token = generate_token(&user, &self.state.config.jwt.secret).unwrap();
		let auth_response = AuthResponse {
			user_id: user.id,
			email: user.email,
			username: user.username,
			token,
		};

		Ok(auth_response)
	}

	pub async fn logout(&self, token: RefreshRequest) -> Result<String> {
		let claims = verify_token(&token.refresh_token, &self.state.config.jwt)
			.map_err(|_| AuthError::BadRequest("退出登录失败！".to_string()))?;
		let jti = claims
			.jti
			.clone()
			.ok_or(AuthError::BadRequest("退出登录失败！".to_string()))?;

		sqlx::query!("UPDATE refresh_tokens SET revoked=true WHERE jti=$1", jti)
			.execute(&self.state.db)
			.await
			.map_err(AppError::Database)?;
		Ok("退出登录成功！".to_string())
	}

	pub async fn create_superadmin(
		&self,
		username: String,
		password: String,
		email: String,
	) -> Result<()> {
		let user = self.user_service.find_by_username(&username).await?;
		if user.is_some() {
			return Err(AppError::BadRequest("用户名已存在".to_string()));
		}
		// _ = self.user_service.create_user(CreateUserRequest {
		// 	username,
		// 	email,
		// 	password: hash_password(&password).unwrap(),
		// });

		Ok(())
	}

	pub async fn exist_superadmin(&self) -> Result<bool> {
		let is_exist = self.user_service.exist_superadmin().await?;
		Ok(is_exist)
	}
}

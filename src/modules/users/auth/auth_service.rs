use anyhow::Result;
use async_trait::async_trait;
use base64::Engine;
use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use chrono::{Duration, Utc};
use rand::distributions::Alphanumeric;
use rand::Rng;
use sha2::{Digest, Sha256};
use uuid::Uuid;

use crate::modules::users::auth::auth_command::*;
use crate::modules::users::auth::auth_dto::*;
use crate::modules::users::auth::auth_model::*;
use crate::modules::users::auth::auth_repo::{AuthRepository, AuthRepositoryInterface};
use crate::modules::users::user::user_model::User;
use crate::modules::users::user::user_repo::{UserRepository, UserRepositoryInterface};
use crate::shared::auth::jwt::{AuthUser, JwtVerifier};
use crate::shared::auth::password::verify_password;
use crate::shared::errors::AppError;
use crate::shared::state::AppState;
use crate::shared::utils::extract_pagination_data;


#[async_trait]
pub trait AuthServiceInterface {

    async fn login(&self, command: LoginCommand) -> Result<Option<String>, AppError>;
    async fn login_alt(&self, command: LoginCommand) -> Result<Option<String>, AppError>;

    async fn register(&self, command: RegisterCommand) -> Result<bool, AppError>;


    async fn request_password_reset(&self, command: CreatePasswordResetCommand) -> Result<TokenGenerationResult, AppError>;
    async fn confirm_password_reset(&self, command: ConfirmPasswordResetCommand) -> Result<Uuid, AppError>;


    async fn request_email_verification(&self, command: CreateEmailVerifyCommand) -> Result<TokenGenerationResult, AppError>;
    async fn confirm_email_verification(&self, command: ConfirmEmailVerifyCommand) -> Result<Uuid, AppError>;


    async fn get_login_log_by_user(&self, command: LoginLogListByUserCommand) -> Result<Vec<LoginLogResponse>, AppError>;

}


#[derive(Clone)]
pub struct AuthService {
    jwt: JwtVerifier,
    auth_repo: AuthRepository,
    user_repo: UserRepository,
    redis_pool: Option<Pool<RedisConnectionManager>>,
}

impl From<&AppState> for AuthService {
    fn from(app_state: &AppState) -> Self {
        Self {
            jwt: app_state.jwt.clone(),
            auth_repo: AuthRepository::from(app_state),
            user_repo: UserRepository::from(app_state),
            redis_pool: app_state.redis_pool.clone()
        }
    }
}


impl AuthService {
    // Generate a secure 64-character random string
    fn generate_secure_token(&self) -> String {
        rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(64)
            .map(char::from)
            .collect()
    }

    // Hash the token using SHA-256
    fn hash_token(&self, raw_token: &str) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(raw_token.as_bytes());
        hasher.finalize().to_vec()
    }
}


fn random_token() -> String {
    use rand::RngCore;
    let mut b = [0u8; 32];
    rand::rngs::OsRng.fill_bytes(&mut b);
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(b)
}

fn sha256_bytes(s: &str) -> [u8; 32] {
    let mut h = Sha256::new();
    h.update(s.as_bytes());
    let out = h.finalize();
    let mut buf = [0u8; 32];
    buf.copy_from_slice(&out);
    buf
}


#[async_trait]
impl AuthServiceInterface for AuthService {
    async fn login(&self, command: LoginCommand) -> Result<Option<String>, AppError> {
        if command.email.is_empty() {
            return Err(AppError::BadRequest("Invalid email".to_string()));
        }
        if command.password.is_empty() {
            return Err(AppError::BadRequest("Invalid password".to_string()));
        }

        let user = self.user_repo.get_by_email(command.email).await?;
        match user {
            Some(user) => {
                let valid_password = verify_password(command.password.as_str(), user.password_hash.as_str());
                match valid_password {
                    Ok(valid_password) => {
                        let mut log = LoginLog::new(user.id.clone().unwrap(), Utc::now().naive_utc(), command.request_ip, command.user_agent, LoginStatus::Success, None);

                        if !valid_password {
                            log.set_status(LoginStatus::Failed, Some("Password don't match".to_string()));
                            self.auth_repo.create_login_log(log).await.map_err(AppError::Internal)?;
                            return Err(AppError::BadRequest("Password don't match".to_string()));
                        }

                        log.set_status(LoginStatus::Success, Some("Successfully Login".to_string()));
                        self.auth_repo.create_login_log(log).await.map_err(AppError::Internal)?;

                        let auth_user = AuthUser::from(user);
                        let token = self.jwt.generate_token(auth_user)?;
                        Ok(Some(token))
                    },
                    Err(_) => Err(AppError::InternalError("Error during password verification".to_string()))
                }
            },
            None => Ok(None)
        }
    }

    async fn login_alt(&self, command: LoginCommand) -> Result<Option<String>, AppError> {
        if command.email.is_empty() {
            return Err(AppError::BadRequest("Invalid email".to_string()));
        }
        if command.password.is_empty() {
            return Err(AppError::BadRequest("Invalid password".to_string()));
        }

        let user = self.user_repo.get_by_email(command.email).await?;
        match user {
            Some(user) => {
                let mut log = LoginLog::new(user.id.clone().unwrap(), Utc::now().naive_utc(), command.request_ip, command.user_agent, LoginStatus::Success, None);

                log.set_status(LoginStatus::Success, Some("Successfully Login".to_string()));
                self.auth_repo.create_login_log(log).await.map_err(AppError::Internal)?;

                let auth_user = AuthUser::from(user);
                let token = self.jwt.generate_token(auth_user)?;
                Ok(Some(token))
            },
            None => Ok(None)
        }
    }

    async fn register(&self, command: RegisterCommand) -> Result<bool, AppError> {
        let existing_user = self.user_repo.get_by_email(command.email.clone()).await?;
        
        if existing_user.is_some() {
            return Err(AppError::BadRequest("User already exists".to_string()));
        }
        
        let user_new = User::from(command);
        let _user = self.user_repo.create(user_new).await?;
        
        // TODO: send welcome mail
        
        Ok(true)
    }


    async fn request_password_reset(&self, command: CreatePasswordResetCommand) -> Result<TokenGenerationResult, AppError> {
        let raw_token = self.generate_secure_token();
        let token_hash = self.hash_token(&raw_token);

        // Tokens expire in 60 minutes
        let expires_in_minutes = 60;
        let expires_at = (Utc::now() + Duration::minutes(expires_in_minutes)).naive_utc();

        let token = PasswordResetToken {
            id: None,
            user_id: command.user_id,
            token_hash,
            expires_at,
            used_at: None,
            request_ip: command.request_ip,
            user_agent: command.user_agent,
            created_at: None,
        };

        self.auth_repo.create_password_reset(token).await.map_err(AppError::Internal)?;

        // TODO: Trigger Email outbox event here with `_token_result.raw_token`

        Ok(TokenGenerationResult { raw_token, expires_in_minutes })
    }

    async fn confirm_password_reset(&self, command: ConfirmPasswordResetCommand) -> Result<Uuid, AppError> {
        let token_hash = self.hash_token(&command.raw_token);

        let user_id = self.auth_repo.consume_password_reset(&token_hash).await
            .map_err(AppError::Internal)?
            .ok_or_else(|| AppError::BadRequest("Invalid or expired password reset token".to_string()))?;

        self.user_repo.update_password(user_id, command.new_password).await?;

        Ok(user_id)
    }

    async fn request_email_verification(&self, command: CreateEmailVerifyCommand) -> Result<TokenGenerationResult, AppError> {
        let raw_token = self.generate_secure_token();
        let token_hash = self.hash_token(&raw_token);

        // Email verifications expire in 24 hours (1440 mins)
        let expires_in_minutes = 24 * 60;
        let expires_at = (Utc::now() + Duration::minutes(expires_in_minutes)).naive_utc();

        let token = EmailVerificationToken {
            id: None,
            user_id: command.user_id,
            token_hash,
            expires_at,
            used_at: None,
            created_at: None,
        };

        self.auth_repo.create_email_verification(token).await
            .map_err(AppError::Internal)?;

        // TODO: Trigger Email outbox event here with `_token_result.raw_token`

        Ok(TokenGenerationResult { raw_token, expires_in_minutes })
    }

    async fn confirm_email_verification(&self, command: ConfirmEmailVerifyCommand) -> Result<Uuid, AppError> {
        let token_hash = self.hash_token(&command.raw_token);

        let user_id = self.auth_repo.consume_email_verification(&token_hash).await
            .map_err(AppError::Internal)?
            .ok_or_else(|| AppError::BadRequest("Invalid or expired email verification token".to_string()))?;

        self.user_repo.verify_email(user_id.clone()).await?;

        Ok(user_id)
    }





    async fn get_login_log_by_user(&self, command: LoginLogListByUserCommand) -> Result<Vec<LoginLogResponse>, AppError> {
        let (limit, offset, _search) = extract_pagination_data(command.pagination);

        let logs = self.auth_repo.get_login_log_by_user(
            command.user_id, limit, offset
        ).await.map_err(AppError::Internal)?;

        Ok(logs.into_iter().map(LoginLogResponse::from).collect())
    }

}

use anyhow::{Error, Result};
use async_trait::async_trait;
use base64::Engine;
use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use chrono::{Duration, Utc};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use uuid::Uuid;

use crate::modules::users::auth::auth_command::{ForgotPasswordCommand, LoginCommand, RegisterCommand};
use crate::modules::users::auth::auth_command::ResetPasswordCommand;
use crate::modules::users::user::user_repo::{UserRepository, UserRepositoryInterface};
use crate::shared::auth::jwt::{AuthUser, JwtVerifier};
use crate::shared::auth::password::verify_password;
use crate::shared::state::AppState;

#[async_trait]
pub trait AuthServiceInterface {

    async fn login(&self, command: LoginCommand) -> Result<Option<String>, Error>;

    async fn register(&self, command: RegisterCommand) -> Result<bool, Error>;

    async fn forgot_password(&self, command: ForgotPasswordCommand) -> Result<Option<bool>, Error>;

    async fn reset_password(&self, command: ResetPasswordCommand) -> Result<Option<bool>, Error>;

}


#[derive(Clone)]
pub struct AuthService {
    jwt: JwtVerifier,
    user_repo: UserRepository,
    redis_pool: Option<Pool<RedisConnectionManager>>,
}

impl From<&AppState> for AuthService {
    fn from(app_state: &AppState) -> Self {
        let user_repo = UserRepository::from(app_state);
        Self {
            jwt: app_state.jwt.clone(),
            user_repo,
            redis_pool: Option::from(app_state.redis_pool.clone())
        }
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
    async fn login(&self, command: LoginCommand) -> Result<Option<String>, Error> {
        if command.email.is_empty() {
            return Err(Error::msg("Invalid email"));
        }
        if command.password.is_empty() {
            return Err(Error::msg("Invalid password"));
        }

        let user = self.user_repo.get_by_email(command.email, None).await;
        match user {
            Ok(user) => {
                match user {
                    Some(user) => {
                        let valid_password = verify_password(command.password.as_str(), user.password_hash.as_str());
                        match valid_password {
                            Ok(valid_password) => {
                                if !valid_password {
                                    return Err(Error::msg("Password don't match"));
                                }

                                let auth_user = AuthUser::from(user);
                                let token = self.jwt.generate_token(auth_user)?;
                                Ok(Some(token))
                            },
                            Err(_) => Err(Error::msg("Invalid password"))
                        }
                    },
                    None => Ok(None)
                }
            },
            Err(_) => Err(Error::msg("Error during user login"))
        }
    }

    async fn register(&self, command: RegisterCommand) -> Result<bool, Error> {
        todo!()
    }

    async fn forgot_password(&self, command: ForgotPasswordCommand) -> Result<Option<bool>, Error> {
        todo!()
    }

    async fn reset_password(&self, command: ResetPasswordCommand) -> Result<Option<bool>, Error> {
        todo!()
    }

}

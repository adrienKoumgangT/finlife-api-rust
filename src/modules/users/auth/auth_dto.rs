use chrono::{DateTime, Utc};
use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use crate::modules::users::auth::auth_model::{LoginLog, LoginStatus};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct LoginRequest {
    pub email: String,
    pub password: String
}


#[derive(Debug, Serialize, Deserialize, ToSchema, IntoParams)]
pub struct RegisterRequest {
    pub email: String,

    pub first_name: String,
    pub last_name: String,

    #[param(example = "EUR")]
    pub base_currency_code: String,
}


#[derive(Debug, Serialize, Deserialize, ToSchema, IntoParams)]
pub struct ForgotPasswordRequest {
    pub email: String,
    #[param(example = "fr")]
    pub locale: Option<String>, // "fr"
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ResetPasswordRequest {
    pub token: String,
    pub new_password: String,
}


// Note: We never return the token hash in standard responses.
// The raw token is only returned internally upon generation to be emailed.

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TokenGenerationResult {
    pub raw_token: String,
    pub expires_in_minutes: i64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PasswordResetRequest {
    pub user_id: Uuid, // Usually you'd look this up by email in a real auth flow
    pub request_ip: Option<String>,
    pub user_agent: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PasswordResetConfirmRequest {
    pub raw_token: String,
    pub new_password: String, // Handled by the auth service, but passed through here
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct EmailVerifyRequest {
    pub user_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct EmailVerifyConfirmRequest {
    pub raw_token: String,
}



#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct LoginLogResponse {
    pub log_id: Uuid,
    pub user_id: Uuid,
    pub login_at: NaiveDateTime,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub status: LoginStatus,
    pub failure_reason: Option<String>,
}

impl From<LoginLog> for LoginLogResponse {
    fn from(log: LoginLog) -> Self {
        Self {
            log_id: log.id.unwrap(),
            user_id: log.user_id,
            login_at: log.login_at,
            ip_address: log.ip_address,
            user_agent: log.user_agent,
            status: log.status,
            failure_reason: log.failure_reason,
        }
    }
}

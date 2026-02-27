use chrono::{DateTime, NaiveDateTime, Utc};
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::modules::users::auth::auth_command::LoginLogCreateCommand;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct PasswordResetToken {
    pub id: Option<Uuid>,
    pub user_id: Uuid,
    pub token_hash: Vec<u8>,
    pub expires_at: NaiveDateTime,
    pub used_at: Option<NaiveDateTime>,
    pub request_ip: Option<String>,
    pub user_agent: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct EmailVerificationToken {
    pub id: Option<Uuid>,
    pub user_id: Uuid,
    pub token_hash: Vec<u8>,
    pub expires_at: NaiveDateTime,
    pub used_at: Option<NaiveDateTime>,
    pub created_at: Option<DateTime<Utc>>,
}





#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LoginStatus {
    Success,
    Failed,
}

impl From<String> for LoginStatus {
    fn from(value: String) -> Self {
        match value.to_uppercase().as_str() {
            "FAILED" => LoginStatus::Failed,
            _ => LoginStatus::Success,
        }
    }
}

impl LoginStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            LoginStatus::Success => "SUCCESS",
            LoginStatus::Failed => "FAILED",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct LoginLog {
    pub id: Option<Uuid>,
    pub user_id: Uuid,
    pub login_at: NaiveDateTime,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub status: LoginStatus,
    pub failure_reason: Option<String>,
}

impl LoginLog {
    pub fn new(user_id: Uuid, login_at: NaiveDateTime, ip_address: Option<String>, user_agent: Option<String>, status: LoginStatus, failure_reason: Option<String>) -> Self {
        Self { id: None, user_id, login_at, ip_address, user_agent, status, failure_reason }
    }
    
    pub fn set_status(&mut self, status: LoginStatus, failure_reason: Option<String>) {
        self.status = status;
        self.failure_reason = failure_reason;
    }
}

impl From<LoginLogCreateCommand> for LoginLog {
    fn from(command: LoginLogCreateCommand) -> Self {
        Self {
            id: None,
            user_id: command.user_id,
            login_at: Utc::now().naive_utc(),
            ip_address: command.ip_address,
            user_agent: command.user_agent,
            status: command.status,
            failure_reason: command.failure_reason,
        }
    }
}


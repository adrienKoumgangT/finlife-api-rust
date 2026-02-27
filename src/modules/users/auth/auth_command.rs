use uuid::Uuid;
use serde::{Serialize, Deserialize};
use crate::modules::users::auth::auth_dto::*;
use crate::modules::users::auth::auth_model::LoginStatus;
use crate::shared::auth::jwt::AuthUser;
use crate::shared::response::PaginationRequest;

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginCommand {
    pub email: String,
    pub password: String,

    pub request_ip: Option<String>,
    pub user_agent: Option<String>,
}

impl LoginCommand {
    pub fn new(email: String, password: String, request_ip: Option<String>, user_agent: Option<String>) -> Self {
        Self { email, password, request_ip, user_agent }
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterCommand {
    pub email: String,

    pub first_name: String,
    pub last_name: String,

    pub base_currency_code: String,
}

impl From<RegisterRequest> for RegisterCommand {
    fn from (request: RegisterRequest) -> Self {
        Self {
            email: request.email,
            first_name: request.first_name,
            last_name: request.last_name,
            base_currency_code: request.base_currency_code,
        }
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct ForgotPasswordCommand {
    pub email: String,
    pub locale: Option<String>, // "fr"
}

impl From<ForgotPasswordRequest> for ForgotPasswordCommand {
    fn from (request: ForgotPasswordRequest) -> Self {
        Self { email: request.email, locale: request.locale }
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct ResetPasswordCommand {
    pub token: String,
    pub new_password: String,
}

impl From<ResetPasswordRequest> for ResetPasswordCommand {
    fn from (request: ResetPasswordRequest) -> Self {
        Self { token: request.token, new_password: request.new_password }
    }
}



#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePasswordResetCommand {
    pub user_id: Uuid,
    pub request_ip: Option<String>,
    pub user_agent: Option<String>,
}


impl CreatePasswordResetCommand {
    pub fn new(user_id: Uuid, request_ip: Option<String>, user_agent: Option<String>) -> Self {
        Self { user_id, request_ip, user_agent }
    }
}

impl From<PasswordResetRequest> for CreatePasswordResetCommand {
    fn from(req: PasswordResetRequest) -> Self {
        Self {
            user_id: req.user_id,
            request_ip: req.request_ip,
            user_agent: req.user_agent,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfirmPasswordResetCommand {
    pub raw_token: String,
    pub new_password: String,
}

impl From<PasswordResetConfirmRequest> for ConfirmPasswordResetCommand {
    fn from(req: PasswordResetConfirmRequest) -> Self {
        Self {
            raw_token: req.raw_token,
            new_password: req.new_password,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateEmailVerifyCommand {
    pub user_id: Uuid,
}

impl From<EmailVerifyRequest> for CreateEmailVerifyCommand {
    fn from(req: EmailVerifyRequest) -> Self {
        Self { user_id: req.user_id }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfirmEmailVerifyCommand {
    pub raw_token: String,
}

impl From<EmailVerifyConfirmRequest> for ConfirmEmailVerifyCommand {
    fn from(req: EmailVerifyConfirmRequest) -> Self {
        Self { raw_token: req.raw_token }
    }
}




#[derive(Debug, Serialize, Deserialize)]
pub struct LoginLogCreateCommand {
    pub user_id: Uuid,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub status: LoginStatus,
    pub failure_reason: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginLogListByUserCommand {
    pub user_id: Uuid,
    pub pagination: Option<PaginationRequest>,
    pub auth_user: AuthUser,
}

impl LoginLogListByUserCommand {
    pub fn new(user_id: Uuid, pagination: Option<PaginationRequest>, auth_user: AuthUser) -> Self {
        Self { user_id, pagination, auth_user }
    }
}

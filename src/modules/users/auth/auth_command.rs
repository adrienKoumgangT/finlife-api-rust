use uuid::Uuid;
use serde::{Serialize, Deserialize};
use crate::modules::users::auth::auth_dto::{ForgotPasswordRequest, LoginRequest, RegisterRequest, ResetPasswordRequest};

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginCommand {
    pub email: String,
    pub password: String
}

impl From<LoginRequest> for LoginCommand {
    fn from(login_request: LoginRequest) -> Self {
        Self {
            email: login_request.email,
            password: login_request.password
        }
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterCommand {
    pub user_email: String,

    pub user_first_name: String,
    pub user_last_name: String,

    pub user_base_currency_code: String,
}

impl From<RegisterRequest> for RegisterCommand {
    fn from (request: RegisterRequest) -> Self {
        Self {
            user_email: request.email,
            user_first_name: request.first_name,
            user_last_name: request.last_name,
            user_base_currency_code: request.base_currency_code,
        }
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct ForgotPasswordCommand {
    pub user_email: String,
    pub locale: Option<String>, // "fr"
}

impl From<ForgotPasswordRequest> for ForgotPasswordCommand {
    fn from (request: ForgotPasswordRequest) -> Self {
        Self { user_email: request.email, locale: request.locale }
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


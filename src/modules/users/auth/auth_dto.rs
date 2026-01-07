use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;


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

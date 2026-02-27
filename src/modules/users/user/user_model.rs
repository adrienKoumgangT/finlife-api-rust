use chrono::{DateTime, Utc};
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::modules::users::{
    auth::auth_command::RegisterCommand,
    user::user_command::UserCreateCommand
};
use crate::shared::{
    auth::password::generate_password,
};


#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type, ToSchema)]
pub enum UserRole {
    ADMIN,
    USER,
}

impl UserRole {
    pub fn is_admin(&self) -> bool {
        *self == UserRole::ADMIN
    }
}

impl From<String> for UserRole {
    fn from(value: String) -> Self {
        match value.as_str() {
            "ADMIN" => UserRole::ADMIN,
            _ => UserRole::USER,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: Option<Uuid>,

    pub email: String,
    pub email_verified: bool,
    pub password_hash: String,

    pub role: UserRole,

    pub first_name: String,
    pub last_name: String,
    pub base_currency_code: String,

    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl From<UserCreateCommand> for User {
    fn from(command: UserCreateCommand) -> Self {
        Self {
            id: None,
            email: command.email,
            email_verified: command.email_verified,
            password_hash: generate_password(12),
            role: UserRole::USER,
            first_name: command.first_name,
            last_name: command.last_name,
            base_currency_code: command.base_currency_code,
            created_at: None,
            updated_at: None,
        }
    }
}

impl From<RegisterCommand> for User {
    fn from(command: RegisterCommand) -> Self {
        Self {
            id: None,
            email: command.email,
            email_verified: false,
            password_hash: generate_password(12),
            role: UserRole::USER,
            first_name: command.first_name,
            last_name: command.last_name,
            base_currency_code: command.base_currency_code,
            created_at: None,
            updated_at: None,
        }
    }
}

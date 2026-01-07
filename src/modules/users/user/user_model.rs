use chrono::{DateTime, Utc};
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use sqlx::{mysql::MySqlRow, Error as SqlxError, Row};
use std::collections::HashMap;
use utoipa::ToSchema;

use crate::modules::users::{
    auth::auth_command::RegisterCommand,
    user::user_command::UserCreateCommand
};
use crate::shared::{
    auth::password::generate_password,
    db::mysql::FromSqlRow
};


#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type, ToSchema)]
pub enum UserRole {
    // GOD,
    ADMIN,
    USER,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: Option<Vec<u8>>,

    pub email: String,
    pub password_hash: String,

    pub role: UserRole,

    pub first_name: String,
    pub last_name: String,
    pub base_currency_code: String,

    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl FromSqlRow for User {
    fn map_row_to_entity(row: MySqlRow, index_map: &HashMap<String, usize>) -> Result<Self, SqlxError> {
        Ok(User {
            id: row.try_get(index_map["id"])?,
            email: row.try_get(index_map["email"])?,
            password_hash: row.try_get(index_map["password_hash"])?,
            role: row.try_get(index_map["role"])?,
            first_name: row.try_get(index_map["first_name"])?,
            last_name: row.try_get(index_map["last_name"])?,
            base_currency_code: row.try_get(index_map["base_currency_code"])?,
            created_at: row.try_get(index_map["created_at"])?,
            updated_at: row.try_get(index_map["updated_at"])?,
        })
    }
}

impl From<UserCreateCommand> for User {
    fn from(command: UserCreateCommand) -> Self {
        Self {
            id: None,
            email: command.user_email,
            password_hash: generate_password(12),
            role: UserRole::USER,
            first_name: command.user_first_name,
            last_name: command.user_last_name,
            base_currency_code: command.user_base_currency_code,
            created_at: None,
            updated_at: None,
        }
    }
}

impl From<RegisterCommand> for User {
    fn from(command: RegisterCommand) -> Self {
        Self {
            id: None,
            email: command.user_email,
            password_hash: generate_password(12),
            role: UserRole::USER,
            first_name: command.user_first_name,
            last_name: command.user_last_name,
            base_currency_code: command.user_base_currency_code,
            created_at: None,
            updated_at: None,
        }
    }
}

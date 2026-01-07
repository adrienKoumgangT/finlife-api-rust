use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use crate::modules::users::user::user_model::User;
use crate::shared::utils::bu;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserResponse {
    pub user_id: Uuid,

    pub user_email: String,

    pub user_first_name: String,
    pub user_last_name: String,

    pub user_base_currency_code: String,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            user_id: bu(user.id.unwrap().as_slice()),
            user_email: user.email,
            user_first_name: user.first_name,
            user_last_name: user.last_name,
            user_base_currency_code: user.base_currency_code,
        }
    }
}

impl From<&User> for UserResponse {
    fn from(user: &User) -> Self {
        Self {
            user_id: bu(user.id.clone().unwrap().as_slice()),
            user_email: user.email.clone(),
            user_first_name: user.first_name.clone(),
            user_last_name: user.last_name.clone(),
            user_base_currency_code: user.base_currency_code.clone(),
        }
    }
}


#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserCreateRequest {
    pub user_email: String,

    pub user_first_name: String,
    pub user_last_name: String,

    pub user_base_currency_code: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserUpdateNameRequest {
    pub user_first_name: String,
    pub user_last_name: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserUpdateBaseCurrencyRequest {
    pub user_base_currency_code: String,
}

use serde::{Serialize, Deserialize};
use utoipa::{ToSchema};
use uuid::Uuid;

use crate::modules::users::user::user_model::User;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserResponse {
    pub user_id: Uuid,

    pub email: String,

    pub first_name: String,
    pub last_name: String,

    pub base_currency_code: String,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            user_id: user.id.unwrap(),
            email: user.email,
            first_name: user.first_name,
            last_name: user.last_name,
            base_currency_code: user.base_currency_code,
        }
    }
}

impl From<&User> for UserResponse {
    fn from(user: &User) -> Self {
        Self {
            user_id: user.id.clone().unwrap(),
            email: user.email.clone(),
            first_name: user.first_name.clone(),
            last_name: user.last_name.clone(),
            base_currency_code: user.base_currency_code.clone(),
        }
    }
}


#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserCreateRequest {
    pub email: String,
    pub email_verified: bool,

    pub first_name: String,
    pub last_name: String,

    pub base_currency_code: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserUpdateNameRequest {
    pub first_name: String,
    pub last_name: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserUpdateBaseCurrencyRequest {
    pub base_currency_code: String,
}

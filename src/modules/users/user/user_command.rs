use uuid::Uuid;
use serde::{Serialize, Deserialize};

use crate::modules::users::user::user_dto::{UserCreateRequest, UserUpdateBaseCurrencyRequest, UserUpdateNameRequest};
use crate::shared::auth::jwt::AuthUser;
use crate::shared::response::PaginationRequest;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserGetCommand {
    pub user_id: Uuid,
    
    pub auth_user: AuthUser,
}

impl UserGetCommand {
    pub fn new(user_id: Uuid, auth_user: AuthUser) -> Self {
        Self { user_id, auth_user }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserGetByEmailCommand {
    pub user_email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserCreateCommand {
    pub email: String,
    pub email_verified: bool,

    pub first_name: String,
    pub last_name: String,

    pub base_currency_code: String,

    pub auth_user: AuthUser,
}

impl UserCreateCommand {
    pub fn new(request: UserCreateRequest, auth_user: AuthUser) -> Self {
        Self {
            email: request.email,
            email_verified: request.email_verified,
            first_name: request.first_name,
            last_name: request.last_name,
            base_currency_code: request.base_currency_code,
            auth_user,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserVerifyEmailCommand {
    pub user_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserUpdatePasswordCommand {
    pub user_id: Uuid,

    pub new_password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserUpdateNameCommand {
    pub user_id: Uuid,

    pub first_name: String,
    pub last_name: String,
    
    pub auth_user: AuthUser,
}

impl UserUpdateNameCommand {
    pub fn new(user_id: Uuid, user_update_request: UserUpdateNameRequest, auth_user: AuthUser) -> Self {
        Self {
            user_id,
            first_name: user_update_request.first_name,
            last_name: user_update_request.last_name,
            auth_user,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserUpdateBaseCurrencyCommand {
    pub user_id: Uuid,

    pub base_currency_code: String,
    
    pub auth_user: AuthUser,
}

impl UserUpdateBaseCurrencyCommand {
    pub fn new(user_id: Uuid, user_update_base_currency_request: UserUpdateBaseCurrencyRequest, auth_user: AuthUser) -> Self {
        Self { 
            user_id,
            base_currency_code: user_update_base_currency_request.base_currency_code,
            auth_user 
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserDeleteCommand {
    pub user_id: Uuid,

    pub auth_user: AuthUser,
}

impl UserDeleteCommand {
    pub fn new(user_id: Uuid, auth_user: AuthUser) -> Self {
        Self { user_id, auth_user }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserListCommand {
    pub pagination: Option<PaginationRequest>,

    pub auth_user: AuthUser,
}

impl UserListCommand {
    pub fn new(pagination: PaginationRequest, auth_user: AuthUser) -> Self {
        Self { pagination: Some(pagination), auth_user }
    }
}



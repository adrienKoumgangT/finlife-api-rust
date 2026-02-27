use uuid::Uuid;
use serde::{Serialize, Deserialize};

use crate::modules::accounts::account_dto::*;
use crate::modules::accounts::account_model::AccountType;
use crate::shared::auth::jwt::AuthUser;
use crate::shared::response::PaginationRequest;

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountGetCommand {
    pub account_id: Uuid,
    pub auth_user: AuthUser,
}

impl AccountGetCommand {
    pub fn new(account_id: Uuid, auth_user: AuthUser) -> Self {
        Self { account_id, auth_user }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountCreateCommand {
    pub user_id: Uuid,
    pub name: String,
    pub account_type: AccountType,
    pub currency_code: String,
    pub institution: Option<String>,
    pub auth_user: AuthUser,
}

impl AccountCreateCommand {
    pub fn new(request: AccountCreateRequest, auth_user: AuthUser) -> Self {
        Self {
            user_id: auth_user.user_id,
            name: request.name,
            account_type: request.account_type,
            currency_code: request.currency_code,
            institution: request.institution,
            auth_user,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountUpdateCommand {
    pub account_id: Uuid,
    pub name: String,
    pub account_type: AccountType,
    pub institution: Option<String>,
    pub auth_user: AuthUser,
}

impl AccountUpdateCommand {
    pub fn new(account_id: Uuid, request: AccountUpdateRequest, auth_user: AuthUser) -> Self {
        Self {
            account_id,
            name: request.name,
            account_type: request.account_type,
            institution: request.institution,
            auth_user,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountArchivedCommand {
    pub account_id: Uuid,
    pub archived: bool,
    pub auth_user: AuthUser,
}

impl AccountArchivedCommand {
    pub fn new(account_id: Uuid, request: AccountUpdateArchivedRequest, auth_user: AuthUser) -> Self {
        Self {
            account_id,
            archived: request.archived,
            auth_user,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountDeleteCommand {
    pub account_id: Uuid,
    pub auth_user: AuthUser,
}

impl AccountDeleteCommand {
    pub fn new(account_id: Uuid, auth_user: AuthUser) -> Self {
        Self { account_id, auth_user }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountListByUserCommand {
    pub user_id: Uuid,
    pub pagination: Option<PaginationRequest>,
    pub auth_user: AuthUser,
}

impl AccountListByUserCommand {
    pub fn new(user_id: Uuid, pagination: Option<PaginationRequest>, auth_user: AuthUser) -> Self {
        Self { user_id, pagination, auth_user }
    }
}

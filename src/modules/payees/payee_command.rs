use uuid::Uuid;
use serde::{Serialize, Deserialize};

use crate::modules::payees::payee_dto::*;
use crate::shared::auth::jwt::AuthUser;
use crate::shared::response::PaginationRequest;

#[derive(Debug, Serialize, Deserialize)]
pub struct PayeeGetCommand {
    pub payee_id: Uuid,
    pub auth_user: AuthUser,
}

impl PayeeGetCommand {
    pub fn new(payee_id: Uuid, auth_user: AuthUser) -> Self {
        Self { payee_id, auth_user }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PayeeCreateCommand {
    pub user_id: Uuid,
    pub name: String,
    pub auth_user: AuthUser,
}

impl PayeeCreateCommand {
    pub fn new(request: PayeeCreateRequest, auth_user: AuthUser) -> Self {
        Self {
            user_id: auth_user.user_id,
            name: request.name,
            auth_user,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PayeeUpdateCommand {
    pub payee_id: Uuid,
    pub name: String,
    pub auth_user: AuthUser,
}

impl PayeeUpdateCommand {
    pub fn new(payee_id: Uuid, request: PayeeUpdateRequest, auth_user: AuthUser) -> Self {
        Self {
            payee_id,
            name: request.name,
            auth_user,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PayeeDeleteCommand {
    pub payee_id: Uuid,
    pub auth_user: AuthUser,
}

impl PayeeDeleteCommand {
    pub fn new(payee_id: Uuid, auth_user: AuthUser) -> Self {
        Self { payee_id, auth_user }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PayeeListByUserCommand {
    pub user_id: Uuid,
    pub pagination: Option<PaginationRequest>,
    pub auth_user: AuthUser,
}

impl PayeeListByUserCommand {
    pub fn new(user_id: Uuid, pagination: Option<PaginationRequest>, auth_user: AuthUser) -> Self {
        Self { user_id, pagination, auth_user }
    }
}

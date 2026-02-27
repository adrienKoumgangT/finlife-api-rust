use uuid::Uuid;
use serde::{Serialize, Deserialize};

use crate::modules::categories::category_dto::*;
use crate::modules::categories::category_model::CategoryKind;
use crate::shared::auth::jwt::AuthUser;
use crate::shared::response::PaginationRequest;

#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryGetCommand {
    pub category_id: Uuid,
    pub auth_user: AuthUser,
}

impl CategoryGetCommand {
    pub fn new(category_id: Uuid, auth_user: AuthUser) -> Self {
        Self { category_id, auth_user }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryCreateCommand {
    pub user_id: Uuid,
    pub name: String,
    pub category_kind: CategoryKind,
    pub parent_id: Option<Uuid>,
    pub sort_order: Option<i32>,
    pub auth_user: AuthUser,
}

impl CategoryCreateCommand {
    pub fn new(request: CategoryCreateRequest, auth_user: AuthUser) -> Self {
        Self {
            user_id: auth_user.user_id,
            name: request.name,
            category_kind: request.kind,
            parent_id: request.parent_id,
            sort_order: request.sort_order,
            auth_user,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryUpdateCommand {
    pub category_id: Uuid,
    pub name: String,
    pub parent_id: Option<Uuid>,
    pub sort_order: Option<i32>,
    pub auth_user: AuthUser,
}

impl CategoryUpdateCommand {
    pub fn new(category_id: Uuid, request: CategoryUpdateRequest, auth_user: AuthUser) -> Self {
        Self {
            category_id,
            name: request.name,
            parent_id: request.parent_id,
            sort_order: request.sort_order,
            auth_user,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryArchivedCommand {
    pub category_id: Uuid,
    pub archived: bool,
    pub auth_user: AuthUser,
}

impl CategoryArchivedCommand {
    pub fn new(category_id: Uuid, request: CategoryUpdateArchivedRequest, auth_user: AuthUser) -> Self {
        Self {
            category_id,
            archived: request.archived,
            auth_user,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryDeleteCommand {
    pub category_id: Uuid,
    pub auth_user: AuthUser,
}

impl CategoryDeleteCommand {
    pub fn new(category_id: Uuid, auth_user: AuthUser) -> Self {
        Self { category_id, auth_user }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryListByUserCommand {
    pub user_id: Uuid,
    pub pagination: Option<PaginationRequest>,
    pub auth_user: AuthUser,
}

impl CategoryListByUserCommand {
    pub fn new(user_id: Uuid, pagination: Option<PaginationRequest>, auth_user: AuthUser) -> Self {
        Self { user_id, pagination, auth_user }
    }
}

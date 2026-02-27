use chrono::NaiveDate;
use uuid::Uuid;
use serde::{Serialize, Deserialize};

use crate::modules::goals::goal_dto::*;
use crate::modules::goals::goal_model::GoalType;
use crate::shared::auth::jwt::AuthUser;
use crate::shared::response::PaginationRequest;

#[derive(Debug, Serialize, Deserialize)]
pub struct GoalGetCommand {
    pub goal_id: Uuid,
    pub auth_user: AuthUser,
}

impl GoalGetCommand {
    pub fn new(goal_id: Uuid, auth_user: AuthUser) -> Self {
        Self { goal_id, auth_user }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GoalCreateCommand {
    pub user_id: Uuid,
    pub name: String,
    pub goal_type: GoalType,
    pub target_base_minor: Option<i64>,
    pub target_date: Option<NaiveDate>,
    pub priority: Option<i32>,
    pub linked_account_id: Option<Uuid>,
    pub auth_user: AuthUser,
}

impl GoalCreateCommand {
    pub fn new(request: GoalCreateRequest, auth_user: AuthUser) -> Self {
        Self {
            user_id: auth_user.user_id,
            name: request.name,
            goal_type: request.goal_type,
            target_base_minor: request.target_base_minor,
            target_date: request.target_date,
            priority: request.priority,
            linked_account_id: request.linked_account_id,
            auth_user,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GoalUpdateCommand {
    pub goal_id: Uuid,
    pub name: String,
    pub goal_type: GoalType,
    pub target_base_minor: i64,
    pub target_date: Option<NaiveDate>,
    pub priority: i32,
    pub linked_account_id: Option<Uuid>,
    pub auth_user: AuthUser,
}

impl GoalUpdateCommand {
    pub fn new(goal_id: Uuid, request: GoalUpdateRequest, auth_user: AuthUser) -> Self {
        Self {
            goal_id,
            name: request.name,
            goal_type: request.goal_type,
            target_base_minor: request.target_base_minor.unwrap_or(0),
            target_date: request.target_date,
            priority: request.priority.unwrap_or(0),
            linked_account_id: request.linked_account_id,
            auth_user,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GoalDeleteCommand {
    pub goal_id: Uuid,
    pub auth_user: AuthUser,
}

impl GoalDeleteCommand {
    pub fn new(goal_id: Uuid, auth_user: AuthUser) -> Self {
        Self { goal_id, auth_user }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GoalListByUserCommand {
    pub user_id: Uuid,
    pub pagination: Option<PaginationRequest>,
    pub auth_user: AuthUser,
}

impl GoalListByUserCommand {
    pub fn new(user_id: Uuid, pagination: Option<PaginationRequest>, auth_user: AuthUser) -> Self {
        Self { user_id, pagination, auth_user }
    }
}

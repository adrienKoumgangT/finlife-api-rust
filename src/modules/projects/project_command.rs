use chrono::NaiveDate;
use uuid::Uuid;
use serde::{Serialize, Deserialize};

use crate::modules::projects::project_dto::*;
use crate::modules::projects::project_model::ProjectStatus;
use crate::shared::auth::jwt::AuthUser;
use crate::shared::response::PaginationRequest;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectGetCommand {
    pub project_id: Uuid,
    pub auth_user: AuthUser,
}

impl ProjectGetCommand {
    pub fn new(project_id: Uuid, auth_user: AuthUser) -> Self {
        Self { project_id, auth_user }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectCreateCommand {
    pub user_id: Uuid,
    pub name: String,
    pub status: Option<ProjectStatus>,
    pub priority: Option<i32>,
    pub start_date: Option<NaiveDate>,
    pub due_date: Option<NaiveDate>,
    pub budget_base_minor: Option<i64>,
    pub goal_id: Option<Uuid>,
    pub person_id: Option<Uuid>,
    pub location_id: Option<Uuid>,
    pub description: Option<String>,
    pub auth_user: AuthUser,
}

impl ProjectCreateCommand {
    pub fn new(request: ProjectCreateRequest, auth_user: AuthUser) -> Self {
        Self {
            user_id: auth_user.user_id,
            name: request.name,
            status: request.status,
            priority: request.priority,
            start_date: request.start_date,
            due_date: request.due_date,
            budget_base_minor: request.budget_base_minor,
            goal_id: request.goal_id,
            person_id: request.person_id,
            location_id: request.location_id,
            description: request.description,
            auth_user,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectUpdateCommand {
    pub project_id: Uuid,
    pub name: String,
    pub status: ProjectStatus,
    pub priority: i32,
    pub start_date: Option<NaiveDate>,
    pub due_date: Option<NaiveDate>,
    pub budget_base_minor: i64,
    pub goal_id: Option<Uuid>,
    pub person_id: Option<Uuid>,
    pub location_id: Option<Uuid>,
    pub description: Option<String>,
    pub auth_user: AuthUser,
}

impl ProjectUpdateCommand {
    pub fn new(project_id: Uuid, request: ProjectUpdateRequest, auth_user: AuthUser) -> Self {
        Self {
            project_id,
            name: request.project_name,
            status: request.project_status,
            priority: request.priority.unwrap_or(0),
            start_date: request.start_date,
            due_date: request.due_date,
            budget_base_minor: request.budget_base_minor.unwrap_or(0),
            goal_id: request.goal_id,
            person_id: request.person_id,
            location_id: request.location_id,
            description: request.description,
            auth_user,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectDeleteCommand {
    pub project_id: Uuid,
    pub auth_user: AuthUser,
}

impl ProjectDeleteCommand {
    pub fn new(project_id: Uuid, auth_user: AuthUser) -> Self {
        Self { project_id, auth_user }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectListByUserCommand {
    pub user_id: Uuid,
    pub pagination: Option<PaginationRequest>,
    pub auth_user: AuthUser,
}

impl ProjectListByUserCommand {
    pub fn new(user_id: Uuid, pagination: Option<PaginationRequest>, auth_user: AuthUser) -> Self {
        Self { user_id, pagination, auth_user }
    }
}

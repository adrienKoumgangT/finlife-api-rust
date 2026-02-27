use chrono::NaiveDate;
use uuid::Uuid;
use serde::{Serialize, Deserialize};

use crate::modules::projects::project_task_dto::*;
use crate::modules::projects::project_task_model::TaskStatus;
use crate::shared::auth::jwt::AuthUser;
use crate::shared::response::PaginationRequest;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectTaskGetCommand {
    pub task_id: Uuid,
    pub auth_user: AuthUser,
}

impl ProjectTaskGetCommand {
    pub fn new(task_id: Uuid, auth_user: AuthUser) -> Self {
        Self { task_id, auth_user }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectTaskCreateCommand {
    pub project_id: Uuid,
    pub title: String,
    pub status: Option<TaskStatus>,
    pub due_date: Option<NaiveDate>,
    pub parent_task_id: Option<Uuid>,
    pub order_idx: Option<i32>,
    pub estimate_minutes: Option<i32>,
    pub actual_minutes: Option<i32>,
    pub assigned_person_id: Option<Uuid>,
    pub location_id: Option<Uuid>,
    pub note: Option<String>,
    pub auth_user: AuthUser,
}

impl ProjectTaskCreateCommand {
    pub fn new(request: ProjectTaskCreateRequest, auth_user: AuthUser) -> Self {
        Self {
            project_id: request.project_id,
            title: request.title,
            status: request.status,
            due_date: request.due_date,
            parent_task_id: request.parent_task_id,
            order_idx: request.order_idx,
            estimate_minutes: request.estimate_minutes,
            actual_minutes: request.actual_minutes,
            assigned_person_id: request.assigned_person_id,
            location_id: request.location_id,
            note: request.note,
            auth_user,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectTaskUpdateCommand {
    pub task_id: Uuid,
    pub title: String,
    pub status: TaskStatus,
    pub due_date: Option<NaiveDate>,
    pub parent_task_id: Option<Uuid>,
    pub order_idx: i32,
    pub estimate_minutes: Option<i32>,
    pub actual_minutes: Option<i32>,
    pub assigned_person_id: Option<Uuid>,
    pub location_id: Option<Uuid>,
    pub note: Option<String>,
    pub auth_user: AuthUser,
}

impl ProjectTaskUpdateCommand {
    pub fn new(task_id: Uuid, request: ProjectTaskUpdateRequest, auth_user: AuthUser) -> Self {
        Self {
            task_id,
            title: request.title,
            status: request.status,
            due_date: request.due_date,
            parent_task_id: request.parent_task_id,
            order_idx: request.order_idx.unwrap_or(0),
            estimate_minutes: request.estimate_minutes,
            actual_minutes: request.actual_minutes,
            assigned_person_id: request.assigned_person_id,
            location_id: request.location_id,
            note: request.note,
            auth_user,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectTaskDeleteCommand {
    pub task_id: Uuid,
    pub auth_user: AuthUser,
}

impl ProjectTaskDeleteCommand {
    pub fn new(task_id: Uuid, auth_user: AuthUser) -> Self {
        Self { task_id, auth_user }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectTaskListByProjectCommand {
    pub project_id: Uuid,
    pub pagination: Option<PaginationRequest>,
    pub auth_user: AuthUser,
}

impl ProjectTaskListByProjectCommand {
    pub fn new(project_id: Uuid, pagination: Option<PaginationRequest>, auth_user: AuthUser) -> Self {
        Self { project_id, pagination, auth_user }
    }
}

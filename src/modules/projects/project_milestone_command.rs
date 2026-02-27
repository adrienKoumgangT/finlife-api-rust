use chrono::NaiveDate;
use uuid::Uuid;
use serde::{Serialize, Deserialize};

use crate::modules::projects::project_milestone_dto::*;
use crate::modules::projects::project_milestone_model::MilestoneStatus;
use crate::shared::auth::jwt::AuthUser;
use crate::shared::response::PaginationRequest;


#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectMilestoneGetCommand {
    pub milestone_id: Uuid,
    pub auth_user: AuthUser,
}

impl ProjectMilestoneGetCommand {
    pub fn new(milestone_id: Uuid, auth_user: AuthUser) -> Self {
        Self { milestone_id, auth_user }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectMilestoneCreateCommand {
    pub project_id: Uuid,
    pub title: String,
    pub due_date: Option<NaiveDate>,
    pub status: Option<MilestoneStatus>,
    pub person_id: Option<Uuid>,
    pub location_id: Option<Uuid>,
    pub note: Option<String>,
    pub auth_user: AuthUser,
}

impl ProjectMilestoneCreateCommand {
    pub fn new(request: ProjectMilestoneCreateRequest, auth_user: AuthUser) -> Self {
        Self {
            project_id: request.project_id,
            title: request.title,
            due_date: request.due_date,
            status: request.status,
            person_id: request.person_id,
            location_id: request.location_id,
            note: request.note,
            auth_user,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectMilestoneUpdateCommand {
    pub milestone_id: Uuid,
    pub title: String,
    pub due_date: Option<NaiveDate>,
    pub status: MilestoneStatus,
    pub person_id: Option<Uuid>,
    pub location_id: Option<Uuid>,
    pub note: Option<String>,
    pub auth_user: AuthUser,
}

impl ProjectMilestoneUpdateCommand {
    pub fn new(milestone_id: Uuid, request: ProjectMilestoneUpdateRequest, auth_user: AuthUser) -> Self {
        Self {
            milestone_id,
            title: request.title,
            due_date: request.due_date,
            status: request.status,
            person_id: request.person_id,
            location_id: request.location_id,
            note: request.note,
            auth_user,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectMilestoneDeleteCommand {
    pub milestone_id: Uuid,
    pub auth_user: AuthUser,
}

impl ProjectMilestoneDeleteCommand {
    pub fn new(milestone_id: Uuid, auth_user: AuthUser) -> Self {
        Self { milestone_id, auth_user }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectMilestoneListByProjectCommand {
    pub project_id: Uuid,
    pub pagination: Option<PaginationRequest>,
    pub auth_user: AuthUser,
}

impl ProjectMilestoneListByProjectCommand {
    pub fn new(project_id: Uuid, pagination: Option<PaginationRequest>, auth_user: AuthUser) -> Self {
        Self { project_id, pagination, auth_user }
    }
}

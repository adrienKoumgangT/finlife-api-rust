use chrono::{DateTime, NaiveDate, Utc};
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::modules::projects::project_model::{Project, ProjectStatus};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ProjectResponse {
    pub project_id: Uuid,
    pub user_id: Uuid,

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

    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl From<Project> for ProjectResponse {
    fn from(project: Project) -> Self {
        Self {
            project_id: project.id.unwrap(),
            user_id: project.user_id,
            name: project.name,
            status: project.status,
            priority: project.priority,
            start_date: project.start_date,
            due_date: project.due_date,
            budget_base_minor: project.budget_base_minor,
            goal_id: project.goal_id,
            person_id: project.person_id,
            location_id: project.location_id,
            description: project.description,
            created_at: project.created_at,
            updated_at: project.updated_at,
        }
    }
}

impl From<&Project> for ProjectResponse {
    fn from(project: &Project) -> Self {
        Self {
            project_id: project.id.clone().unwrap(),
            user_id: project.user_id.clone(),
            name: project.name.clone(),
            status: project.status.clone(),
            priority: project.priority,
            start_date: project.start_date.clone(),
            due_date: project.due_date.clone(),
            budget_base_minor: project.budget_base_minor,
            goal_id: project.goal_id.clone(),
            person_id: project.person_id.clone(),
            location_id: project.location_id.clone(),
            description: project.description.clone(),
            created_at: project.created_at.clone(),
            updated_at: project.updated_at.clone(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ProjectCreateRequest {
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
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ProjectUpdateRequest {
    pub project_name: String,
    pub project_status: ProjectStatus,
    pub priority: Option<i32>,
    pub start_date: Option<NaiveDate>,
    pub due_date: Option<NaiveDate>,
    pub budget_base_minor: Option<i64>,
    pub goal_id: Option<Uuid>,
    pub person_id: Option<Uuid>,
    pub location_id: Option<Uuid>,
    pub description: Option<String>,
}

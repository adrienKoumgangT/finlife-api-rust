use chrono::{DateTime, NaiveDate, Utc};
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::modules::projects::project_command::ProjectCreateCommand;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum ProjectStatus {
    Planned,
    Active,
    Paused,
    Done,
    Cancelled,
}

impl From<String> for ProjectStatus {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_str() {
            "active" => ProjectStatus::Active,
            "paused" => ProjectStatus::Paused,
            "done" => ProjectStatus::Done,
            "cancelled" => ProjectStatus::Cancelled,
            _ => ProjectStatus::Planned, // Default fallback
        }
    }
}

impl ProjectStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            ProjectStatus::Planned => "planned",
            ProjectStatus::Active => "active",
            ProjectStatus::Paused => "paused",
            ProjectStatus::Done => "done",
            ProjectStatus::Cancelled => "cancelled",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Project {
    pub id: Option<Uuid>,
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

impl From<ProjectCreateCommand> for Project {
    fn from(command: ProjectCreateCommand) -> Self {
        Self {
            id: None,
            user_id: command.user_id,
            name: command.name,
            status: command.status.unwrap_or(ProjectStatus::Planned),
            priority: command.priority.unwrap_or(0),
            start_date: command.start_date,
            due_date: command.due_date,
            budget_base_minor: command.budget_base_minor.unwrap_or(0),
            goal_id: command.goal_id,
            person_id: command.person_id,
            location_id: command.location_id,
            description: command.description,
            created_at: None,
            updated_at: None,
        }
    }
}

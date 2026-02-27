use chrono::{DateTime, NaiveDate, Utc};
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::modules::projects::project_milestone_command::ProjectMilestoneCreateCommand;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum MilestoneStatus {
    Planned,
    Done,
    Cancelled,
}

impl From<String> for MilestoneStatus {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_str() {
            "done" => MilestoneStatus::Done,
            "cancelled" => MilestoneStatus::Cancelled,
            _ => MilestoneStatus::Planned, // Default fallback
        }
    }
}

impl MilestoneStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            MilestoneStatus::Planned => "planned",
            MilestoneStatus::Done => "done",
            MilestoneStatus::Cancelled => "cancelled",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ProjectMilestone {
    pub id: Option<Uuid>,
    pub project_id: Uuid,

    pub title: String,
    pub due_date: Option<NaiveDate>,
    pub status: MilestoneStatus,

    pub person_id: Option<Uuid>,
    pub location_id: Option<Uuid>,
    pub note: Option<String>,

    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl From<ProjectMilestoneCreateCommand> for ProjectMilestone {
    fn from(command: ProjectMilestoneCreateCommand) -> Self {
        Self {
            id: None,
            project_id: command.project_id,
            title: command.title,
            due_date: command.due_date,
            status: command.status.unwrap_or(MilestoneStatus::Planned),
            person_id: command.person_id,
            location_id: command.location_id,
            note: command.note,
            created_at: None,
            updated_at: None,
        }
    }
}

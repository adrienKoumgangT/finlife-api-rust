use chrono::{DateTime, NaiveDate, Utc};
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::modules::projects::project_task_command::ProjectTaskCreateCommand;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum TaskStatus {
    Todo,
    Doing,
    Done,
}

impl From<String> for TaskStatus {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_str() {
            "doing" => TaskStatus::Doing,
            "done" => TaskStatus::Done,
            _ => TaskStatus::Todo,
        }
    }
}

impl TaskStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            TaskStatus::Todo => "todo",
            TaskStatus::Doing => "doing",
            TaskStatus::Done => "done",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ProjectTask {
    pub id: Option<Uuid>,
    pub project_id: Uuid,

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

    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl From<ProjectTaskCreateCommand> for ProjectTask {
    fn from(command: ProjectTaskCreateCommand) -> Self {
        Self {
            id: None,
            project_id: command.project_id,
            title: command.title,
            status: command.status.unwrap_or(TaskStatus::Todo),
            due_date: command.due_date,
            parent_task_id: command.parent_task_id,
            order_idx: command.order_idx.unwrap_or(0),
            estimate_minutes: command.estimate_minutes,
            actual_minutes: command.actual_minutes,
            assigned_person_id: command.assigned_person_id,
            location_id: command.location_id,
            note: command.note,
            created_at: None,
            updated_at: None,
        }
    }
}

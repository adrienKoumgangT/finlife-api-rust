use chrono::{DateTime, NaiveDate, Utc};
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::modules::projects::project_task_model::{ProjectTask, TaskStatus};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ProjectTaskResponse {
    pub task_id: Uuid,
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
    pub task_created_at: Option<DateTime<Utc>>,
    pub task_updated_at: Option<DateTime<Utc>>,
}

impl From<ProjectTask> for ProjectTaskResponse {
    fn from(task: ProjectTask) -> Self {
        Self {
            task_id: task.id.unwrap(),
            project_id: task.project_id,
            title: task.title,
            status: task.status,
            due_date: task.due_date,
            parent_task_id: task.parent_task_id,
            order_idx: task.order_idx,
            estimate_minutes: task.estimate_minutes,
            actual_minutes: task.actual_minutes,
            assigned_person_id: task.assigned_person_id,
            location_id: task.location_id,
            note: task.note,
            task_created_at: task.created_at,
            task_updated_at: task.updated_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ProjectTaskCreateRequest {
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
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ProjectTaskUpdateRequest {
    pub title: String,
    pub status: TaskStatus,
    pub due_date: Option<NaiveDate>,
    pub parent_task_id: Option<Uuid>,
    pub order_idx: Option<i32>,
    pub estimate_minutes: Option<i32>,
    pub actual_minutes: Option<i32>,
    pub assigned_person_id: Option<Uuid>,
    pub location_id: Option<Uuid>,
    pub note: Option<String>,
}

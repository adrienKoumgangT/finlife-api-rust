use chrono::{DateTime, NaiveDate, Utc};
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::modules::reviews::sessions::review_session_command::ReviewSessionCreateCommand;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ReviewType {
    Weekly,
    Monthly,
}

impl From<String> for ReviewType {
    fn from(value: String) -> Self {
        match value.to_uppercase().as_str() {
            "MONTHLY" => ReviewType::Monthly,
            _ => ReviewType::Weekly,
        }
    }
}

impl ReviewType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ReviewType::Weekly => "WEEKLY",
            ReviewType::Monthly => "MONTHLY",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ReviewStatus {
    Draft,
    Completed,
}

impl From<String> for ReviewStatus {
    fn from(value: String) -> Self {
        match value.to_uppercase().as_str() {
            "COMPLETED" => ReviewStatus::Completed,
            _ => ReviewStatus::Draft,
        }
    }
}

impl ReviewStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            ReviewStatus::Draft => "DRAFT",
            ReviewStatus::Completed => "COMPLETED",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ReviewSession {
    pub id: Option<Uuid>,
    pub user_id: Uuid,
    pub review_type: ReviewType,
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
    pub status: ReviewStatus,

    pub notes: Option<String>,
    pub actions: Option<serde_json::Value>,
    pub decisions: Option<serde_json::Value>,

    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl From<ReviewSessionCreateCommand> for ReviewSession {
    fn from(command: ReviewSessionCreateCommand) -> Self {
        Self {
            id: None,
            user_id: command.user_id,
            review_type: command.review_type,
            period_start: command.period_start,
            period_end: command.period_end,
            status: command.status.unwrap_or(ReviewStatus::Draft),
            notes: command.notes,
            actions: command.actions,
            decisions: command.decisions,
            created_at: None,
            updated_at: None,
        }
    }
}

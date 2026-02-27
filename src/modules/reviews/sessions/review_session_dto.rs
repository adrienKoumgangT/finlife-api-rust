use chrono::{DateTime, NaiveDate, Utc};
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::modules::reviews::sessions::review_session_model::{ReviewSession, ReviewStatus, ReviewType};


#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ReviewSessionResponse {
    pub review_session_id: Uuid,
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

impl From<ReviewSession> for ReviewSessionResponse {
    fn from(review: ReviewSession) -> Self {
        Self {
            review_session_id: review.id.unwrap(),
            user_id: review.user_id,
            review_type: review.review_type,
            period_start: review.period_start,
            period_end: review.period_end,
            status: review.status,
            notes: review.notes,
            actions: review.actions,
            decisions: review.decisions,
            created_at: review.created_at,
            updated_at: review.updated_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ReviewSessionCreateRequest {
    pub review_type: ReviewType,
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
    pub status: Option<ReviewStatus>,

    pub notes: Option<String>,
    pub actions: Option<serde_json::Value>,
    pub decisions: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ReviewSessionUpdateRequest {
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
    pub status: ReviewStatus,

    pub notes: Option<String>,
    pub actions: Option<serde_json::Value>,
    pub decisions: Option<serde_json::Value>,
}

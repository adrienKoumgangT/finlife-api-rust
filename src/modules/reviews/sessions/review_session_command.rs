use chrono::NaiveDate;
use uuid::Uuid;
use serde::{Serialize, Deserialize};

use crate::modules::reviews::sessions::{
    review_session_dto::*,
    review_session_model::{ReviewStatus, ReviewType}
};
use crate::shared::{
    auth::jwt::AuthUser,
    response::PaginationRequest
};


#[derive(Debug, Serialize, Deserialize)]
pub struct ReviewSessionGetCommand {
    pub review_session_id: Uuid,
    pub auth_user: AuthUser,
}

impl ReviewSessionGetCommand {
    pub fn new(review_session_id: Uuid, auth_user: AuthUser) -> Self {
        Self { review_session_id, auth_user }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReviewSessionCreateCommand {
    pub user_id: Uuid,
    pub review_type: ReviewType,
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
    pub status: Option<ReviewStatus>,
    pub notes: Option<String>,
    pub actions: Option<serde_json::Value>,
    pub decisions: Option<serde_json::Value>,
    pub auth_user: AuthUser,
}

impl ReviewSessionCreateCommand {
    pub fn new(request: ReviewSessionCreateRequest, auth_user: AuthUser) -> Self {
        Self {
            user_id: auth_user.user_id,
            review_type: request.review_type,
            period_start: request.period_start,
            period_end: request.period_end,
            status: request.status,
            notes: request.notes,
            actions: request.actions,
            decisions: request.decisions,
            auth_user,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReviewSessionUpdateCommand {
    pub review_session_id: Uuid,
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
    pub status: ReviewStatus,
    pub notes: Option<String>,
    pub actions: Option<serde_json::Value>,
    pub decisions: Option<serde_json::Value>,
    pub auth_user: AuthUser,
}

impl ReviewSessionUpdateCommand {
    pub fn new(review_session_id: Uuid, request: ReviewSessionUpdateRequest, auth_user: AuthUser) -> Self {
        Self {
            review_session_id,
            period_start: request.period_start,
            period_end: request.period_end,
            status: request.status,
            notes: request.notes,
            actions: request.actions,
            decisions: request.decisions,
            auth_user,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReviewSessionDeleteCommand {
    pub review_session_id: Uuid,
    pub auth_user: AuthUser,
}

impl ReviewSessionDeleteCommand {
    pub fn new(review_session_id: Uuid, auth_user: AuthUser) -> Self {
        Self { review_session_id, auth_user }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReviewSessionListByUserCommand {
    pub user_id: Uuid,
    pub pagination: Option<PaginationRequest>,
    pub auth_user: AuthUser,
}

impl ReviewSessionListByUserCommand {
    pub fn new(user_id: Uuid, pagination: Option<PaginationRequest>, auth_user: AuthUser) -> Self {
        Self { user_id, pagination, auth_user }
    }
}

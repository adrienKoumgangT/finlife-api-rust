use chrono::NaiveDateTime;
use uuid::Uuid;
use serde::{Serialize, Deserialize};

use crate::modules::transactions::transaction_dto::*;
use crate::modules::transactions::transaction_model::TransactionStatus;
use crate::shared::auth::jwt::AuthUser;
use crate::shared::response::PaginationRequest;

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionGetCommand {
    pub transaction_id: Uuid,
    pub auth_user: AuthUser,
}

impl TransactionGetCommand {
    pub fn new(transaction_id: Uuid, auth_user: AuthUser) -> Self {
        Self { transaction_id, auth_user }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionCreateCommand {
    pub user_id: Uuid,
    pub account_id: Uuid,
    pub occurred_at: NaiveDateTime,
    pub amount_minor: i64,
    pub currency_code: String,
    pub base_amount_minor: i64,
    pub base_currency_code: String,
    pub fx_rate_id: Option<Uuid>,
    pub category_id: Option<Uuid>,
    pub payee_id: Option<Uuid>,
    pub person_id: Option<Uuid>,
    pub location_id: Option<Uuid>,
    pub note: Option<String>,
    pub project_id: Option<Uuid>,
    pub goal_id: Option<Uuid>,
    pub status: Option<TransactionStatus>,
    pub auth_user: AuthUser,
}

impl TransactionCreateCommand {
    pub fn new(request: TransactionCreateRequest, auth_user: AuthUser) -> Self {
        Self {
            user_id: auth_user.user_id,
            account_id: request.account_id,
            occurred_at: request.occurred_at,
            amount_minor: request.amount_minor,
            currency_code: request.currency_code,
            base_amount_minor: request.base_amount_minor,
            base_currency_code: request.base_currency_code,
            fx_rate_id: request.fx_rate_id,
            category_id: request.category_id,
            payee_id: request.payee_id,
            person_id: request.person_id,
            location_id: request.location_id,
            note: request.note,
            project_id: request.project_id,
            goal_id: request.goal_id,
            status: request.status,
            auth_user,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionUpdateCommand {
    pub transaction_id: Uuid,
    pub account_id: Uuid,
    pub occurred_at: NaiveDateTime,
    pub amount_minor: i64,
    pub currency_code: String,
    pub base_amount_minor: i64,
    pub base_currency_code: String,
    pub fx_rate_id: Option<Uuid>,
    pub category_id: Option<Uuid>,
    pub payee_id: Option<Uuid>,
    pub person_id: Option<Uuid>,
    pub location_id: Option<Uuid>,
    pub note: Option<String>,
    pub project_id: Option<Uuid>,
    pub goal_id: Option<Uuid>,
    pub status: TransactionStatus,
    pub auth_user: AuthUser,
}

impl TransactionUpdateCommand {
    pub fn new(transaction_id: Uuid, request: TransactionUpdateRequest, auth_user: AuthUser) -> Self {
        Self {
            transaction_id,
            account_id: request.account_id,
            occurred_at: request.occurred_at,
            amount_minor: request.amount_minor,
            currency_code: request.currency_code,
            base_amount_minor: request.base_amount_minor,
            base_currency_code: request.base_currency_code,
            fx_rate_id: request.fx_rate_id,
            category_id: request.category_id,
            payee_id: request.payee_id,
            person_id: request.person_id,
            location_id: request.location_id,
            note: request.note,
            project_id: request.project_id,
            goal_id: request.goal_id,
            status: request.status,
            auth_user,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionDeleteCommand {
    pub transaction_id: Uuid,
    pub auth_user: AuthUser,
}

impl TransactionDeleteCommand {
    pub fn new(transaction_id: Uuid, auth_user: AuthUser) -> Self {
        Self { transaction_id, auth_user }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionListByUserCommand {
    pub user_id: Uuid,
    pub pagination: Option<PaginationRequest>,
    pub auth_user: AuthUser,
}

impl TransactionListByUserCommand {
    pub fn new(user_id: Uuid, pagination: Option<PaginationRequest>, auth_user: AuthUser) -> Self {
        Self { user_id, pagination, auth_user }
    }
}

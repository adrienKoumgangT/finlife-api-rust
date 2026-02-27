use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::modules::transactions::transaction_model::{Transaction, TransactionStatus};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TransactionResponse {
    pub transaction_id: Uuid,
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

    pub status: TransactionStatus,

    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl From<Transaction> for TransactionResponse {
    fn from(tx: Transaction) -> Self {
        Self {
            transaction_id: tx.id.unwrap(),
            user_id: tx.user_id,
            account_id: tx.account_id,
            occurred_at: tx.occurred_at,
            amount_minor: tx.amount_minor,
            currency_code: tx.currency_code,
            base_amount_minor: tx.base_amount_minor,
            base_currency_code: tx.base_currency_code,
            fx_rate_id: tx.fx_rate_id,
            category_id: tx.category_id,
            payee_id: tx.payee_id,
            person_id: tx.person_id,
            location_id: tx.location_id,
            note: tx.note,
            project_id: tx.project_id,
            goal_id: tx.goal_id,
            status: tx.status,
            created_at: tx.created_at,
            updated_at: tx.updated_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TransactionCreateRequest {
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
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TransactionUpdateRequest {
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
}

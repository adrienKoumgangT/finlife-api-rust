use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::modules::transactions::transaction_model::{MonthlyCategoryExpense, MonthlyFlow, Transaction, TransactionStatus};

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


#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct MonthlyFlowResponse {
    pub year: u32,
    pub month: u32,
    pub total_income: i64,
    pub total_expense: i64,
}

impl From<MonthlyFlow> for MonthlyFlowResponse {
    fn from(flow: MonthlyFlow) -> Self {
        let mut year = 0;
        let mut month = 0;

        if let Some(month_str) = flow.month {
            let parts: Vec<&str> = month_str.split('-').collect();
            if parts.len() == 2 {
                year = parts[0].parse::<u32>().unwrap_or(0);
                month = parts[1].parse::<u32>().unwrap_or(0);
            }
        }

        MonthlyFlowResponse {
            year,
            month,
            total_income: flow.total_income,
            total_expense: flow.total_expense,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct MonthlyCategoryExpenseResponse {
    pub year: u32,
    pub month: u32,
    pub category_id: Option<Uuid>,
    pub total_amount: i64,
}

impl From<MonthlyCategoryExpense> for MonthlyCategoryExpenseResponse {
    fn from(flow: MonthlyCategoryExpense) -> Self {
        let mut year = 0;
        let mut month = 0;

        if let Some(month_str) = flow.month {
            let parts: Vec<&str> = month_str.split('-').collect();
            if parts.len() == 2 {
                year = parts[0].parse::<u32>().unwrap_or(0);
                month = parts[1].parse::<u32>().unwrap_or(0);
            }
        }

        MonthlyCategoryExpenseResponse {
            year,
            month,
            category_id: flow.category_id,
            total_amount: flow.total_amount,
        }
    }
}


use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::modules::accounts::account_model::{Account, AccountType};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AccountResponse {
    pub account_id: Uuid,

    pub name: String,
    pub account_type: AccountType,
    pub currency_code: String,
    pub institution: Option<String>,

    pub archived: bool,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl From<Account> for AccountResponse {
    fn from(account: Account) -> Self {
        Self {
            account_id: account.id.unwrap(),
            name: account.name,
            account_type: account.account_type,
            currency_code: account.currency_code,
            institution: account.institution,
            archived: account.archived,
            created_at: account.created_at,
            updated_at: account.updated_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AccountCreateRequest {
    pub name: String,
    pub account_type: AccountType,
    pub currency_code: String,
    pub institution: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AccountUpdateRequest {
    pub name: String,
    pub account_type: AccountType,
    pub institution: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AccountUpdateArchivedRequest {
    pub archived: bool,
}

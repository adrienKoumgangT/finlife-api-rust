use chrono::{DateTime, Utc};
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::modules::accounts::account_command::AccountCreateCommand;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum AccountType {
    Checking,
    Savings,
    Cash,
    Broker,
    Debt,
}

impl From<String> for AccountType {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_str() {
            "savings" => AccountType::Savings,
            "cash" => AccountType::Cash,
            "broker" => AccountType::Broker,
            "debt" => AccountType::Debt,
            _ => AccountType::Checking,
        }
    }
}

impl AccountType {
    pub fn as_str(&self) -> &'static str {
        match self {
            AccountType::Checking => "checking",
            AccountType::Savings => "savings",
            AccountType::Cash => "cash",
            AccountType::Broker => "broker",
            AccountType::Debt => "debt",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Account {
    pub id: Option<Uuid>,
    pub user_id: Uuid,

    pub name: String,
    pub account_type: AccountType,
    pub currency_code: String,
    pub institution: Option<String>,

    pub archived: bool,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl Account {
    pub fn is_archived(&self) -> bool {
        self.archived
    }
}

impl From<AccountCreateCommand> for Account {
    fn from(command: AccountCreateCommand) -> Self {
        Self {
            id: None,
            user_id: command.user_id,
            name: command.name,
            account_type: command.account_type,
            currency_code: command.currency_code,
            institution: command.institution,
            archived: false,
            created_at: None,
            updated_at: None,
        }
    }
}

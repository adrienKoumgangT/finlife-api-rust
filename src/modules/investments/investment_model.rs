use chrono::{DateTime, NaiveDateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::modules::investments::investment_command::{
    PortfolioCreateCommand, PositionCreateCommand, TradeCreateCommand,
};

// --- ENUMS ---

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PositionStatus {
    Open,
    Closed,
}

impl From<String> for PositionStatus {
    fn from(value: String) -> Self {
        match value.to_uppercase().as_str() {
            "CLOSED" => PositionStatus::Closed,
            _ => PositionStatus::Open,
        }
    }
}

impl PositionStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            PositionStatus::Open => "OPEN",
            PositionStatus::Closed => "CLOSED",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TradeType {
    Buy,
    Sell,
}

impl From<String> for TradeType {
    fn from(value: String) -> Self {
        match value.to_uppercase().as_str() {
            "SELL" => TradeType::Sell,
            _ => TradeType::Buy,
        }
    }
}

impl TradeType {
    pub fn as_str(&self) -> &'static str {
        match self {
            TradeType::Buy => "BUY",
            TradeType::Sell => "SELL",
        }
    }
}

// --- MODELS ---

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Portfolio {
    pub id: Option<Uuid>,
    pub user_id: Uuid,
    pub name: String,
    pub base_currency_code: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl From<PortfolioCreateCommand> for Portfolio {
    fn from(command: PortfolioCreateCommand) -> Self {
        Self {
            id: None,
            user_id: command.user_id,
            name: command.name,
            base_currency_code: command.base_currency_code,
            created_at: None,
            updated_at: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Position {
    pub id: Option<Uuid>,
    pub portfolio_id: Uuid,
    pub symbol: String,
    pub name: String,
    pub status: PositionStatus,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl From<PositionCreateCommand> for Position {
    fn from(command: PositionCreateCommand) -> Self {
        Self {
            id: None,
            portfolio_id: command.portfolio_id,
            symbol: command.symbol,
            name: command.name,
            status: command.status.unwrap_or(PositionStatus::Open),
            created_at: None,
            updated_at: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Trade {
    pub id: Option<Uuid>,
    pub position_id: Uuid,
    pub trade_type: TradeType,
    pub trade_date: NaiveDateTime,
    pub quantity: Decimal,
    pub price_minor: i64,
    pub fees_minor: i64,
    pub currency_code: String,
    pub created_at: Option<DateTime<Utc>>,
}

impl From<TradeCreateCommand> for Trade {
    fn from(command: TradeCreateCommand) -> Self {
        Self {
            id: None,
            position_id: command.position_id,
            trade_type: command.trade_type,
            trade_date: command.trade_date,
            quantity: command.quantity,
            price_minor: command.price_minor,
            fees_minor: command.fees_minor.unwrap_or(0),
            currency_code: command.currency_code,
            created_at: None,
        }
    }
}

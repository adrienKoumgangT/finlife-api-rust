use chrono::{DateTime, NaiveDateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::modules::investments::investment_model::{Portfolio, Position, Trade, PositionStatus, TradeType};

// --- PORTFOLIO ---
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PortfolioResponse {
    pub portfolio_id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub base_currency_code: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl From<Portfolio> for PortfolioResponse {
    fn from(p: Portfolio) -> Self {
        Self {
            portfolio_id: p.id.unwrap(),
            user_id: p.user_id,
            name: p.name,
            base_currency_code: p.base_currency_code,
            created_at: p.created_at,
            updated_at: p.updated_at
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PortfolioCreateRequest {
    pub name: String,
    pub base_currency_code: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PortfolioUpdateRequest {
    pub name: String,
    pub base_currency_code: String,
}

// --- POSITION ---
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PositionResponse {
    pub position_id: Uuid,
    pub portfolio_id: Uuid,
    pub symbol: String,
    pub name: String,
    pub status: PositionStatus,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl From<Position> for PositionResponse {
    fn from(p: Position) -> Self {
        Self {
            position_id: p.id.unwrap(),
            portfolio_id: p.portfolio_id,
            symbol: p.symbol,
            name: p.name,
            status: p.status,
            created_at: p.created_at,
            updated_at: p.updated_at
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PositionCreateRequest {
    pub portfolio_id: Uuid,
    pub symbol: String,
    pub name: String,
    pub status: Option<PositionStatus>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PositionUpdateRequest {
    pub symbol: String,
    pub name: String,
    pub status: PositionStatus,
}

// --- TRADE ---
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TradeResponse {
    pub trade_id: Uuid,
    pub position_id: Uuid,
    pub trade_type: TradeType,
    pub trade_date: NaiveDateTime,
    pub quantity: Decimal,
    pub price_minor: i64,
    pub fees_minor: i64,
    pub currency_code: String,
    pub created_at: Option<DateTime<Utc>>,
}

impl From<Trade> for TradeResponse {
    fn from(t: Trade) -> Self {
        Self {
            trade_id: t.id.unwrap(),
            position_id: t.position_id,
            trade_type: t.trade_type,
            trade_date: t.trade_date,
            quantity: t.quantity,
            price_minor: t.price_minor,
            fees_minor: t.fees_minor,
            currency_code: t.currency_code,
            created_at: t.created_at
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TradeCreateRequest {
    pub position_id: Uuid,
    pub trade_type: TradeType,
    pub trade_date: NaiveDateTime,
    pub quantity: Decimal,
    pub price_minor: i64,
    pub fees_minor: Option<i64>,
    pub currency_code: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TradeUpdateRequest {
    pub trade_type: TradeType,
    pub trade_date: NaiveDateTime,
    pub quantity: Decimal,
    pub price_minor: i64,
    pub fees_minor: i64,
    pub currency_code: String,
}

use chrono::NaiveDateTime;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::modules::investments::investment_dto::*;
use crate::modules::investments::investment_model::{PositionStatus, TradeType};
use crate::shared::auth::jwt::AuthUser;
use crate::shared::response::PaginationRequest;


// --- PORTFOLIO COMMANDS ---
#[derive(Debug, Serialize, Deserialize)]
pub struct PortfolioCreateCommand {
    pub user_id: Uuid,
    pub name: String,
    pub base_currency_code: String,

    pub auth_user: AuthUser
}

impl PortfolioCreateCommand {
    pub fn new(req: PortfolioCreateRequest, auth_user: AuthUser) -> Self {
        Self {
            user_id: auth_user.user_id,
            name: req.name,
            base_currency_code: req.base_currency_code,
            auth_user
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PortfolioUpdateCommand {
    pub portfolio_id: Uuid,
    pub name: String,
    pub base_currency_code: String,

    pub auth_user: AuthUser
}

impl PortfolioUpdateCommand {
    pub fn new(portfolio_id: Uuid, req: PortfolioUpdateRequest, auth_user: AuthUser) -> Self {
        Self {
            portfolio_id,
            name: req.name,
            base_currency_code: req.base_currency_code,
            auth_user
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PortfolioDeleteCommand {
    pub portfolio_id: Uuid,

    pub auth_user: AuthUser
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PortfolioGetCommand {
    pub portfolio_id: Uuid,

    pub auth_user: AuthUser
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PortfolioListByUserCommand {
    pub user_id: Uuid,

    pub pagination: Option<PaginationRequest>,

    pub auth_user: AuthUser
}

impl PortfolioListByUserCommand {
    pub fn new(user_id: Uuid, pagination: Option<PaginationRequest>, auth_user: AuthUser) -> Self {
        Self { user_id, pagination, auth_user }
    }
}



// --- POSITION COMMANDS ---
#[derive(Debug, Serialize, Deserialize)]
pub struct PositionCreateCommand {
    pub portfolio_id: Uuid,
    pub symbol: String,
    pub name: String,
    pub status: Option<PositionStatus>,

    pub auth_user: AuthUser
}

impl PositionCreateCommand {
    pub fn new(req: PositionCreateRequest, auth_user: AuthUser) -> Self {
        Self {
            portfolio_id: req.portfolio_id,
            symbol: req.symbol,
            name: req.name,
            status: req.status,
            auth_user
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PositionUpdateCommand {
    pub position_id: Uuid,
    pub symbol: String,
    pub name: String,
    pub status: PositionStatus,

    pub auth_user: AuthUser
}

impl PositionUpdateCommand {
    pub fn new(position_id: Uuid, req: PositionUpdateRequest, auth_user: AuthUser) -> Self {
        Self {
            position_id,
            symbol: req.symbol,
            name: req.name,
            status: req.status,
            auth_user
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PositionDeleteCommand {
    pub position_id: Uuid,

    pub auth_user: AuthUser
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PositionGetCommand {
    pub position_id: Uuid,

    pub auth_user: AuthUser
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PositionListByPortfolioCommand {
    pub portfolio_id: Uuid,

    pub pagination: Option<PaginationRequest>,

    pub auth_user: AuthUser
}



// --- TRADE COMMANDS ---
#[derive(Debug, Serialize, Deserialize)]
pub struct TradeCreateCommand {
    pub position_id: Uuid,
    pub trade_type: TradeType,
    pub trade_date: NaiveDateTime,
    pub quantity: Decimal,
    pub price_minor: i64,
    pub fees_minor: Option<i64>,
    pub currency_code: String,

    pub auth_user: AuthUser
}

impl TradeCreateCommand {
    pub fn new(req: TradeCreateRequest, auth_user: AuthUser) -> Self {
        Self {
            position_id: req.position_id,
            trade_type: req.trade_type,
            trade_date: req.trade_date,
            quantity: req.quantity,
            price_minor: req.price_minor,
            fees_minor: req.fees_minor,
            currency_code: req.currency_code,
            auth_user
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TradeUpdateCommand {
    pub trade_id: Uuid,
    pub trade_type: TradeType,
    pub trade_date: NaiveDateTime,
    pub quantity: Decimal,
    pub price_minor: i64,
    pub fees_minor: i64,
    pub currency_code: String,

    pub auth_user: AuthUser
}

impl TradeUpdateCommand {
    pub fn new(trade_id: Uuid, req: TradeUpdateRequest, auth_user: AuthUser) -> Self {
        Self {
            trade_id,
            trade_type: req.trade_type,
            trade_date: req.trade_date,
            quantity: req.quantity,
            price_minor: req.price_minor,
            fees_minor: req.fees_minor,
            currency_code: req.currency_code,
            auth_user
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TradeDeleteCommand {
    pub trade_id: Uuid,

    pub auth_user: AuthUser
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TradeGetCommand {
    pub trade_id: Uuid,

    pub auth_user: AuthUser
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TradeListByPositionCommand {
    pub position_id: Uuid,

    pub pagination: Option<PaginationRequest>,

    pub auth_user: AuthUser
}

use chrono::{NaiveDate, Utc};
use rust_decimal::Decimal;
use uuid::Uuid;
use serde::{Serialize, Deserialize};

use crate::modules::currencies::currency_dto::{
    CurrencyCreateRequest, CurrencyUpdateNameRequest,
    FxRateCreateRequest, FxRateUpdateRateRequest
};
use crate::shared::auth::jwt::AuthUser;
use crate::shared::response::PaginationRequest;


// --- Currency ---

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrencyGetCommand {
    pub code: String,

    pub auth_user: AuthUser,
}

impl CurrencyGetCommand {
    pub fn new(code: String, auth_user: AuthUser) -> Self {
        Self { code, auth_user }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrencyCreateCommand {
    pub code: String,
    pub name: String,

    pub minor_unit: u8,

    pub auth_user: AuthUser,
}

impl CurrencyCreateCommand {
    pub fn new(request: CurrencyCreateRequest, auth_user: AuthUser) -> Self {
        Self {
            code: request.code,
            name: request.name,
            minor_unit: request.minor_unit,
            auth_user,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrencyUpdateNameCommand {
    pub code: String,
    pub name: String,

    pub auth_user: AuthUser,
}

impl CurrencyUpdateNameCommand {
    pub fn new(request: CurrencyUpdateNameRequest, auth_user: AuthUser) -> Self {
        Self {
            code: request.code,
            name: request.name,
            auth_user,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrencyDeleteCommand {
    pub code: String,

    pub auth_user: AuthUser,
}

impl CurrencyDeleteCommand {
    pub fn new(code: String, auth_user: AuthUser) -> Self {
        Self { code, auth_user }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrencyListCommand {
    pub pagination: Option<PaginationRequest>,

    pub auth_user: AuthUser,
}

impl CurrencyListCommand {
    pub fn new(pagination: PaginationRequest, auth_user: AuthUser) -> Self {
        Self { pagination: Some(pagination), auth_user }
    }
}


// --- FxRate ---

#[derive(Debug, Serialize, Deserialize)]
pub struct FxRateGetCommand {
    pub fx_rate_id: Uuid,

    pub auth_user: AuthUser,
}

impl FxRateGetCommand {
    pub fn new(fx_rate_id: Uuid, auth_user: AuthUser) -> Self {
        Self { fx_rate_id, auth_user }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FxRateByBaseCodeCommand {
    pub base_code: String,

    pub pagination: Option<PaginationRequest>,

    pub auth_user: AuthUser,
}

impl FxRateByBaseCodeCommand {
    pub fn new(base_code: String, pagination: Option<PaginationRequest>, auth_user: AuthUser) -> Self {
        Self { base_code, pagination, auth_user }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FxRateCreateCommand {
    pub base_code: String,
    pub quote_code: String,
    pub rate: Decimal,
    pub as_of_date: NaiveDate,
    pub source: String,

    pub auth_user: AuthUser,
}

impl FxRateCreateCommand {
    pub fn new(request: FxRateCreateRequest, auth_user: AuthUser) -> Self {
        Self {
            base_code: request.base_code,
            quote_code: request.quote_code,
            rate: request.rate,
            as_of_date: request.as_of_date.unwrap_or_else(|| Utc::now().date_naive()),
            source: request.source.unwrap_or_else(|| "manual".to_string()),
            auth_user,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FxRateUpdateRateCommand {
    pub fx_rate_id: Uuid,

    pub rate: Decimal,

    pub auth_user: AuthUser,
}

impl FxRateUpdateRateCommand {
    pub fn new(fx_rate_id: Uuid, request: FxRateUpdateRateRequest, auth_user: AuthUser) -> Self {
        Self {
            fx_rate_id,
            rate: request.rate,
            auth_user,
        }
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct FxRateDeleteCommand {
    pub fx_rate_id: Uuid,

    pub auth_user: AuthUser,
}

impl FxRateDeleteCommand {
    pub fn new(fx_rate_id: Uuid, auth_user: AuthUser) -> Self {
        Self { fx_rate_id, auth_user }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FxRateListCommand {
    pub pagination: Option<PaginationRequest>,

    pub auth_user: AuthUser,
}

impl FxRateListCommand {
    pub fn new(pagination: Option<PaginationRequest>, auth_user: AuthUser) -> Self {
        Self { pagination, auth_user }
    }
}

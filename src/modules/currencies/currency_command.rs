use chrono::NaiveDate;
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
    pub currency_code: String,

    pub auth_user: AuthUser,
}

impl CurrencyGetCommand {
    pub fn new(currency_code: String, auth_user: AuthUser) -> Self {
        Self { currency_code, auth_user }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrencyCreateCommand {
    pub currency_code: String,
    pub currency_name: String,

    pub currency_minor_unit: u8,

    pub auth_user: AuthUser,
}

impl CurrencyCreateCommand {
    pub fn new(request: CurrencyCreateRequest, auth_user: AuthUser) -> Self {
        Self {
            currency_code: request.currency_code,
            currency_name: request.currency_name,
            currency_minor_unit: request.currency_minor_unit,
            auth_user,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrencyUpdateNameCommand {
    pub currency_code: String,
    pub currency_name: String,

    pub auth_user: AuthUser,
}

impl CurrencyUpdateNameCommand {
    pub fn new(request: CurrencyUpdateNameRequest, auth_user: AuthUser) -> Self {
        Self {
            currency_code: request.currency_code,
            currency_name: request.currency_name,
            auth_user,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrencyDeleteCommand {
    pub currency_code: String,

    pub auth_user: AuthUser,
}

impl CurrencyDeleteCommand {
    pub fn new(currency_code: String, auth_user: AuthUser) -> Self {
        Self { currency_code, auth_user }
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
    pub fx_rate_base_code: String,

    pub pagination: Option<PaginationRequest>,

    pub auth_user: AuthUser,
}

impl FxRateByBaseCodeCommand {
    pub fn new(fx_rate_base_code: String, pagination: Option<PaginationRequest>, auth_user: AuthUser) -> Self {
        Self { fx_rate_base_code, pagination, auth_user }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FxRateCreateCommand {
    pub fx_rate_base_code: String,
    pub fx_rate_quote_code: String,
    pub fx_rate_rate: Decimal,
    pub fx_rate_as_of_date: NaiveDate,
    pub fx_rate_source: String,

    pub auth_user: AuthUser,
}

impl FxRateCreateCommand {
    pub fn new(request: FxRateCreateRequest, auth_user: AuthUser) -> Self {
        Self {
            fx_rate_base_code: request.fx_rate_base_code,
            fx_rate_quote_code: request.fx_rate_quote_code,
            fx_rate_rate: request.fx_rate_rate,
            fx_rate_as_of_date: request.fx_rate_as_of_date,
            fx_rate_source: request.fx_rate_source,
            auth_user,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FxRateUpdateRateCommand {
    pub fx_rate_id: Uuid,

    pub fx_rate_rate: Decimal,

    pub auth_user: AuthUser,
}

impl FxRateUpdateRateCommand {
    pub fn new(fx_rate_id: Uuid, request: FxRateUpdateRateRequest, auth_user: AuthUser) -> Self {
        Self {
            fx_rate_id,
            fx_rate_rate: request.fx_rate_rate,
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

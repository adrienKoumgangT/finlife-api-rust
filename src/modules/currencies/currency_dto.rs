use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Serialize, Deserialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use crate::modules::currencies::currency_model::{Currency, FxRate};
use crate::shared::utils::bu;


// --- Currency ---

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CurrencyResponse {
    pub currency_code: String,
    pub currency_name: String,

    pub currency_minor_unit: u8,
}

impl From<Currency> for CurrencyResponse {
    fn from(currency: Currency) -> Self {
        Self {
            currency_code: currency.code,
            currency_name: currency.name,
            currency_minor_unit: currency.minor_unit,
        }
    }
}

impl From<&Currency> for CurrencyResponse {
    fn from(currency: &Currency) -> Self {
        Self {
            currency_code: currency.code.clone(),
            currency_name: currency.name.clone(),
            currency_minor_unit: currency.minor_unit,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CurrencyCreateRequest {
    pub currency_code: String,
    pub currency_name: String,

    pub currency_minor_unit: u8,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CurrencyUpdateNameRequest {
    pub currency_code: String,
    pub currency_name: String,
}


// --- FxRate ---

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct FxRateResponse {
    pub fx_rate_id: Uuid,
    pub fx_rate_base_code: String,
    pub fx_rate_quote_code: String,
    pub fx_rate_rate: Decimal,
    pub fx_rate_as_of_date: NaiveDate,
    pub fx_rate_source: String,
}

impl From<FxRate> for FxRateResponse {
    fn from(fx_rate: FxRate) -> Self {
        Self {
            fx_rate_id: bu(fx_rate.id.unwrap().as_slice()),
            fx_rate_base_code: fx_rate.base_code,
            fx_rate_quote_code: fx_rate.quote_code,
            fx_rate_rate: fx_rate.rate,
            fx_rate_as_of_date: fx_rate.as_of_date,
            fx_rate_source: fx_rate.source,
        }
    }
}

impl From<&FxRate> for FxRateResponse {
    fn from(fx_rate: &FxRate) -> Self {
        Self {
            fx_rate_id: bu(fx_rate.id.clone().unwrap().as_slice()),
            fx_rate_base_code: fx_rate.base_code.clone(),
            fx_rate_quote_code: fx_rate.quote_code.clone(),
            fx_rate_rate: fx_rate.rate.clone(),
            fx_rate_as_of_date: fx_rate.as_of_date.clone(),
            fx_rate_source: fx_rate.source.clone(),
        }
    }
}


#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct FxRateCreateRequest {
    pub fx_rate_base_code: String,
    pub fx_rate_quote_code: String,
    pub fx_rate_rate: Decimal,
    pub fx_rate_as_of_date: NaiveDate,
    pub fx_rate_source: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct FxRateUpdateRateRequest {
    pub fx_rate_rate: Decimal,
}

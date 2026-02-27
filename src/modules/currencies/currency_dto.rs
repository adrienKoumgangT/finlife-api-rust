use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::modules::currencies::currency_model::{Currency, FxRate};

// --- Currency ---

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CurrencyResponse {
    pub code: String,
    pub name: String,

    pub minor_unit: u8,
}

impl From<Currency> for CurrencyResponse {
    fn from(currency: Currency) -> Self {
        Self {
            code: currency.code,
            name: currency.name,
            minor_unit: currency.minor_unit,
        }
    }
}

impl From<&Currency> for CurrencyResponse {
    fn from(currency: &Currency) -> Self {
        Self {
            code: currency.code.clone(),
            name: currency.name.clone(),
            minor_unit: currency.minor_unit,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CurrencyCreateRequest {
    pub code: String,
    pub name: String,

    pub minor_unit: u8,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CurrencyUpdateNameRequest {
    pub code: String,
    pub name: String,
}


// --- FxRate ---

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct FxRateResponse {
    pub fx_rate_id: Uuid,
    pub base_code: String,
    pub quote_code: String,
    pub rate: Decimal,
    pub as_of_date: NaiveDate,
    pub source: String,
}

impl From<FxRate> for FxRateResponse {
    fn from(fx_rate: FxRate) -> Self {
        Self {
            fx_rate_id: fx_rate.id.unwrap(), // Direct unwrap!
            base_code: fx_rate.base_code,
            quote_code: fx_rate.quote_code,
            rate: fx_rate.rate,
            as_of_date: fx_rate.as_of_date,
            source: fx_rate.source,
        }
    }
}

impl From<&FxRate> for FxRateResponse {
    fn from(fx_rate: &FxRate) -> Self {
        Self {
            fx_rate_id: fx_rate.id.clone().unwrap(),
            base_code: fx_rate.base_code.clone(),
            quote_code: fx_rate.quote_code.clone(),
            rate: fx_rate.rate.clone(),
            as_of_date: fx_rate.as_of_date.clone(),
            source: fx_rate.source.clone(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct FxRateCreateRequest {
    pub base_code: String,
    pub quote_code: String,
    pub rate: Decimal,
    pub as_of_date: NaiveDate,
    pub source: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct FxRateUpdateRateRequest {
    pub rate: Decimal,
}

use anyhow::{Error, Result};
use async_trait::async_trait;
use rust_decimal::Decimal;
use uuid::Uuid;
use sqlx::MySqlPool;

use crate::modules::currencies::currency_model::{Currency, FxRate};
use crate::shared::state::AppState;

#[async_trait]
pub trait CurrencyRepositoryInterface {

    async fn get(&self, code: String) -> Result<Option<Currency>, Error>;

    async fn create(&self, currency: Currency) -> Result<Currency, Error>;

    async fn update_name(&self, code: String, name: String) -> Result<Option<Currency>, Error>;

    async fn delete(&self, code: String) -> Result<(), Error>;

    async fn get_all(&self) -> Result<Vec<Currency>, Error>;

}

#[derive(Clone)]
pub struct CurrencyRepository {
    pool: MySqlPool,
}

impl From<&AppState> for CurrencyRepository {
    fn from(app_state: &AppState) -> Self {
        Self { pool: app_state.mysql_pool.clone() }
    }
}

#[async_trait]
impl CurrencyRepositoryInterface for CurrencyRepository {
    async fn get(&self, code: String) -> Result<Option<Currency>, Error> {
        let currency = sqlx::query_as!(
            Currency,
            "SELECT code, name, minor_unit FROM currencies WHERE code = ?",
            code
        )
            .fetch_optional(&self.pool)
            .await?;

        Ok(currency)
    }

    async fn create(&self, currency: Currency) -> Result<Currency, Error> {
        sqlx::query!(
            "INSERT INTO currencies (code, name, minor_unit) VALUES (?, ?, ?)",
            currency.code,
            currency.name,
            currency.minor_unit
        )
            .execute(&self.pool)
            .await?;

        let result = self.get(currency.code.clone()).await?;
        result.ok_or_else(|| Error::msg("Currency not found after creation"))
    }

    async fn update_name(&self, code: String, name: String) -> Result<Option<Currency>, Error> {
        sqlx::query!(
            "UPDATE currencies SET name = ? WHERE code = ?",
            name,
            code
        )
            .execute(&self.pool)
            .await?;

        self.get(code).await
    }

    async fn delete(&self, code: String) -> Result<(), Error> {
        sqlx::query!("DELETE FROM currencies WHERE code = ?", code)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn get_all(&self) -> Result<Vec<Currency>, Error> {
        let currencies = sqlx::query_as!(
            Currency,
            "SELECT code, name, minor_unit FROM currencies"
        )
            .fetch_all(&self.pool)
            .await?;

        Ok(currencies)
    }
}


#[async_trait]
pub trait FxRateRepositoryInterface {

    async fn get(&self, fx_rate_id: Uuid) -> Result<Option<FxRate>, Error>;

    async fn create(&self, fx_rate: FxRate) -> Result<FxRate, Error>;

    async fn update(&self, fx_rate_id: Uuid, fx_rate_rate: Decimal) -> Result<Option<FxRate>, Error>;

    async fn delete(&self, fx_rate_id: Uuid) -> Result<(), Error>;

    async fn get_all(&self) -> Result<Vec<FxRate>, Error>;

    async fn get_by_base_code(&self, base_code: String) -> Result<Vec<FxRate>, Error>;

}

#[derive(Clone)]
pub struct FxRateRepository {
    pool: MySqlPool,
}

impl From<&AppState> for FxRateRepository {
    fn from(app_state: &AppState) -> Self {
        Self { pool: app_state.mysql_pool.clone() }
    }
}

#[async_trait]
impl FxRateRepositoryInterface for FxRateRepository {
    async fn get(&self, fx_rate_id: Uuid) -> Result<Option<FxRate>, Error> {
        let fx_rate = sqlx::query_as!(
            FxRate,
            r#"
            SELECT
                id AS "id: _",
                base_code, quote_code, rate, as_of_date, source, created_at
            FROM fx_rates
            WHERE id = ?
            "#,
            fx_rate_id
        )
            .fetch_optional(&self.pool)
            .await?;

        Ok(fx_rate)
    }

    async fn create(&self, fx_rate: FxRate) -> Result<FxRate, Error> {
        let new_id = Uuid::new_v4();

        sqlx::query!(
            r#"
            INSERT INTO fx_rates (id, base_code, quote_code, rate, as_of_date, source)
            VALUES (?, ?, ?, ?, ?, ?)
            "#,
            new_id,
            fx_rate.base_code,
            fx_rate.quote_code,
            fx_rate.rate,
            fx_rate.as_of_date,
            fx_rate.source
        )
            .execute(&self.pool)
            .await?;

        let result = self.get(new_id).await?;
        result.ok_or_else(|| Error::msg("FxRate not found after creation"))
    }

    async fn update(&self, fx_rate_id: Uuid, rate: Decimal) -> Result<Option<FxRate>, Error> {
        sqlx::query!(
            "UPDATE fx_rates SET rate = ? WHERE id = ?",
            rate,
            fx_rate_id
        )
            .execute(&self.pool)
            .await?;

        self.get(fx_rate_id).await
    }

    async fn delete(&self, fx_rate_id: Uuid) -> Result<(), Error> {
        sqlx::query!("DELETE FROM fx_rates WHERE id = ?", fx_rate_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn get_all(&self) -> Result<Vec<FxRate>, Error> {
        let fx_rates = sqlx::query_as!(
            FxRate,
            r#"
            SELECT
                id AS "id: _",
                base_code, quote_code, rate, as_of_date, source, created_at
            FROM fx_rates
            "#
        )
            .fetch_all(&self.pool)
            .await?;

        Ok(fx_rates)
    }

    async fn get_by_base_code(&self, base_code: String) -> Result<Vec<FxRate>, Error> {
        let fx_rates = sqlx::query_as!(
            FxRate,
            r#"
            SELECT
                id AS "id: _",
                base_code, quote_code, rate, as_of_date, source, created_at
            FROM fx_rates
            WHERE base_code = ?
            "#,
            base_code
        )
            .fetch_all(&self.pool)
            .await?;

        Ok(fx_rates)
    }
}

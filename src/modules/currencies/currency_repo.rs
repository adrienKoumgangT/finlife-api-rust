use anyhow::{Error, Result};
use async_trait::async_trait;
use rust_decimal::Decimal;
use uuid::Uuid;
use sqlx::MySqlPool;

use crate::modules::currencies::currency_model::{Currency, FxRate};
use crate::shared::db::mysql::{GenericRepository, MySqlParam};
use crate::shared::crud_repository::CrudRepository;
use crate::shared::state::AppState;
use crate::shared::utils::{oub, ub};

#[async_trait]
pub trait CurrencyRepositoryInterface {

    async fn get(&self, code: String, meta_user: Option<Uuid>) -> Result<Option<Currency>, Error>;

    async fn create(&self, currency: Currency, meta_user: Option<Uuid>) -> Result<Currency, Error>;

    async fn update_name(&self, code: String, name: String, meta_user: Option<Uuid>) -> Result<Option<Currency>, Error>;

    async fn delete(&self, code: String, meta_user: Option<Uuid>) -> Result<(), Error>;
    
    async fn get_all(&self, meta_user: Option<Uuid>) -> Result<Vec<Currency>, Error>;
    
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

impl GenericRepository<Currency> for CurrencyRepository {
    fn get_pool(&self) -> &MySqlPool {
        &self.pool
    }
}

#[async_trait]
impl CurrencyRepositoryInterface for CurrencyRepository {
    async fn get(&self, code: String, meta_user: Option<Uuid>) -> Result<Option<Currency>, Error> {
        let params = vec![
            MySqlParam::from(code),
            MySqlParam::from(oub(meta_user)),
        ];
        
        self.call_procedure_for_optional("proc_currency_get_by_code", params).await
    }

    async fn create(&self, currency: Currency, meta_user: Option<Uuid>) -> Result<Currency, Error> {
        let params = vec![
            MySqlParam::from(currency.code),
            MySqlParam::from(currency.name),
            MySqlParam::from(currency.minor_unit),
            MySqlParam::from(oub(meta_user)),
        ];
        
        self.call_procedure_for_one("proc_currency_insert", params).await
    }

    async fn update_name(&self, code: String, name: String, meta_user: Option<Uuid>) -> Result<Option<Currency>, Error> {
        let params = vec![
            MySqlParam::from(code),
            MySqlParam::from(name),
            MySqlParam::from(oub(meta_user)),
        ];
        
        self.call_procedure_for_optional("proc_currency_update_name", params).await
    }

    async fn delete(&self, code: String, meta_user: Option<Uuid>) -> Result<(), Error> {
        let params = vec![
            MySqlParam::from(code),
            MySqlParam::from(oub(meta_user)),
        ];
        
        self.call_procedure("proc_currency_delete", params).await
    }

    async fn get_all(&self, meta_user: Option<Uuid>) -> Result<Vec<Currency>, Error> {
        let params = vec![
            MySqlParam::from(oub(meta_user)),
        ];
        
        self.call_procedure_for_list("proc_currency_list", params).await
    }
}



#[async_trait]
pub trait FxRateRepositoryInterface {

    async fn get(&self, fx_rate_id: Uuid, meta_user: Option<Uuid>) -> Result<Option<FxRate>, Error>;

    async fn create(&self, fx_rate: FxRate, meta_user: Option<Uuid>) -> Result<FxRate, Error>;

    async fn update(&self, fx_rate_id: Uuid, fx_rate_rate: Decimal, meta_user: Option<Uuid>) -> Result<Option<FxRate>, Error>;

    async fn delete(&self, fx_rate_id: Uuid, meta_user: Option<Uuid>) -> Result<(), Error>;

    async fn get_all(&self, meta_user: Option<Uuid>) -> Result<Vec<FxRate>, Error>;
    
    async fn get_by_base_code(&self, base_code: String, meta_user: Option<Uuid>) -> Result<Vec<FxRate>, Error>;

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

impl GenericRepository<FxRate> for FxRateRepository {
    fn get_pool(&self) -> &MySqlPool {
        &self.pool
    }
}

#[async_trait]
impl FxRateRepositoryInterface for FxRateRepository {
    async fn get(&self, fx_rate_id: Uuid, meta_user: Option<Uuid>) -> Result<Option<FxRate>, Error> {
        let params = vec![
            MySqlParam::from(ub(fx_rate_id)),
            MySqlParam::from(oub(meta_user)),
        ];
        
        self.call_procedure_for_optional("proc_fx_rate_get_by_id", params).await
    }

    async fn create(&self, fx_rate: FxRate, meta_user: Option<Uuid>) -> Result<FxRate, Error> {
        let params = vec![
            MySqlParam::from(fx_rate.base_code),
            MySqlParam::from(fx_rate.quote_code),
            MySqlParam::from(fx_rate.rate),
            MySqlParam::from(fx_rate.as_of_date),
            MySqlParam::from(fx_rate.source),
            MySqlParam::from(oub(meta_user)),
        ];
        
        self.call_procedure_for_one("proc_fx_rate_create", params).await
    }

    async fn update(&self, fx_rate_id: Uuid, rate: Decimal, meta_user: Option<Uuid>) -> Result<Option<FxRate>, Error> {
        let params = vec![
            MySqlParam::from(ub(fx_rate_id)),
            MySqlParam::from(rate),
            MySqlParam::from(oub(meta_user)),
        ];
        
        self.call_procedure_for_optional("proc_fx_rate_update_rate", params).await
    }

    async fn delete(&self, fx_rate_id: Uuid, meta_user: Option<Uuid>) -> Result<(), Error> {
        let params = vec![
            MySqlParam::from(ub(fx_rate_id)),
            MySqlParam::from(oub(meta_user)),
        ];
        
        self.call_procedure("proc_fx_rate_delete", params).await
    }

    async fn get_all(&self, meta_user: Option<Uuid>) -> Result<Vec<FxRate>, Error> {
        let params = vec![
            MySqlParam::from(oub(meta_user)),
        ];
        
        self.call_procedure_for_list("proc_fx_rate_list", params).await
    }

    async fn get_by_base_code(&self, base_code: String, meta_user: Option<Uuid>) -> Result<Vec<FxRate>, Error> {
        let params = vec![
            MySqlParam::from(base_code),
            MySqlParam::from(oub(meta_user)),
        ];

        self.call_procedure_for_list("proc_fx_rate_by_code", params).await
    }
}

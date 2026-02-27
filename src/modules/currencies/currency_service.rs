use anyhow::Result;
use async_trait::async_trait;
use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use uuid::Uuid;

use crate::modules::currencies::{
    currency_command::*,
    currency_dto::{CurrencyResponse, FxRateResponse},
    currency_model::{Currency, FxRate},
    currency_repo::{
        CurrencyRepository, CurrencyRepositoryInterface,
        FxRateRepository, FxRateRepositoryInterface
    },
};
use crate::shared::db::redis::{delete_key, get_key, set_key};
use crate::shared::errors::AppError;
use crate::shared::state::AppState;

#[async_trait]
pub trait CurrencyServiceInterface {

    // --- Currency ---

    async fn get_currency(&self, command: CurrencyGetCommand) -> Result<Option<CurrencyResponse>, AppError>;

    async fn create_currency(&self, command: CurrencyCreateCommand) -> Result<CurrencyResponse, AppError>;
    
    async fn update_currency_name(&self, command: CurrencyUpdateNameCommand) -> Result<Option<CurrencyResponse>, AppError>;
    
    async fn delete_currency(&self, command: CurrencyDeleteCommand) -> Result<(), AppError>;
    
    async fn list_currencies(&self, command: CurrencyListCommand) -> Result<Vec<CurrencyResponse>, AppError>;

    
    // --- FxRate ---
    
    async fn get_fx_rate(&self, command: FxRateGetCommand) -> Result<Option<FxRateResponse>, AppError>;
    
    async fn create_fx_rate(&self, command: FxRateCreateCommand) -> Result<Option<FxRateResponse>, AppError>;
    
    async fn update_fx_rate(&self, command: FxRateUpdateRateCommand) -> Result<Option<FxRateResponse>, AppError>;
    
    async fn delete_fx_rate(&self, command: FxRateDeleteCommand) -> Result<(), AppError>;
    
    async fn list_fx_rates(&self, command: FxRateListCommand) -> Result<Vec<FxRateResponse>, AppError>;
    
    async fn list_fx_rates_by_base_code(&self, command: FxRateByBaseCodeCommand) -> Result<Option<Vec<FxRateResponse>>, AppError>;
    
}

#[derive(Clone)]
pub struct CurrencyService {
    currency_repo: CurrencyRepository,
    fx_rate_repo: FxRateRepository,
    redis_pool: Option<Pool<RedisConnectionManager>>,
}

impl From<&AppState> for CurrencyService {
    fn from(app_state: &AppState) -> Self {
        let currency_repo = CurrencyRepository::from(app_state);
        let fx_rate_repo = FxRateRepository::from(app_state);
        Self { currency_repo, fx_rate_repo, redis_pool: Option::from(app_state.redis_pool.clone()) }
    }
}

impl CurrencyService {
    pub fn redis_key_currency_ttl(&self) -> Option<u64> {
        Some(60*60)
    }

    pub fn form_redis_key_single_currency(&self, code: &String) -> String {
        format!("currency:{}", code)
    }
    
    pub fn form_redis_key_list_currencies(&self) -> String {
        "currency:list".to_string()
    }
    
    
    pub fn redis_key_fx_rate_ttl(&self) -> Option<u64> {
        Some(60*60)
    }

    pub fn form_redis_key_single_fx_rate(&self, key: &Uuid) -> String {
        format!("fx_rate:{}", key)
    }

    pub fn form_redis_key_list_fx_rates(&self) -> String {
        "fx_rate:list".to_string()
    }
}

#[async_trait]
impl CurrencyServiceInterface for CurrencyService {
    async fn get_currency(&self, command: CurrencyGetCommand) -> Result<Option<CurrencyResponse>, AppError> {
        if let Some(redis_pool) = &self.redis_pool {
            let currency_cache: Option<CurrencyResponse> = get_key(
                &redis_pool,
                self.form_redis_key_single_currency(&command.code).as_str()
            ).await?;
            if let Some(currency) = currency_cache {
                return Ok(Some(currency));
            }
        }
        
        let currency = self.currency_repo.get(command.code).await?;
        match currency {
            Some(currency) => {
                let currency_response = CurrencyResponse::from(currency);
                if let Some(redis_pool) = &self.redis_pool {
                    let _: () = set_key(
                        &redis_pool,
                        self.form_redis_key_single_currency(&currency_response.code).as_str(),
                        &currency_response,
                        self.redis_key_currency_ttl()
                    ).await?;
                }
                Ok(Some(currency_response))
            },
            None => Ok(None)
        }
    }

    async fn create_currency(&self, command: CurrencyCreateCommand) -> Result<CurrencyResponse, AppError> {
        let currency_create = Currency::from(command);
        
        let currency = self.currency_repo.create(currency_create).await?;
        let currency_response = CurrencyResponse::from(currency);
        if let Some(redis_pool) = &self.redis_pool {
            let _: () = set_key(
                &redis_pool,
                self.form_redis_key_single_currency(&currency_response.code).as_str(),
                &currency_response,
                self.redis_key_currency_ttl()
            ).await?;
        }
        Ok(currency_response)
    }

    async fn update_currency_name(&self, command: CurrencyUpdateNameCommand) -> Result<Option<CurrencyResponse>, AppError> {
        let currency = self.currency_repo.update_name(command.code, command.name).await?;
        match currency {
            Some(currency) => {
                let currency_response = CurrencyResponse::from(currency);
                if let Some(redis_pool) = &self.redis_pool {
                    let _: () = set_key(
                        &redis_pool,
                        self.form_redis_key_single_currency(&currency_response.code).as_str(),
                        &currency_response,
                        self.redis_key_currency_ttl()
                    ).await?;
                }
                Ok(Some(currency_response))
            },
            None => Ok(None)
        }
    }

    async fn delete_currency(&self, command: CurrencyDeleteCommand) -> Result<(), AppError> {
        self.currency_repo.delete(command.code.clone()).await?;

        if let Some(redis_pool) = &self.redis_pool {
            let _: () = delete_key(&redis_pool, self.form_redis_key_single_currency(&command.code).as_str()).await?;
        }

        Ok(())
    }

    async fn list_currencies(&self, command: CurrencyListCommand) -> Result<Vec<CurrencyResponse>, AppError> {
        if let Some(redis_pool) = &self.redis_pool {
            let currencies_cache: Option<Vec<CurrencyResponse>> = get_key(
                &redis_pool,
                self.form_redis_key_list_currencies().as_str()
            ).await?;
            if let Some(currencies) = currencies_cache {
                return Ok(currencies);
            }
        }
        
        let currencies = self.currency_repo.get_all().await?;

        let currencies_response = currencies.into_iter().map(CurrencyResponse::from).collect();
        if let Some(redis_pool) = &self.redis_pool {
            let _: () = set_key(
                &redis_pool,
                self.form_redis_key_list_currencies().as_str(),
                &currencies_response,
                self.redis_key_currency_ttl()
            ).await?;
        }
        Ok(currencies_response)
    }

    async fn get_fx_rate(&self, command: FxRateGetCommand) -> Result<Option<FxRateResponse>, AppError> {
        if let Some(redis_pool) = &self.redis_pool {
            let fx_rate_cache: Option<FxRateResponse> = get_key(
                &redis_pool,
                self.form_redis_key_single_fx_rate(&command.fx_rate_id).as_str()
            ).await?;
            if let Some(fx_rate) = fx_rate_cache {
                return Ok(Some(fx_rate));
            }
        }
        
        let fx_rate = self.fx_rate_repo.get(command.fx_rate_id).await?;

        match fx_rate {
            Some(fx_rate) => {
                let fx_rate_response = FxRateResponse::from(fx_rate);
                if let Some(redis_pool) = &self.redis_pool {
                    let _: () = set_key(
                        &redis_pool,
                        self.form_redis_key_single_fx_rate(&command.fx_rate_id).as_str(),
                        &fx_rate_response,
                        self.redis_key_fx_rate_ttl()
                    ).await?;
                }
                Ok(Some(fx_rate_response))
            },
            None => Ok(None)
        }
    }

    async fn create_fx_rate(&self, command: FxRateCreateCommand) -> Result<Option<FxRateResponse>, AppError> {
        let fx_rate_base_code = command.base_code.clone();
        let fx_rate_quote_code = command.quote_code.clone();
        
        let currency_base_code = self.currency_repo.get(fx_rate_base_code).await?;
        if currency_base_code.is_none() {
            return Ok(None);
        }
        
        let currency_quote_code = self.currency_repo.get(fx_rate_quote_code).await?;
        if currency_quote_code.is_none() {
            return Ok(None);
        }
        
        let fx_rate_create = FxRate::from(command);
        
        let fx_rate = self.fx_rate_repo.create(fx_rate_create).await?;

        let fx_rate_response = FxRateResponse::from(fx_rate);
        if let Some(redis_pool) = &self.redis_pool {
            let _: () = set_key(
                &redis_pool,
                self.form_redis_key_single_fx_rate(&fx_rate_response.fx_rate_id).as_str(),
                &fx_rate_response,
                self.redis_key_fx_rate_ttl()
            ).await?;
        }
        Ok(Some(fx_rate_response))
    }

    async fn update_fx_rate(&self, command: FxRateUpdateRateCommand) -> Result<Option<FxRateResponse>, AppError> {
        let fx_rate = self.fx_rate_repo.update(command.fx_rate_id, command.rate).await?;

        match fx_rate {
            Some(fx_rate) => {
                let fx_rate_response = FxRateResponse::from(fx_rate);
                if let Some(redis_pool) = &self.redis_pool {
                    let _: () = set_key(
                        &redis_pool,
                        self.form_redis_key_single_fx_rate(&fx_rate_response.fx_rate_id).as_str(),
                        &fx_rate_response,
                        self.redis_key_fx_rate_ttl()
                    ).await?;
                }
                Ok(Some(fx_rate_response))
            },
            None => Ok(None)
        }
    }

    async fn delete_fx_rate(&self, command: FxRateDeleteCommand) -> Result<(), AppError> {
        self.fx_rate_repo.delete(command.fx_rate_id).await?;

        if let Some(redis_pool) = &self.redis_pool {
            let _: () = delete_key(&redis_pool, self.form_redis_key_single_fx_rate(&command.fx_rate_id).as_str()).await?;
        }

        Ok(())
    }

    async fn list_fx_rates(&self, command: FxRateListCommand) -> Result<Vec<FxRateResponse>, AppError> {
        if let Some(redis_pool) = &self.redis_pool {
            let fx_rates_cache : Option<Vec<FxRateResponse>> = get_key(
                &redis_pool,
                self.form_redis_key_list_fx_rates().as_str()
            ).await?;
            if let Some(fx_rates) = fx_rates_cache {
                return Ok(fx_rates);
            }
        }
        
        let fx_rates = self.fx_rate_repo.get_all().await?;

        let fx_rates_response = fx_rates.into_iter().map(FxRateResponse::from).collect();
        if let Some(redis_pool) = &self.redis_pool {
            let _: () = set_key(
                &redis_pool,
                self.form_redis_key_list_fx_rates().as_str(),
                &fx_rates_response,
                self.redis_key_fx_rate_ttl()
            ).await?;
        }
        Ok(fx_rates_response)
    }

    async fn list_fx_rates_by_base_code(&self, command: FxRateByBaseCodeCommand) -> Result<Option<Vec<FxRateResponse>>, AppError> {
        let fx_rate_base_code = command.base_code.clone();
        let currency_base_code = self.currency_repo.get(fx_rate_base_code).await?;

        if currency_base_code.is_none() {
            return Ok(None);
        }
        
        let fx_rates = self.fx_rate_repo.get_by_base_code(command.base_code).await?;

        let fx_rates_response = fx_rates.into_iter().map(FxRateResponse::from).collect();
        Ok(Some(fx_rates_response))
    }

}

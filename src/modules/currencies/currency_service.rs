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
        Self {
            currency_repo: CurrencyRepository::from(app_state),
            fx_rate_repo: FxRateRepository::from(app_state),
            redis_pool: app_state.redis_pool.clone()
        }
    }
}

impl CurrencyService {
    pub fn redis_key_currency_ttl(&self) -> Option<u64> {
        Some(60*60)
    }

    pub fn form_redis_key_currency(&self, code: &String) -> String {
        format!("currency:{}", code)
    }
    
    pub fn form_redis_key_currencies(&self) -> String {
        "currency:list".to_string()
    }
    
    
    pub fn redis_key_fx_rate_ttl(&self) -> Option<u64> {
        Some(60*60)
    }

    pub fn form_redis_key_fx_rate(&self, key: &Uuid) -> String {
        format!("fx_rate:{}", key)
    }

    pub fn form_redis_key_fx_rates(&self, base_code: &String) -> String {
        format!("currency:{}:fx_rate", base_code)
    }
    
    async fn cache_currency(&self, currency: &CurrencyResponse) -> Result<(), AppError> {
        if let Some(redis_pool) = &self.redis_pool {
            let _: () = set_key(
                &redis_pool,
                self.form_redis_key_currency(&currency.code).as_str(),
                &currency,
                self.redis_key_currency_ttl()
            ).await.map_err(AppError::Internal)?;
        }
        Ok(())
    }
    
    async fn cache_currencies(&self, currencies: &Vec<CurrencyResponse>) -> Result<(), AppError> {
        if let Some(redis_pool) = &self.redis_pool {
            let _: () = set_key(
                &redis_pool,
                self.form_redis_key_currencies().as_str(),
                &currencies,
                self.redis_key_currency_ttl()
            ).await.map_err(AppError::Internal)?;
        }
        Ok(())
    }
    
    async fn cache_fx_rate(&self, fx_rate: &FxRateResponse) -> Result<(), AppError> {
        if let Some(redis_pool) = &self.redis_pool {
            let _: () = set_key(
                &redis_pool,
                self.form_redis_key_fx_rate(&fx_rate.fx_rate_id).as_str(),
                &fx_rate,
                self.redis_key_fx_rate_ttl()
            ).await.map_err(AppError::Internal)?;
        }
        Ok(())
    }
    
    async fn cache_fx_rates(&self, fx_rates: &Vec<FxRateResponse>, base_code: &String) -> Result<(), AppError> {
        if let Some(redis_pool) = &self.redis_pool {
            let _: () = set_key(
                &redis_pool,
                self.form_redis_key_fx_rates(&base_code).as_str(),
                &fx_rates,
                self.redis_key_fx_rate_ttl()
            ).await.map_err(AppError::Internal)?;
        }
        Ok(())
    }
    
    async fn delete_cache_currency(&self, code: &String) -> Result<(), AppError> {
        if let Some(redis_pool) = &self.redis_pool {
            let _: () = delete_key(&redis_pool, self.form_redis_key_currency(code).as_str()).await
                .map_err(AppError::Internal)?;
            let _: () = delete_key(&redis_pool, self.form_redis_key_currencies().as_str()).await
                .map_err(AppError::Internal)?;
        }
        Ok(())
    }
    
    async fn delete_cache_currencies(&self) -> Result<(), AppError> {
        if let Some(redis_pool) = &self.redis_pool {
            let _: () = delete_key(&redis_pool, self.form_redis_key_currencies().as_str()).await
                .map_err(AppError::Internal)?;
        }
        Ok(())
    }
    
    async fn delete_cache_fx_rate(&self, fx_rate_id: &Uuid, base_code: &String) -> Result<(), AppError> {
        if let Some(redis_pool) = &self.redis_pool {
            let _: () = delete_key(&redis_pool, self.form_redis_key_fx_rate(&fx_rate_id).as_str()).await
                .map_err(AppError::Internal)?;
            let _: () = delete_key(&redis_pool, self.form_redis_key_fx_rates(&base_code).as_str()).await
                .map_err(AppError::Internal)?;
        }
        Ok(())
    }
    
    async fn delete_cache_fx_rates(&self, base_code: &String) -> Result<(), AppError> {
        if let Some(redis_pool) = &self.redis_pool {
            let _: () = delete_key(&redis_pool, self.form_redis_key_fx_rates(&base_code).as_str()).await
                .map_err(AppError::Internal)?;
        }
        Ok(())
    }
    
    async fn get_currency_cache(&self, code: &String) -> Result<Option<CurrencyResponse>, AppError> {
        if let Some(redis_pool) = &self.redis_pool {
            let currency_cache: Option<CurrencyResponse> = get_key(
                &redis_pool,
                self.form_redis_key_currency(code).as_str()
            ).await.map_err(AppError::Internal)?;
            return Ok(currency_cache);
        }
        Ok(None)
    }
    
    async fn get_currencies_cache(&self) -> Result<Option<Vec<CurrencyResponse>>, AppError> {
        if let Some(redis_pool) = &self.redis_pool {
            let currencies_cache: Option<Vec<CurrencyResponse>> = get_key(
                &redis_pool,
                self.form_redis_key_currencies().as_str()
            ).await?;
            return Ok(currencies_cache);
        }
        Ok(None)
    }
    
    async fn get_fx_rate_cache(&self, fx_rate_id: &Uuid) -> Result<Option<FxRateResponse>, AppError> {
        if let Some(redis_pool) = &self.redis_pool {
            let fx_rate_cache: Option<FxRateResponse> = get_key(
                &redis_pool,
                self.form_redis_key_fx_rate(&fx_rate_id).as_str()
            ).await.map_err(AppError::Internal)?;
            return Ok(fx_rate_cache);
        }
        Ok(None)
    }
    
    async fn get_fx_rates_cache(&self, base_code: &String) -> Result<Option<Vec<FxRateResponse>>, AppError> {
        if let Some(redis_pool) = &self.redis_pool {
            let fx_rates_cache: Option<Vec<FxRateResponse>> = get_key(
                &redis_pool,
                self.form_redis_key_fx_rates(&base_code).as_str()
            ).await.map_err(AppError::Internal)?;
            return Ok(fx_rates_cache);
        }
        Ok(None)
    }
    
    async fn handle_res_opt_currency(&self, currency: Result<Option<Currency>>, delete_cache_list: bool) -> Result<Option<CurrencyResponse>, AppError> {
        let currency = currency.map_err(AppError::Internal)?;
        
        if let Some(cur) = currency {
            let response = CurrencyResponse::from(cur);
            self.cache_currency(&response).await?;
            if delete_cache_list { self.delete_cache_currencies().await?; }
            
            Ok(Some(response))
        } else {
            Ok(None)
        }
    }
    
    async fn handle_res_opt_fx_rate(&self, fx_rate: Result<Option<FxRate>>, delete_cache_list: bool) -> Result<Option<FxRateResponse>, AppError> {
        let fx_rate = fx_rate.map_err(AppError::Internal)?;
        
        if let Some(ft) = fx_rate {
            let response = FxRateResponse::from(ft);
            self.cache_fx_rate(&response).await?;
            if delete_cache_list { self.delete_cache_fx_rates(&response.base_code).await?; }
            
            Ok(Some(response))
        } else {
            Ok(None)
        }
    }

}

#[async_trait]
impl CurrencyServiceInterface for CurrencyService {
    async fn get_currency(&self, command: CurrencyGetCommand) -> Result<Option<CurrencyResponse>, AppError> {
        let currency_cache = self.get_currency_cache(&command.code).await?;
        if let Some(currency) = currency_cache {
            return Ok(Some(currency));
        }
        
        let currency = self.currency_repo.get(command.code).await;
        self.handle_res_opt_currency(currency, false).await
    }

    async fn create_currency(&self, command: CurrencyCreateCommand) -> Result<CurrencyResponse, AppError> {
        let currency_create = Currency::from(command);
        
        let currency = self.currency_repo.create(currency_create).await?;
        let currency_response = CurrencyResponse::from(currency);
        
        self.cache_currency(&currency_response).await?;
        self.delete_cache_currencies().await?;
        
        Ok(currency_response)
    }

    async fn update_currency_name(&self, command: CurrencyUpdateNameCommand) -> Result<Option<CurrencyResponse>, AppError> {
        let currency = self.currency_repo.update_name(command.code, command.name).await;
        self.handle_res_opt_currency(currency, true).await
    }

    async fn delete_currency(&self, command: CurrencyDeleteCommand) -> Result<(), AppError> {
        self.currency_repo.delete(command.code.clone()).await?;

        self.delete_cache_currency(&command.code).await?;

        Ok(())
    }

    async fn list_currencies(&self, _command: CurrencyListCommand) -> Result<Vec<CurrencyResponse>, AppError> {
        let currencies_cache = self.get_currencies_cache().await?;
        if let Some(currencies) = currencies_cache {
            return Ok(currencies);
        }
        
        let currencies = self.currency_repo.get_all().await?;

        let currencies_response = currencies.into_iter().map(CurrencyResponse::from).collect();
        self.cache_currencies(&currencies_response).await?;
        
        Ok(currencies_response)
    }
    
    

    async fn get_fx_rate(&self, command: FxRateGetCommand) -> Result<Option<FxRateResponse>, AppError> {
        let fx_rate_cache = self.get_fx_rate_cache(&command.fx_rate_id).await?;
        if let Some(fx_rate) = fx_rate_cache {
            return Ok(Some(fx_rate));
        }
        
        let fx_rate = self.fx_rate_repo.get(command.fx_rate_id).await;
        self.handle_res_opt_fx_rate(fx_rate, false).await
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
        self.cache_fx_rate(&fx_rate_response).await?;
        Ok(Some(fx_rate_response))
    }

    async fn update_fx_rate(&self, command: FxRateUpdateRateCommand) -> Result<Option<FxRateResponse>, AppError> {
        let fx_rate = self.fx_rate_repo.update(command.fx_rate_id, command.rate).await;
        self.handle_res_opt_fx_rate(fx_rate, true).await
    }

    async fn delete_fx_rate(&self, command: FxRateDeleteCommand) -> Result<(), AppError> {
        let fx_rate = self.fx_rate_repo.get(command.fx_rate_id).await?;
        
        if let Some(fx_rate) = fx_rate {
            self.fx_rate_repo.delete(command.fx_rate_id).await?;

            self.delete_cache_fx_rate(&command.fx_rate_id, &fx_rate.base_code).await?;
        }

        Ok(())
    }

    async fn list_fx_rates_by_base_code(&self, command: FxRateByBaseCodeCommand) -> Result<Option<Vec<FxRateResponse>>, AppError> {
        let fx_rate_base_code = command.base_code.clone();
        
        let fx_rates_cache = self.get_fx_rates_cache(&fx_rate_base_code).await?;
        if let Some(fx_rates) = fx_rates_cache {
            return Ok(Some(fx_rates));
        }
        
        let currency_base_code = self.currency_repo.get(fx_rate_base_code.clone()).await?;

        if currency_base_code.is_none() {
            return Ok(None);
        }
        
        let fx_rates = self.fx_rate_repo.get_by_base_code(command.base_code).await?;

        let fx_rates_response = fx_rates.into_iter().map(FxRateResponse::from).collect();
        self.cache_fx_rates(&fx_rates_response, &fx_rate_base_code).await?;
        
        Ok(Some(fx_rates_response))
    }

}

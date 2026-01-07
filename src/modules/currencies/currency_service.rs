use anyhow::{Error, Result};
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
use crate::shared::state::AppState;

#[async_trait]
pub trait CurrencyServiceInterface {

    // --- Currency ---

    async fn get_currency(&self, command: CurrencyGetCommand) -> Result<Option<CurrencyResponse>, Error>;

    async fn create_currency(&self, command: CurrencyCreateCommand) -> Result<CurrencyResponse, Error>;
    
    async fn update_currency_name(&self, command: CurrencyUpdateNameCommand) -> Result<Option<CurrencyResponse>, Error>;
    
    async fn delete_currency(&self, command: CurrencyDeleteCommand) -> Result<(), Error>;
    
    async fn list_currencies(&self, command: CurrencyListCommand) -> Result<Vec<CurrencyResponse>, Error>;

    
    // --- FxRate ---
    
    async fn get_fx_rate(&self, command: FxRateGetCommand) -> Result<Option<FxRateResponse>, Error>;
    
    async fn create_fx_rate(&self, command: FxRateCreateCommand) -> Result<Option<FxRateResponse>, Error>;
    
    async fn update_fx_rate(&self, command: FxRateUpdateRateCommand) -> Result<Option<FxRateResponse>, Error>;
    
    async fn delete_fx_rate(&self, command: FxRateDeleteCommand) -> Result<(), Error>;
    
    async fn list_fx_rates(&self, command: FxRateListCommand) -> Result<Vec<FxRateResponse>, Error>;
    
    async fn list_fx_rates_by_base_code(&self, command: FxRateByBaseCodeCommand) -> Result<Option<Vec<FxRateResponse>>, Error>;
    
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
    async fn get_currency(&self, command: CurrencyGetCommand) -> Result<Option<CurrencyResponse>, Error> {
        if let Some(redis_pool) = &self.redis_pool {
            let currency_cache: Option<CurrencyResponse> = get_key(
                &redis_pool,
                self.form_redis_key_single_currency(&command.currency_code).as_str()
            ).await?;
            if let Some(currency) = currency_cache {
                return Ok(Some(currency));
            }
        }
        
        let currency = self.currency_repo.get(command.currency_code, Some(command.auth_user.user_id)).await;
        match currency {
            Ok(currency) => {
                match currency {
                    Some(currency) => {
                        let currency_response = CurrencyResponse::from(currency);
                        if let Some(redis_pool) = &self.redis_pool {
                            let _: () = set_key(
                                &redis_pool,
                                self.form_redis_key_single_currency(&currency_response.currency_code).as_str(),
                                &currency_response,
                                self.redis_key_currency_ttl()
                            ).await?;
                        }
                        Ok(Some(currency_response))
                    },
                    None => Ok(None)
                }
            },
            Err(_) => Err(Error::msg("Error during getting currency"))
        }
    }

    async fn create_currency(&self, command: CurrencyCreateCommand) -> Result<CurrencyResponse, Error> {
        let meta_user = command.auth_user.user_id.clone();
        let currency_create = Currency::from(command);
        
        let currency = self.currency_repo.create(currency_create, Some(meta_user)).await;
        match currency {
            Ok(currency) => {
                let currency_response = CurrencyResponse::from(currency);
                if let Some(redis_pool) = &self.redis_pool {
                    let _: () = set_key(
                        &redis_pool,
                        self.form_redis_key_single_currency(&currency_response.currency_code).as_str(),
                        &currency_response,
                        self.redis_key_currency_ttl()
                    ).await?;
                }
                Ok(currency_response)
            },
            Err(_) => Err(Error::msg("Error during creating currency"))
        }
    }

    async fn update_currency_name(&self, command: CurrencyUpdateNameCommand) -> Result<Option<CurrencyResponse>, Error> {
        let currency = self.currency_repo.update_name(command.currency_code, command.currency_name, Some(command.auth_user.user_id)).await;
        match currency {
            Ok(currency) => {
                match currency { 
                    Some(currency) => {
                        let currency_response = CurrencyResponse::from(currency);
                        if let Some(redis_pool) = &self.redis_pool {
                            let _: () = set_key(
                                &redis_pool,
                                self.form_redis_key_single_currency(&currency_response.currency_code).as_str(),
                                &currency_response,
                                self.redis_key_currency_ttl()
                            ).await?;
                        }
                        Ok(Some(currency_response))
                    },
                    None => Ok(None)
                }
            },
            Err(_) => Err(Error::msg("Error during updating currency"))
        }
    }

    async fn delete_currency(&self, command: CurrencyDeleteCommand) -> Result<(), Error> {
        let result = self.currency_repo.delete(command.currency_code.clone(), Some(command.auth_user.user_id)).await;
        if let Some(redis_pool) = &self.redis_pool {
            let _: () = delete_key(&redis_pool, self.form_redis_key_single_currency(&command.currency_code).as_str()).await?;
        }
        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(Error::msg("Error during deleting currency"))
        }
    }

    async fn list_currencies(&self, command: CurrencyListCommand) -> Result<Vec<CurrencyResponse>, Error> {
        if let Some(redis_pool) = &self.redis_pool {
            let currencies_cache: Option<Vec<CurrencyResponse>> = get_key(
                &redis_pool,
                self.form_redis_key_list_currencies().as_str()
            ).await?;
            if let Some(currencies) = currencies_cache {
                return Ok(currencies);
            }
        }
        
        let currencies = self.currency_repo.get_all(Some(command.auth_user.user_id)).await;
        match currencies {
            Ok(currencies) => {
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
            Err(_) => Err(Error::msg("Error during getting currency"))
        }
    }

    async fn get_fx_rate(&self, command: FxRateGetCommand) -> Result<Option<FxRateResponse>, Error> {
        if let Some(redis_pool) = &self.redis_pool {
            let fx_rate_cache: Option<FxRateResponse> = get_key(
                &redis_pool,
                self.form_redis_key_single_fx_rate(&command.fx_rate_id).as_str()
            ).await?;
            if let Some(fx_rate) = fx_rate_cache {
                return Ok(Some(fx_rate));
            }
        }
        
        let fx_rate = self.fx_rate_repo.get(command.fx_rate_id, Some(command.auth_user.user_id)).await;
        match fx_rate {
            Ok(fx_rate) => {
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
            },
            Err(_) => Err(Error::msg("Error during getting fx_rate"))
        }
    }

    async fn create_fx_rate(&self, command: FxRateCreateCommand) -> Result<Option<FxRateResponse>, Error> {
        let meta_user = command.auth_user.user_id.clone();
        let fx_rate_base_code = command.fx_rate_base_code.clone();
        let fx_rate_quote_code = command.fx_rate_quote_code.clone();
        
        let currency_base_code = self.currency_repo.get(fx_rate_base_code, Some(meta_user.clone())).await;
        match currency_base_code {
            Ok(currency) => {
                if currency.is_none() {
                    return Ok(None);
                }
            },
            Err(_) => return Err(Error::msg("Error during getting currency base code"))
        }
        
        let currency_quote_code = self.currency_repo.get(fx_rate_quote_code, Some(meta_user.clone())).await;
        match currency_quote_code {
            Ok(currency) => {
                if currency.is_none() {
                    return Ok(None);
                }
            },
            Err(_) => return Err(Error::msg("Error during getting currency quote code"))
        }
        
        let fx_rate_create = FxRate::from(command);
        
        let fx_rate = self.fx_rate_repo.create(fx_rate_create, Some(meta_user)).await;
        match fx_rate { 
            Ok(fx_rate) => {
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
            Err(_) => Err(Error::msg("Error during creating fx_rate"))
        }
    }

    async fn update_fx_rate(&self, command: FxRateUpdateRateCommand) -> Result<Option<FxRateResponse>, Error> {
        let fx_rate = self.fx_rate_repo.update(command.fx_rate_id, command.fx_rate_rate, Some(command.auth_user.user_id)).await;
        match fx_rate {
            Ok(fx_rate) => {
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
            },
            Err(_) => Err(Error::msg("Error during updating fx_rate"))
        }
    }

    async fn delete_fx_rate(&self, command: FxRateDeleteCommand) -> Result<(), Error> {
        let result = self.fx_rate_repo.delete(command.fx_rate_id, Some(command.auth_user.user_id)).await;
        if let Some(redis_pool) = &self.redis_pool {
            let _: () = delete_key(&redis_pool, self.form_redis_key_single_fx_rate(&command.fx_rate_id).as_str()).await?;
        }
        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(Error::msg("Error during deleting fx_rate"))
        }
    }

    async fn list_fx_rates(&self, command: FxRateListCommand) -> Result<Vec<FxRateResponse>, Error> {
        if let Some(redis_pool) = &self.redis_pool {
            let fx_rates_cache : Option<Vec<FxRateResponse>> = get_key(
                &redis_pool,
                self.form_redis_key_list_fx_rates().as_str()
            ).await?;
            if let Some(fx_rates) = fx_rates_cache {
                return Ok(fx_rates);
            }
        }
        
        let fx_rates = self.fx_rate_repo.get_all(Some(command.auth_user.user_id)).await;
        match fx_rates {
            Ok(fx_rates) => {
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
            },
            Err(_) => Err(Error::msg("Error getting fx rates"))
        }
    }

    async fn list_fx_rates_by_base_code(&self, command: FxRateByBaseCodeCommand) -> Result<Option<Vec<FxRateResponse>>, Error> {
        let meta_user = command.auth_user.user_id.clone();
        
        let fx_rate_base_code = command.fx_rate_base_code.clone();
        let currency_base_code = self.currency_repo.get(fx_rate_base_code, Some(meta_user.clone())).await;
        match currency_base_code {
            Ok(currency) => {
                if currency.is_none() {
                    return Ok(None);
                }
            },
            Err(_) => return Err(Error::msg("Error during getting currency base code"))
        }
        
        let fx_rates = self.fx_rate_repo.get_by_base_code(command.fx_rate_base_code, Some(meta_user)).await;
        match fx_rates { 
            Ok(fx_rates) => {
                let fx_rates_response = fx_rates.into_iter().map(FxRateResponse::from).collect();
                Ok(Some(fx_rates_response))
            },
            Err(_) => Err(Error::msg("Error getting fx rates"))
        }
    }
}

use anyhow::Result;
use async_trait::async_trait;
use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use uuid::Uuid;

use crate::modules::accounts::{
    account_command::*,
    account_dto::*,
    account_model::Account,
    account_repo::{AccountRepository, AccountRepositoryInterface}
};
use crate::shared::{
    db::redis::{delete_key, get_key, set_key},
    errors::AppError,
    state::AppState,
};

#[async_trait]
pub trait AccountInterface {

    async fn get(&self, command: AccountGetCommand) -> Result<Option<AccountResponse>, AppError>;

    async fn create(&self, command: AccountCreateCommand) -> Result<AccountResponse, AppError>;

    async fn update(&self, command: AccountUpdateCommand) -> Result<Option<AccountResponse>, AppError>;

    async fn archived(&self, command: AccountArchivedCommand) -> Result<Option<AccountResponse>, AppError>;

    async fn delete(&self, command: AccountDeleteCommand) -> Result<(), AppError>;

    async fn get_by_user(&self, command: AccountListByUserCommand) -> Result<Vec<AccountResponse>, AppError>;

}

#[derive(Clone)]
pub struct AccountService {
    account_repo: AccountRepository,
    redis_pool: Option<Pool<RedisConnectionManager>>,
}

impl From<&AppState> for AccountService {
    fn from(app_state: &AppState) -> Self {
        Self {
            account_repo: AccountRepository::from(app_state),
            redis_pool: app_state.redis_pool.clone()
        }
    }
}

impl AccountService {
    fn redis_key_ttl(&self) -> Option<u64> { Some(60 * 60) }

    fn form_redis_key_account(&self, key: &Uuid) -> String { format!("account:{}", key) }

    fn form_redis_key_list_by_user(&self, user: &Uuid) -> String { format!("user:{}:accounts", user) }

    async fn cache_account(&self, account: &AccountResponse) -> Result<(), AppError> {
        if let Some(redis_pool) = &self.redis_pool {
            let _: () = set_key(
                &redis_pool,
                self.form_redis_key_account(&account.account_id).as_str(),
                &account,
                self.redis_key_ttl()
            ).await.map_err(AppError::Internal)?;
        }
        Ok(())
    }

    async fn cache_accounts_by_user(&self, user: &Uuid, accounts: &Vec<AccountResponse>) -> Result<(), AppError> {
        if let Some(redis_pool) = &self.redis_pool {
            let _: () = set_key(
                &redis_pool,
                self.form_redis_key_list_by_user(user).as_str(),
                &accounts,
                self.redis_key_ttl()
            ).await.map_err(AppError::Internal)?;
        }
        Ok(())
    }

    async fn get_cache_account(&self, key: &Uuid) -> Result<Option<AccountResponse>, AppError> {
        if let Some(redis_pool) = &self.redis_pool {
            let account_cache: Option<AccountResponse> = get_key(
                &redis_pool,
                self.form_redis_key_account(key).as_str()
            ).await.map_err(AppError::Internal)?;
            return Ok(account_cache);
        }
        Ok(None)
    }

    async fn get_cache_by_user(&self, user: &Uuid) -> Result<Option<Vec<AccountResponse>>, AppError> {
        if let Some(redis_pool) = &self.redis_pool {
            let cache: Option<Vec<AccountResponse>> = get_key(
                &redis_pool, self.form_redis_key_list_by_user(user).as_str()
            ).await.map_err(AppError::Internal)?;

            return Ok(cache);
        }

        Ok(None)
    }

    async fn delete_cache(&self, key: &Uuid, user: &Uuid) -> Result<(), AppError> {
        if let Some(redis_pool) = &self.redis_pool {
            let _: () = delete_key(&redis_pool, self.form_redis_key_account(key).as_str()).await
                .map_err(AppError::Internal)?;
            let _: () = delete_key(&redis_pool, self.form_redis_key_list_by_user(user).as_str()).await
                .map_err(AppError::Internal)?;
        }
        Ok(())
    }

    async fn delete_cache_list(&self, user: &Uuid) -> Result<(), AppError> {
        if let Some(redis_pool) = &self.redis_pool {
            let _: () = delete_key(&redis_pool, self.form_redis_key_list_by_user(user).as_str()).await
                .map_err(AppError::Internal)?;
        }
        Ok(())
    }

    async fn handle_res_opt_account(&self, account: Option<Account>, user: &Uuid) -> Result<Option<AccountResponse>, AppError> {
        if let Some(acc) = account {
            let response = AccountResponse::from(acc);
            self.cache_account(&response).await?;
            self.delete_cache_list(user).await?;

            Ok(Some(response))
        } else {
            Ok(None)
        }
    }
}

#[async_trait]
impl AccountInterface for AccountService {
    async fn get(&self, command: AccountGetCommand) -> Result<Option<AccountResponse>, AppError> {
        let cache = self.get_cache_account(&command.account_id).await?;
        if let Some(account) = cache {
            return Ok(Some(account));
        }

        let account = self.account_repo.get(command.account_id, command.auth_user.user_id).await?;

        self.handle_res_opt_account(account, &command.auth_user.user_id).await
    }

    async fn create(&self, command: AccountCreateCommand) -> Result<AccountResponse, AppError> {
        let meta_user = command.auth_user.user_id.clone();
        let account_create = Account::from(command);

        let account = self.account_repo.create(account_create, meta_user).await
            .map_err(AppError::Internal)?;
        let response = AccountResponse::from(account);

        self.cache_account(&response).await?;
        self.delete_cache_list(&meta_user).await?;

        Ok(response)
    }

    async fn update(&self, command: AccountUpdateCommand) -> Result<Option<AccountResponse>, AppError> {
        let account = self.account_repo.update(
            command.account_id, command.name, command.account_type,
            command.institution, command.auth_user.user_id
        ).await?;

        self.handle_res_opt_account(account, &command.auth_user.user_id).await
    }

    async fn archived(&self, command: AccountArchivedCommand) -> Result<Option<AccountResponse>, AppError> {
        let account = self.account_repo.archived(
            command.account_id, command.archived, command.auth_user.user_id
        ).await?;

        self.handle_res_opt_account(account, &command.auth_user.user_id).await
    }

    async fn delete(&self, command: AccountDeleteCommand) -> Result<(), AppError> {
        self.account_repo.delete(command.account_id.clone(), command.auth_user.user_id).await
            .map_err(AppError::Internal)?;

        self.delete_cache(&command.account_id, &command.auth_user.user_id).await?;

        Ok(())
    }

    async fn get_by_user(&self, command: AccountListByUserCommand) -> Result<Vec<AccountResponse>, AppError> {
        let cache = self.get_cache_by_user(&command.user_id).await?;
        if let Some(accounts) = cache { return Ok(accounts); }

        let accounts = self.account_repo.get_by_user(command.user_id).await
            .map_err(AppError::Internal)?;

        let response: Vec<AccountResponse> = accounts.into_iter().map(AccountResponse::from).collect();
        self.cache_accounts_by_user(&command.user_id, &response).await?;

        Ok(response)
    }
}

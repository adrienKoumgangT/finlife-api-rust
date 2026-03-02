use anyhow::Result;
use async_trait::async_trait;
use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use uuid::Uuid;

use crate::modules::payees::{
    payee_command::*,
    payee_dto::*,
    payee_model::Payee,
    payee_repo::{PayeeRepository, PayeeRepositoryInterface}
};
use crate::shared::{
    db::redis::{delete_key, get_key, set_key},
    errors::AppError,
    state::AppState,
    utils::extract_pagination_data
};


#[async_trait]
pub trait PayeeInterface {

    async fn get(&self, command: PayeeGetCommand) -> Result<Option<PayeeResponse>, AppError>;

    async fn create(&self, command: PayeeCreateCommand) -> Result<PayeeResponse, AppError>;

    async fn update(&self, command: PayeeUpdateCommand) -> Result<Option<PayeeResponse>, AppError>;

    async fn delete(&self, command: PayeeDeleteCommand) -> Result<(), AppError>;

    async fn get_by_user(&self, command: PayeeListByUserCommand) -> Result<Vec<PayeeResponse>, AppError>;

}

#[derive(Clone)]
pub struct PayeeService {
    payee_repo: PayeeRepository,
    redis_pool: Option<Pool<RedisConnectionManager>>,
}

impl From<&AppState> for PayeeService {
    fn from(app_state: &AppState) -> Self {
        Self {
            payee_repo: PayeeRepository::from(app_state),
            redis_pool: app_state.redis_pool.clone()
        }
    }
}

impl PayeeService {
    fn redis_key_ttl(&self) -> Option<u64> { Some(60 * 60) }

    fn form_redis_key_payee(&self, key: &Uuid) -> String { format!("payee:{}", key) }

    fn form_redis_key_list_by_user(&self, user: &Uuid) -> String { format!("user:{}:payees", user) }

    async fn cache_payee(&self, payee: &PayeeResponse) -> Result<(), AppError> {
        if let Some(redis_pool) = &self.redis_pool {
            let _: () = set_key(
                &redis_pool,
                self.form_redis_key_payee(&payee.payee_id).as_str(),
                &payee,
                self.redis_key_ttl()
            ).await.map_err(AppError::Internal)?;
        }
        Ok(())
    }

    async fn cache_payees_by_user(&self, user: &Uuid, payees: &Vec<PayeeResponse>) -> Result<(), AppError> {
        if let Some(redis_pool) = &self.redis_pool {
            let _: () = set_key(
                &redis_pool,
                self.form_redis_key_list_by_user(user).as_str(),
                &payees,
                self.redis_key_ttl()
            ).await.map_err(AppError::Internal)?;
        }
        Ok(())
    }

    async fn get_cache_payee(&self, key: &Uuid) -> Result<Option<PayeeResponse>, AppError> {
        if let Some(redis_pool) = &self.redis_pool {
            let payee_cache: Option<PayeeResponse> = get_key(
                &redis_pool,
                self.form_redis_key_payee(key).as_str()
            ).await.map_err(AppError::Internal)?;
            return Ok(payee_cache);
        }
        Ok(None)
    }

    async fn delete_cache(&self, key: &Uuid, user: &Uuid) -> Result<(), AppError> {
        if let Some(redis_pool) = &self.redis_pool {
            let _: () = delete_key(&redis_pool, self.form_redis_key_payee(key).as_str()).await
                .map_err(AppError::Internal)?;
            let _: () = delete_key(&redis_pool, self.form_redis_key_list_by_user(user).as_str()).await
                .map_err(AppError::Internal)?;
        }
        Ok(())
    }

    async fn handle_res_opt_payee(&self, payee: anyhow::Result<Option<Payee>>, auth_user: &Uuid) -> Result<Option<PayeeResponse>, AppError> {
        let payee = payee.map_err(AppError::Internal)?;

        if let Some(p) = payee {
            let response = PayeeResponse::from(p);
            self.cache_payee(&response).await?;

            // Invalidate the list cache whenever a payee is updated
            if let Some(redis_pool) = &self.redis_pool {
                let _: () = delete_key(&redis_pool, self.form_redis_key_list_by_user(auth_user).as_str()).await
                    .map_err(AppError::Internal)?;
            }

            Ok(Some(response))
        } else {
            Ok(None)
        }
    }
}

#[async_trait]
impl PayeeInterface for PayeeService {
    async fn get(&self, command: PayeeGetCommand) -> Result<Option<PayeeResponse>, AppError> {
        let cache = self.get_cache_payee(&command.payee_id).await?;
        if let Some(payee) = cache {
            return Ok(Some(payee));
        }

        let payee = self.payee_repo.get(command.payee_id, Some(command.auth_user.user_id)).await;
        self.handle_res_opt_payee(payee, &command.auth_user.user_id).await
    }

    async fn create(&self, command: PayeeCreateCommand) -> Result<PayeeResponse, AppError> {
        let meta_user = command.auth_user.user_id.clone();
        let payee_create = Payee::from(command);

        let payee = self.payee_repo.create(payee_create, Some(meta_user)).await
            .map_err(AppError::Internal)?;
        let response = PayeeResponse::from(payee);

        self.cache_payee(&response).await?;

        // Invalidate list cache
        if let Some(redis_pool) = &self.redis_pool {
            let _: () = delete_key(&redis_pool, self.form_redis_key_list_by_user(&meta_user).as_str()).await
                .map_err(AppError::Internal)?;
        }

        Ok(response)
    }

    async fn update(&self, command: PayeeUpdateCommand) -> Result<Option<PayeeResponse>, AppError> {
        let payee = self.payee_repo.update(
            command.payee_id, command.name, Some(command.auth_user.user_id)
        ).await;

        self.handle_res_opt_payee(payee, &command.auth_user.user_id).await
    }

    async fn delete(&self, command: PayeeDeleteCommand) -> Result<(), AppError> {
        self.payee_repo.delete(command.payee_id.clone(), Some(command.auth_user.user_id)).await
            .map_err(AppError::Internal)?;
        self.delete_cache(&command.payee_id, &command.auth_user.user_id).await?;
        Ok(())
    }

    async fn get_by_user(&self, command: PayeeListByUserCommand) -> Result<Vec<PayeeResponse>, AppError> {
        let (limit, offset, _search) = extract_pagination_data(command.pagination);

        if let Some(redis_pool) = &self.redis_pool {
            let cache: Option<Vec<PayeeResponse>> = get_key(
                &redis_pool, self.form_redis_key_list_by_user(&command.user_id).as_str()
            ).await.map_err(AppError::Internal)?;

            if let Some(payees) = cache { return Ok(payees); }
        }

        let payees = self.payee_repo.get_by_user(
            command.user_id, limit, offset
        ).await.map_err(AppError::Internal)?;

        let response: Vec<PayeeResponse> = payees.into_iter().map(PayeeResponse::from).collect();
        self.cache_payees_by_user(&command.user_id, &response).await?;

        Ok(response)
    }
}

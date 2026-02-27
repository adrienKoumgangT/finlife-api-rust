use anyhow::Result;
use async_trait::async_trait;
use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use uuid::Uuid;

use crate::modules::users::user::{
    user_command::*,
    user_dto::UserResponse,
    user_model::User,
    user_repo::{UserRepository, UserRepositoryInterface}
};
use crate::shared::{
    db::redis::{delete_key, get_key, set_key},
    errors::AppError,
    state::AppState,
};


#[async_trait]
pub trait UserServiceInterface {
    
    async fn get(&self, command: UserGetCommand) -> Result<Option<UserResponse>, AppError>;
    
    async fn get_by_email(&self, command: UserGetByEmailCommand) -> Result<Option<UserResponse>, AppError>;
    
    async fn create(&self, command: UserCreateCommand) -> Result<UserResponse, AppError>;

    async fn verify_email(&self, command: UserVerifyEmailCommand) -> Result<bool, AppError>;
    
    async fn update_password(&self, command: UserUpdatePasswordCommand) -> Result<bool, AppError>;
    
    async fn update_name(&self, command: UserUpdateNameCommand) -> Result<Option<UserResponse>, AppError>;
    
    async fn update_base_currency(&self, command: UserUpdateBaseCurrencyCommand) -> Result<Option<UserResponse>, AppError>;
    
    async fn delete(&self, command: UserDeleteCommand) -> Result<(), AppError>;
    
    async fn list(&self, command: UserListCommand) -> Result<Vec<UserResponse>, AppError>;
    
}

#[derive(Clone)]
pub struct UserService {
    user_repo: UserRepository,
    redis_pool: Option<Pool<RedisConnectionManager>>,
}

impl From<&AppState> for UserService {
    fn from(app_state: &AppState) -> Self {
        let user_repo = UserRepository::from(app_state);
        Self { user_repo, redis_pool: Option::from(app_state.redis_pool.clone()) }
    }
}

impl UserService {
    pub fn redis_key_single_ttl(&self) -> Option<u64> {
        Some(60*60)
    }

    pub fn form_redis_key_single(&self, key: &Uuid) -> String {
        format!("user:{}", key)
    }
}

#[async_trait]
impl UserServiceInterface for UserService {

    async fn get(&self, command: UserGetCommand) -> Result<Option<UserResponse>, AppError> {
        if let Some(redis_pool) = &self.redis_pool {
            let user_cache: Option<UserResponse> = get_key(
                &redis_pool,
                self.form_redis_key_single(&command.user_id).as_str()
            ).await.map_err(|e| AppError::InternalError(format!("Redis error: {}", e)))?;

            if let Some(user) = user_cache {
                return Ok(Some(user));
            }
        }

        let user = self.user_repo.get(command.user_id).await?;

        if let Some(user) = user {
            let user_response = UserResponse::from(user);
            if let Some(redis_pool) = &self.redis_pool {
                let _: () = set_key(
                    &redis_pool,
                    self.form_redis_key_single(&user_response.user_id).as_str(),
                    &user_response,
                    self.redis_key_single_ttl()
                ).await.map_err(AppError::Internal)?;
            }
            Ok(Some(user_response))
        } else {
            Ok(None)
        }
    }

    async fn get_by_email(&self, command: UserGetByEmailCommand) -> Result<Option<UserResponse>, AppError> {
        let user = self.user_repo.get_by_email(command.user_email).await?;

        if let Some(user) = user {
            Ok(Some(UserResponse::from(user)))
        } else {
            Ok(None)
        }
    }

    async fn create(&self, command: UserCreateCommand) -> Result<UserResponse, AppError> {
        let user_create = User::from(command);

        let user = self.user_repo.create(user_create).await?;

        let user_response = UserResponse::from(user);
        if let Some(redis_pool) = &self.redis_pool {
            let _: () = set_key(
                &redis_pool,
                self.form_redis_key_single(&user_response.user_id).as_str(),
                &user_response,
                self.redis_key_single_ttl()
            ).await.map_err(AppError::Internal)?;
        }

        Ok(user_response)
    }

    async fn verify_email(&self, command: UserVerifyEmailCommand) -> Result<bool, AppError> {
        let result = self.user_repo.verify_email(command.user_id).await?;

        if let Some(redis_pool) = &self.redis_pool {
            let _: () = delete_key(&redis_pool, self.form_redis_key_single(&command.user_id).as_str()).await
                .map_err(AppError::Internal)?;
        }

        Ok(result)
    }

    async fn update_password(&self, command: UserUpdatePasswordCommand) -> Result<bool, AppError> {
        let user = self.user_repo.update_password(command.user_id, command.new_password).await?;

        if let Some(user) = user {
            let user_response = UserResponse::from(user);
            if let Some(redis_pool) = &self.redis_pool {
                let _: () = set_key(
                    &redis_pool,
                    self.form_redis_key_single(&user_response.user_id).as_str(),
                    &user_response,
                    self.redis_key_single_ttl()
                ).await.map_err(AppError::Internal)?;
            }
            Ok(true)
        } else {
            Ok(false)
        }
    }

    async fn update_name(&self, command: UserUpdateNameCommand) -> Result<Option<UserResponse>, AppError> {
        let user = self.user_repo.update_name(command.user_id, command.first_name, command.last_name).await?;

        if let Some(user) = user {
            let user_response = UserResponse::from(user);
            if let Some(redis_pool) = &self.redis_pool {
                let _: () = set_key(
                    &redis_pool,
                    self.form_redis_key_single(&user_response.user_id).as_str(),
                    &user_response,
                    self.redis_key_single_ttl()
                ).await.map_err(AppError::Internal)?;
            }
            Ok(Some(user_response))
        } else {
            Ok(None)
        }
    }

    async fn update_base_currency(&self, command: UserUpdateBaseCurrencyCommand) -> Result<Option<UserResponse>, AppError> {
        let user = self.user_repo.update_base_currency(command.user_id, command.base_currency_code).await?;

        if let Some(user) = user {
            let user_response = UserResponse::from(user);
            if let Some(redis_pool) = &self.redis_pool {
                let _: () = set_key(
                    &redis_pool,
                    self.form_redis_key_single(&user_response.user_id).as_str(),
                    &user_response,
                    self.redis_key_single_ttl()
                ).await.map_err(AppError::Internal)?;
            }
            Ok(Some(user_response))
        } else {
            Ok(None)
        }
    }

    async fn delete(&self, command: UserDeleteCommand) -> Result<(), AppError> {
        self.user_repo.delete(command.user_id).await?;

        if let Some(redis_pool) = &self.redis_pool {
            let _: () = delete_key(&redis_pool, self.form_redis_key_single(&command.user_id).as_str()).await
                .map_err(AppError::Internal)?;
        }

        Ok(())
    }

    async fn list(&self, command: UserListCommand) -> Result<Vec<UserResponse>, AppError> {
        let mut limit: Option<u32> = None;
        let mut offset: Option<u32> = None;

        if let Some(pagination) = command.pagination {
            limit = pagination.page_size;

            if let (Some(page_size), Some(page)) = (pagination.page_size, pagination.page) {
                offset = Some(page * page_size);
            }
        }

        let users = self.user_repo.get_all(limit, offset).await?;
        Ok(users.into_iter().map(UserResponse::from).collect())
    }
}

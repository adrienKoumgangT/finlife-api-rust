use anyhow::{Error, Result};
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
use crate::shared::auth::password::verify_password;
use crate::shared::db::redis::{delete_key, get_key, set_key};
use crate::shared::state::AppState;


#[async_trait]
pub trait UserServiceInterface {

    async fn get(&self, command: UserGetCommand) -> Result<Option<UserResponse>, Error>;

    async fn create(&self, command: UserCreateCommand) -> Result<UserResponse, Error>;

    async fn update_password(&self, command: UserUpdatePasswordCommand) -> Result<Option<UserResponse>, Error>;

    async fn update_name(&self, command: UserUpdateNameCommand) -> Result<Option<UserResponse>, Error>;

    async fn update_base_currency(&self, command: UserUpdateBaseCurrencyCommand) -> Result<Option<UserResponse>, Error>;

    async fn delete(&self, command: UserDeleteCommand) -> Result<(), Error>;

    async fn list(&self, command: UserListCommand) -> Result<Vec<UserResponse>, Error>;

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
    async fn get(&self, command: UserGetCommand) -> Result<Option<UserResponse>, Error> {
        if let Some(redis_pool) = &self.redis_pool {
            let user_cache: Option<UserResponse> = get_key(
                &redis_pool,
                self.form_redis_key_single(&command.user_id).as_str()
            ).await?;
            if let Some(user) = user_cache {
                return Ok(Some(user));
            }
        }

        let user = self.user_repo.get(command.user_id, Some(command.auth_user.user_id)).await;
        match user {
            Ok(user) => {
                match user {
                    Some(user) => {
                        let user_response = UserResponse::from(user);
                        if let Some(redis_pool) = &self.redis_pool {
                            let _: () = set_key(
                                &redis_pool,
                                self.form_redis_key_single(&user_response.user_id).as_str(),
                                &user_response,
                                self.redis_key_single_ttl()
                            ).await?;
                        }
                        Ok(Some(user_response))
                    },
                    None => Ok(None)
                }
            },
            Err(_) => Err(Error::msg("Error during getting user"))
        }
    }

    async fn create(&self, command: UserCreateCommand) -> Result<UserResponse, Error> {
        let meta_user = command.auth_user.user_id.clone();
        let user_create = User::from(command);

        let user = self.user_repo.create(user_create, Some(meta_user)).await;
        match user {
            Ok(user) => {
                let user_response = UserResponse::from(user);
                if let Some(redis_pool) = &self.redis_pool {
                    let _: () = set_key(
                        &redis_pool,
                        self.form_redis_key_single(&user_response.user_id).as_str(),
                        &user_response,
                        self.redis_key_single_ttl()
                    ).await?;
                }
                Ok(user_response)
            },
            Err(_) => Err(Error::msg("Error during creating user"))
        }
    }

    async fn update_password(&self, command: UserUpdatePasswordCommand) -> Result<Option<UserResponse>, Error> {
        let old_user = self.user_repo.get(command.user_id, Some(command.auth_user.user_id)).await;
        match old_user {
            Ok(old_user) => {
                match old_user {
                    Some(old_user) => {
                        let valid_password = verify_password(command.user_old_password.as_str(), old_user.password_hash.as_str());
                        match valid_password {
                            Ok(valid_password) => {
                                if !valid_password {
                                    return Err(Error::msg("Password don't match"));
                                }

                                let user = self.user_repo.update_password(command.user_id, command.user_new_password, Some(command.auth_user.user_id)).await;
                                match user {
                                    Ok(user) => {
                                        match user {
                                            Some(user) => {
                                                let user_response = UserResponse::from(user);
                                                if let Some(redis_pool) = &self.redis_pool {
                                                    let _: () = set_key(
                                                        &redis_pool,
                                                        self.form_redis_key_single(&user_response.user_id).as_str(),
                                                        &user_response,
                                                        self.redis_key_single_ttl()
                                                    ).await?;
                                                }
                                                Ok(Some(user_response))
                                            },
                                            None => Ok(None)
                                        }
                                    },
                                    Err(_) => Err(Error::msg("Error during updating user"))
                                }
                            },
                            Err(_) => Err(Error::msg("Invalid password"))
                        }
                    },
                    None => Ok(None)
                }
            },
            Err(_) => Err(Error::msg("Error during updating user password"))
        }
    }

    async fn update_name(&self, command: UserUpdateNameCommand) -> Result<Option<UserResponse>, Error> {
        let user = self.user_repo.update_name(command.user_id, command.user_first_name, command.user_last_name, Some(command.auth_user.user_id)).await;
        match user {
            Ok(user) => {
                match user {
                    Some(user) => {
                        let user_response = UserResponse::from(user);
                        if let Some(redis_pool) = &self.redis_pool {
                            let _: () = set_key(
                                &redis_pool,
                                self.form_redis_key_single(&user_response.user_id).as_str(),
                                &user_response,
                                self.redis_key_single_ttl()
                            ).await?;
                        }
                        Ok(Some(user_response))
                    },
                    None => Ok(None)
                }
            },
            Err(_) => Err(Error::msg("Error during updating user name"))
        }
    }

    async fn update_base_currency(&self, command: UserUpdateBaseCurrencyCommand) -> Result<Option<UserResponse>, Error> {
        let user = self.user_repo.update_base_currency(command.user_id, command.user_base_currency_code, Some(command.auth_user.user_id)).await;
        match user {
            Ok(user) => {
                match user {
                    Some(user) => {
                        let user_response = UserResponse::from(user);
                        if let Some(redis_pool) = &self.redis_pool {
                            let _: () = set_key(
                                &redis_pool,
                                self.form_redis_key_single(&user_response.user_id).as_str(),
                                &user_response,
                                self.redis_key_single_ttl()
                            ).await?;
                        }
                        Ok(Some(user_response))
                    },
                    None => Ok(None)
                }
            },
            Err(_) => Err(Error::msg("Error during updating user base currency"))
        }
    }

    async fn delete(&self, command: UserDeleteCommand) -> Result<(), Error> {
        let result = self.user_repo.delete(command.user_id, Some(command.auth_user.user_id)).await;
        if let Some(redis_pool) = &self.redis_pool {
            let _: () = delete_key(&redis_pool, self.form_redis_key_single(&command.user_id).as_str()).await?;
        }
        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(Error::msg("Error during deleting user"))
        }
    }

    async fn list(&self, command: UserListCommand) -> Result<Vec<UserResponse>, Error> {
        let mut limit: Option<u32> = None;
        let mut offset: Option<u32> = None;

        if let Some(pagination) = command.pagination {
            limit = pagination.page_size;

            if let (Some(page_size), Some(page)) = (pagination.page_size, pagination.page) {
                offset = Some(page * page_size);
            }
        }

        let users = self.user_repo.get_all(limit, offset, Some(command.auth_user.user_id)).await;
        match users {
            Ok(users) => Ok(users.into_iter().map(UserResponse::from).collect()),
            Err(_) => Err(Error::msg("Error getting users"))
        }
    }
}

use anyhow::{Error, Result};
use async_trait::async_trait;
use uuid::Uuid;
use sqlx::MySqlPool;

use crate::modules::users::user::user_model::User;
use crate::shared::db::mysql::{GenericRepository, MySqlParam};
use crate::shared::crud_repository::CrudRepository;
use crate::shared::state::AppState;
use crate::shared::utils::{oub, ub};

#[async_trait]
pub trait UserRepositoryInterface {
    async fn get(&self, user_id: Uuid, meta_user: Option<Uuid>) -> Result<Option<User>, Error>;

    async fn create(&self, user: User, meta_user: Option<Uuid>) -> Result<User, Error>;

    async fn update_password(&self, user_id: Uuid, password: String, meta_user: Option<Uuid>) -> Result<Option<User>, Error>;

    async fn update_name(&self, user_id: Uuid, first_name: String, last_name: String, meta_user: Option<Uuid>) -> Result<Option<User>, Error>;

    async fn update_base_currency(&self, user_id: Uuid, base_currency: String, meta_user: Option<Uuid>) -> Result<Option<User>, Error>;

    async fn delete(&self, user_id: Uuid, meta_user: Option<Uuid>) -> Result<(), Error>;

    async fn get_by_email(&self, email: String, meta_user: Option<Uuid>) -> Result<Option<User>, Error>;

    async fn get_all(&self, limit: Option<u32>, offset: Option<u32>, meta_user: Option<Uuid>) -> Result<Vec<User>, Error>;
}

#[derive(Clone)]
pub struct UserRepository {
    pool: MySqlPool,
}

impl From<&AppState> for UserRepository {
    fn from(app_state: &AppState) -> Self {
        Self { pool: app_state.mysql_pool.clone() }
    }
}

impl GenericRepository<User> for UserRepository {
    fn get_pool(&self) -> &MySqlPool {
        &self.pool
    }
}


#[async_trait]
impl UserRepositoryInterface for UserRepository {
    async fn get(&self, user_id: Uuid, meta_user: Option<Uuid>) -> Result<Option<User>, Error> {
        let params = vec![
            MySqlParam::from(ub(user_id)),
            MySqlParam::from(oub(meta_user)),
        ];

        self.call_procedure_for_optional("proc_user_get_by_id", params).await
    }

    async fn create(&self, user: User, meta_user: Option<Uuid>) -> Result<User, Error> {
        let params = vec![
            MySqlParam::from(user.email),
            MySqlParam::from(user.password_hash),
            MySqlParam::from(user.first_name),
            MySqlParam::from(user.last_name),
            MySqlParam::from(user.base_currency_code),
            MySqlParam::from(oub(meta_user)),
        ];

        self.call_procedure_for_one("proc_user_insert", params).await
    }

    async fn update_password(&self, user_id: Uuid, password: String, meta_user: Option<Uuid>) -> Result<Option<User>, Error> {
        let params = vec![
            MySqlParam::from(ub(user_id)),
            MySqlParam::from(password),
            MySqlParam::from(oub(meta_user)),
        ];
        
        self.call_procedure_for_optional("proc_user_update_password", params).await
    }

    async fn update_name(&self, user_id: Uuid, first_name: String, last_name: String, meta_user: Option<Uuid>) -> Result<Option<User>, Error> {
        let params = vec![
            MySqlParam::from(ub(user_id)),
            MySqlParam::from(first_name),
            MySqlParam::from(last_name),
            MySqlParam::from(oub(meta_user)),
        ];
        
        self.call_procedure_for_optional("proc_user_update_name", params).await
    }

    async fn update_base_currency(&self, user_id: Uuid, base_currency: String, meta_user: Option<Uuid>) -> Result<Option<User>, Error> {
        let params = vec![
            MySqlParam::from(ub(user_id)),
            MySqlParam::from(base_currency),
            MySqlParam::from(oub(meta_user)),
        ];
        
        self.call_procedure_for_optional("proc_user_update_base_currency", params).await
    }

    async fn delete(&self, user_id: Uuid, meta_user: Option<Uuid>) -> Result<(), Error> {
        let params = vec![
            MySqlParam::from(ub(user_id)),
            MySqlParam::from(oub(meta_user)),
        ];
        
       self.call_procedure("proc_user_delete", params.clone()).await
    }

    async fn get_by_email(&self, email: String, meta_user: Option<Uuid>) -> Result<Option<User>, Error> {
        let params = vec![
            MySqlParam::from(email),
            MySqlParam::from(oub(meta_user)),
        ];
        
        self.call_procedure_for_optional("proc_user_get_by_email", params).await
    }

    async fn get_all(&self, limit: Option<u32>, offset: Option<u32>, meta_user: Option<Uuid>) -> Result<Vec<User>, Error> {
        let params = vec![
            MySqlParam::from(limit),
            MySqlParam::from(offset),
            MySqlParam::from(oub(meta_user)),
        ];
        
        self.call_procedure_for_list("proc_user_list", params.clone()).await
    }
}

use anyhow::{Error, Result};
use async_trait::async_trait;
use rust_decimal::Decimal;
use uuid::Uuid;
use sqlx::MySqlPool;

use crate::modules::people::people_model::People;
use crate::shared::db::mysql::{GenericRepository, MySqlParam};
use crate::shared::crud_repository::CrudRepository;
use crate::shared::state::AppState;
use crate::shared::utils::{oub, ub};


#[async_trait]
pub trait PeopleRepositoryInterface {
    
    async fn get(&self, people_id: Uuid, meta_user: Option<Uuid>) -> Result<Option<People>, Error>;
    
    async fn create(&self, people: People, meta_user: Option<Uuid>) -> Result<People, Error>;
    
    async fn update_image(&self, people_id: Uuid, image_url: Option<String>, meta_user: Option<Uuid>) -> Result<Option<People>, Error>;
    
    async fn update(&self, people_id: Uuid, name: String, email: Option<String>, phone: Option<String>, note: Option<String>, meta_user: Option<Uuid>) -> Result<Option<People>, Error>;
    
    async fn archived(&self, people_id: Uuid, archived: bool, meta_user: Option<Uuid>) -> Result<Option<People>, Error>;
    
    async fn delete(&self, people_id: Uuid, meta_user: Option<Uuid>) -> Result<(), Error>;
    
    async fn get_all(&self, limit: Option<u32>, offset: Option<u32>, meta_user: Option<Uuid>) -> Result<Vec<People>, Error>;
    
    async fn get_by_user(&self, user_id: Uuid, meta_user: Option<Uuid>) -> Result<Vec<People>, Error>;
    
    async fn search(&self, query: String,  limit: Option<u32>, offset: Option<u32>, meta_user: Option<Uuid>) -> Result<Vec<People>, Error>;
    
    async fn search_by_user(&self, user_id: Uuid, query: String, meta_user: Option<Uuid>) -> Result<Vec<People>, Error>;
    
}


#[derive(Clone)]
pub struct PeopleRepository {
    pool: MySqlPool,
}

impl From<&AppState> for PeopleRepository {
    fn from(app_state: &AppState) -> Self {
        Self { pool: app_state.mysql_pool.clone() }
    }
}

impl GenericRepository<People> for PeopleRepository {
    fn get_pool(&self) -> &MySqlPool {
        &self.pool
    }
}

#[async_trait]
impl PeopleRepositoryInterface for PeopleRepository {
    async fn get(&self, people_id: Uuid, meta_user: Option<Uuid>) -> Result<Option<People>, Error> {
        let params = vec![
            MySqlParam::from(ub(people_id)),
            MySqlParam::from(oub(meta_user)),
        ];

        self.call_procedure_for_optional("proc_people_get_by_id", params).await
    }

    async fn create(&self, people: People, meta_user: Option<Uuid>) -> Result<People, Error> {
        let params = vec![
            MySqlParam::from(people.user_id),
            MySqlParam::from(people.name),
            MySqlParam::from(people.email),
            MySqlParam::from(people.phone),
            MySqlParam::from(people.image_url),
            MySqlParam::from(people.note),
            MySqlParam::from(people.archived),
            MySqlParam::from(oub(meta_user)),
        ];

        self.call_procedure_for_one("proc_people_create", params).await
    }

    async fn update_image(&self, people_id: Uuid, image_url: Option<String>, meta_user: Option<Uuid>) -> Result<Option<People>, Error> {
        let params = vec![
            MySqlParam::from(ub(people_id)),
            MySqlParam::from(image_url),
            MySqlParam::from(oub(meta_user)),
        ];

        self.call_procedure_for_optional("proc_people_update_image", params).await
    }

    async fn update(&self, people_id: Uuid, name: String, email: Option<String>, phone: Option<String>, note: Option<String>, meta_user: Option<Uuid>) -> Result<Option<People>, Error> {
        let params = vec![
            MySqlParam::from(ub(people_id)),
            MySqlParam::from(name),
            MySqlParam::from(email),
            MySqlParam::from(phone),
            MySqlParam::from(note),
            MySqlParam::from(oub(meta_user)),
        ];

        self.call_procedure_for_optional("proc_people_update", params).await
    }

    async fn archived(&self, people_id: Uuid, archived: bool, meta_user: Option<Uuid>) -> Result<Option<People>, Error> {
        let params = vec![
            MySqlParam::from(ub(people_id)),
            MySqlParam::from(archived),
            MySqlParam::from(oub(meta_user)),
        ];

        self.call_procedure_for_optional("proc_people_update_archived", params).await
    }

    async fn delete(&self, people_id: Uuid, meta_user: Option<Uuid>) -> Result<(), Error> {
        let params = vec![
            MySqlParam::from(ub(people_id)),
            MySqlParam::from(oub(meta_user)),
        ];

        self.call_procedure("proc_people_delete", params).await
    }

    async fn get_all(&self, limit: Option<u32>, offset: Option<u32>, meta_user: Option<Uuid>) -> Result<Vec<People>, Error> {
        let params = vec![
            MySqlParam::from(limit),
            MySqlParam::from(offset),
            MySqlParam::from(oub(meta_user)),
        ];

        self.call_procedure_for_list("proc_people_list", params).await
    }

    async fn get_by_user(&self, user_id: Uuid, meta_user: Option<Uuid>) -> Result<Vec<People>, Error> {
        let params = vec![
            MySqlParam::from(ub(user_id)),
            MySqlParam::from(oub(meta_user)),
        ];

        self.call_procedure_for_list("proc_people_by_user", params).await
    }

    async fn search(&self, query: String,  limit: Option<u32>, offset: Option<u32>, meta_user: Option<Uuid>) -> Result<Vec<People>, Error> {
        let params = vec![
            MySqlParam::from(query),
            MySqlParam::from(limit),
            MySqlParam::from(offset),
            MySqlParam::from(oub(meta_user)),
        ];

        self.call_procedure_for_list("proc_people_search", params).await
    }

    async fn search_by_user(&self, user_id: Uuid, query: String, meta_user: Option<Uuid>) -> Result<Vec<People>, Error> {
        let params = vec![
            MySqlParam::from(ub(user_id)),
            MySqlParam::from(query),
            MySqlParam::from(oub(meta_user)),
        ];

        self.call_procedure_for_list("proc_people_search_by_user", params).await
    }
}

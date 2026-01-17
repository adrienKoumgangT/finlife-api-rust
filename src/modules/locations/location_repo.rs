use anyhow::{Error, Result};
use async_trait::async_trait;
use rust_decimal::Decimal;
use uuid::Uuid;
use sqlx::MySqlPool;

use crate::modules::locations::location_model::Location;
use crate::shared::db::mysql::{GenericRepository, MySqlParam};
use crate::shared::crud_repository::CrudRepository;
use crate::shared::state::AppState;
use crate::shared::utils::{oub, ub};


#[async_trait]
pub trait LocationRepositoryInterface {

    async fn get(&self, location_id: Uuid, meta_user: Option<Uuid>) -> Result<Option<Location>, Error>;

    async fn create(&self, location: Location, meta_user: Option<Uuid>) -> Result<Location, Error>;

    async fn update_name(&self, location_id: Uuid, name: String, meta_user: Option<Uuid>) -> Result<Option<Location>, Error>;

    async fn update(&self, location_id: Uuid, address: Option<String>, city: Option<String>, region: Option<String>, postal_code: Option<String>, country_code: Option<String>, meta_user: Option<Uuid>) -> Result<Option<Location>, Error>;

    async fn update_lat_long(&self, location_id: Uuid, latitude: Option<Decimal>, longitude: Option<Decimal>, meta_user: Option<Uuid>) -> Result<Option<Location>, Error>;

    async fn archived(&self, location_id: Uuid, archived: bool, meta_user: Option<Uuid>) -> Result<Option<Location>, Error>;

    async fn delete(&self, location_id: Uuid, meta_user: Option<Uuid>) -> Result<(), Error>;

    async fn get_all(&self, limit: Option<u32>, offset: Option<u32>, meta_user: Option<Uuid>) -> Result<Vec<Location>, Error>;

    async fn get_by_user(&self, user_id: Uuid, meta_user: Option<Uuid>) -> Result<Vec<Location>, Error>;

    async fn search(&self, query: String, limit: Option<u32>, offset: Option<u32>, meta_user: Option<Uuid>) -> Result<Vec<Location>, Error>;

    async fn search_by_user(&self, user_id: Uuid, query: String, meta_user: Option<Uuid>) -> Result<Vec<Location>, Error>;

}

#[derive(Clone)]
pub struct LocationRepository {
    pool: MySqlPool,
}

impl From<&AppState> for LocationRepository {
    fn from(app_state: &AppState) -> Self {
        Self { pool: app_state.mysql_pool.clone() }
    }
}

impl GenericRepository<Location> for LocationRepository {
    fn get_pool(&self) -> &MySqlPool {
        &self.pool
    }
}

#[async_trait]
impl LocationRepositoryInterface for LocationRepository {
    async fn get(&self, location_id: Uuid, meta_user: Option<Uuid>) -> Result<Option<Location>, Error> {
        let params = vec![
            MySqlParam::from(ub(location_id)),
            MySqlParam::from(oub(meta_user)),
        ];

        self.call_procedure_for_optional("proc_location_get_by_id", params).await
    }

    async fn create(&self, location: Location, meta_user: Option<Uuid>) -> Result<Location, Error> {
        let params = vec![
            MySqlParam::from(location.user_id),
            MySqlParam::from(location.name),
            MySqlParam::from(location.address),
            MySqlParam::from(location.city),
            MySqlParam::from(location.region),
            MySqlParam::from(location.postal_code),
            MySqlParam::from(location.country_code),
            MySqlParam::from(location.latitude),
            MySqlParam::from(location.longitude),
            MySqlParam::from(location.archived),
            MySqlParam::from(oub(meta_user)),
        ];

        self.call_procedure_for_one("proc_location_create", params).await
    }

    async fn update_name(&self, location_id: Uuid, name: String, meta_user: Option<Uuid>) -> Result<Option<Location>, Error> {
        let params = vec![
            MySqlParam::from(ub(location_id)),
            MySqlParam::from(name),
            MySqlParam::from(oub(meta_user)),
        ];

        self.call_procedure_for_optional("proc_location_update_name", params).await
    }

    async fn update(&self, location_id: Uuid, address: Option<String>, city: Option<String>, region: Option<String>, postal_code: Option<String>, country_code: Option<String>, meta_user: Option<Uuid>) -> Result<Option<Location>, Error> {
        let params = vec![
            MySqlParam::from(ub(location_id)),
            MySqlParam::from(address),
            MySqlParam::from(city),
            MySqlParam::from(region),
            MySqlParam::from(postal_code),
            MySqlParam::from(country_code),
            MySqlParam::from(oub(meta_user)),
        ];

        self.call_procedure_for_optional("proc_location_update", params).await
    }

    async fn update_lat_long(&self, location_id: Uuid, latitude: Option<Decimal>, longitude: Option<Decimal>, meta_user: Option<Uuid>) -> Result<Option<Location>, Error> {
        let params = vec![
            MySqlParam::from(ub(location_id)),
            MySqlParam::from(latitude),
            MySqlParam::from(longitude),
            MySqlParam::from(oub(meta_user)),
        ];

        self.call_procedure_for_optional("proc_location_update_lat_long", params).await
    }

    async fn archived(&self, location_id: Uuid, archived: bool, meta_user: Option<Uuid>) -> Result<Option<Location>, Error> {
        let params = vec![
            MySqlParam::from(ub(location_id)),
            MySqlParam::from(archived),
            MySqlParam::from(oub(meta_user)),
        ];

        self.call_procedure_for_optional("proc_location_update_archived", params).await
    }

    async fn delete(&self, location_id: Uuid, meta_user: Option<Uuid>) -> Result<(), Error> {
        let params = vec![
            MySqlParam::from(ub(location_id)),
            MySqlParam::from(oub(meta_user)),
        ];

        self.call_procedure("proc_location_delete", params).await
    }

    async fn get_all(&self, limit: Option<u32>, offset: Option<u32>, meta_user: Option<Uuid>) -> Result<Vec<Location>, Error> {
        let params = vec![
            MySqlParam::from(limit),
            MySqlParam::from(offset),
            MySqlParam::from(oub(meta_user)),
        ];

        self.call_procedure_for_list("proc_location_list", params).await
    }

    async fn get_by_user(&self, user_id: Uuid, meta_user: Option<Uuid>) -> Result<Vec<Location>, Error> {
        let params = vec![
            MySqlParam::from(ub(user_id)),
            MySqlParam::from(oub(meta_user)),
        ];

        self.call_procedure_for_list("proc_location_by_user", params).await
    }

    async fn search(&self, query: String, limit: Option<u32>, offset: Option<u32>, meta_user: Option<Uuid>) -> Result<Vec<Location>, Error> {
        let params = vec![
            MySqlParam::from(query),
            MySqlParam::from(limit),
            MySqlParam::from(offset),
            MySqlParam::from(oub(meta_user)),
        ];

        self.call_procedure_for_list("proc_location_search", params).await
    }

    async fn search_by_user(&self, user_id: Uuid, query: String, meta_user: Option<Uuid>) -> Result<Vec<Location>, Error> {
        let params = vec![
            MySqlParam::from(ub(user_id)),
            MySqlParam::from(query),
            MySqlParam::from(oub(meta_user)),
        ];

        self.call_procedure_for_list("proc_location_search_by_user", params).await
    }
}

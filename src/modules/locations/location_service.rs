use anyhow::Result;
use async_trait::async_trait;
use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use uuid::Uuid;

use crate::modules::locations::{
    location_command::*,
    location_dto::*,
    location_model::Location,
    location_repo::{LocationRepository, LocationRepositoryInterface}
};
use crate::shared::{
    db::redis::{delete_key, get_key, set_key},
    errors::AppError,
    state::AppState,
    utils::extract_pagination_data
};

#[async_trait]
pub trait LocationInterface {

    async fn get(&self, command: LocationGetCommand) -> Result<Option<LocationResponse>, AppError>;

    async fn create(&self, command: LocationCreateCommand) -> Result<LocationResponse, AppError>;

    async fn update(&self, command: LocationUpdateCommand) -> Result<Option<LocationResponse>, AppError>;

    async fn update_lat_long(&self, command: LocationUpdateLatLongCommand) -> Result<Option<LocationResponse>, AppError>;

    async fn archived(&self, command: LocationArchivedCommand) -> Result<Option<LocationResponse>, AppError>;

    async fn delete(&self, command: LocationDeleteCommand) -> Result<(), AppError>;

    async fn get_by_user(&self, command: LocationListByUserCommand) -> Result<Vec<LocationResponse>, AppError>;

}

#[derive(Clone)]
pub struct LocationService {
    location_repo: LocationRepository,
    redis_pool: Option<Pool<RedisConnectionManager>>,
}

impl From<&AppState> for LocationService {
    fn from(app_state: &AppState) -> Self {
        Self {
            location_repo: LocationRepository::from(app_state),
            redis_pool: app_state.redis_pool.clone()
        }
    }
}

impl LocationService {
    fn redis_key_ttl(&self) -> Option<u64> { Some(60 * 60) }

    fn form_redis_key_location(&self, key: &Uuid) -> String { format!("location:{}", key) }

    fn form_redis_key_list_by_user(&self, user: &Uuid) -> String { format!("user:{}:locations", user) }

    async fn cache_location(&self, location: &LocationResponse) -> Result<(), AppError> {
        if let Some(redis_pool) = &self.redis_pool {
            let _: () = set_key(
                &redis_pool,
                self.form_redis_key_location(&location.location_id).as_str(),
                &location,
                self.redis_key_ttl()
            ).await.map_err(AppError::Internal)?;
        }
        Ok(())
    }

    async fn cache_locations_by_user(&self, user: &Uuid, locations: &Vec<LocationResponse>) -> Result<(), AppError> {
        if let Some(redis_pool) = &self.redis_pool {
            let _: () = set_key(
                &redis_pool,
                self.form_redis_key_list_by_user(user).as_str(),
                &locations,
                self.redis_key_ttl()
            ).await.map_err(AppError::Internal)?;
        }
        Ok(())
    }

    async fn get_cache_location(&self, key: &Uuid) -> Result<Option<LocationResponse>, AppError> {
        if let Some(redis_pool) = &self.redis_pool {
            let location_cache: Option<LocationResponse> = get_key(
                &redis_pool,
                self.form_redis_key_location(key).as_str()
            ).await
                .map_err(AppError::Internal)?;
            return Ok(location_cache);
        }
        Ok(None)
    }

    async fn delete_cache(&self, key: &Uuid, user: &Uuid) -> Result<(), AppError> {
        if let Some(redis_pool) = &self.redis_pool {
            let _: () = delete_key(&redis_pool, self.form_redis_key_location(key).as_str()).await
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

    async fn handle_res_opt_location(&self, location: Result<Option<Location>>, delete_cache_list: bool, auth_user: &Uuid) -> Result<Option<LocationResponse>, AppError> {
        let location = location.map_err(AppError::Internal)?;

        if let Some(loc) = location {
            let response = LocationResponse::from(loc);
            self.cache_location(&response).await?;

            if delete_cache_list { self.delete_cache_list(auth_user).await?; }

            Ok(Some(response))
        } else {
            Ok(None)
        }
    }
}

#[async_trait]
impl LocationInterface for LocationService {
    async fn get(&self, command: LocationGetCommand) -> Result<Option<LocationResponse>, AppError> {
        let cache = self.get_cache_location(&command.location_id).await?;
        if let Some(location) = cache {
            return Ok(Some(location));
        }

        let location = self.location_repo.get(command.location_id, Some(command.auth_user.user_id)).await;
        self.handle_res_opt_location(location, false, &command.auth_user.user_id).await
    }

    async fn create(&self, command: LocationCreateCommand) -> Result<LocationResponse, AppError> {
        let meta_user = command.auth_user.user_id.clone();
        let location_create = Location::from(command);

        let location = self.location_repo.create(location_create, Some(meta_user.clone())).await
            .map_err(AppError::Internal)?;
        let response = LocationResponse::from(location);

        self.delete_cache(&response.location_id, &meta_user).await?;
        self.cache_location(&response).await?;

        if let Some(redis_pool) = &self.redis_pool {
            let _: () = delete_key(&redis_pool, self.form_redis_key_list_by_user(&meta_user).as_str()).await
                .map_err(AppError::Internal)?;
        }

        Ok(response)
    }

    async fn update(&self, command: LocationUpdateCommand) -> Result<Option<LocationResponse>, AppError> {
        let location_id = command.location_id.clone();
        let meta_user = command.auth_user.user_id.clone();

        let location = self.location_repo.update(
            command.location_id, command.name, command.address,
            command.city, command.district, command.region, command.postal_code,
            command.country_code, Some(command.auth_user.user_id)
        ).await;

        self.delete_cache(&location_id, &meta_user).await?;

        self.handle_res_opt_location(location, true, &command.auth_user.user_id).await
    }

    async fn update_lat_long(&self, command: LocationUpdateLatLongCommand) -> Result<Option<LocationResponse>, AppError> {
        let location_id = command.location_id.clone();
        let meta_user = command.auth_user.user_id.clone();

        let location = self.location_repo.update_lat_long(
            command.location_id, command.latitude, command.longitude, Some(command.auth_user.user_id)
        ).await;

        self.delete_cache(&location_id, &meta_user).await?;

        self.handle_res_opt_location(location, true, &command.auth_user.user_id).await
    }

    async fn archived(&self, command: LocationArchivedCommand) -> Result<Option<LocationResponse>, AppError> {
        let location_id = command.location_id.clone();
        let meta_user = command.auth_user.user_id.clone();

        let location = self.location_repo.archived(
            command.location_id, command.archived, Some(command.auth_user.user_id)
        ).await;

        self.delete_cache(&location_id, &meta_user).await?;

        self.handle_res_opt_location(location, true, &command.auth_user.user_id).await
    }

    async fn delete(&self, command: LocationDeleteCommand) -> Result<(), AppError> {
        self.location_repo.delete(command.location_id.clone(), Some(command.auth_user.user_id)).await.map_err(AppError::Internal)?;
        self.delete_cache(&command.location_id, &command.auth_user.user_id).await?;
        Ok(())
    }

    async fn get_by_user(&self, command: LocationListByUserCommand) -> Result<Vec<LocationResponse>, AppError> {
        let (limit, offset, search) = extract_pagination_data(command.pagination);

        if search.is_some() {
            let locations = self.location_repo.search_by_user(
                command.user_id, search.unwrap(), limit, offset
            ).await.map_err(AppError::Internal)?;

            Ok(locations.into_iter().map(LocationResponse::from).collect())
        } else {
            if let Some(redis_pool) = &self.redis_pool {
                let cache: Option<Vec<LocationResponse>> = get_key(
                    &redis_pool, self.form_redis_key_list_by_user(&command.user_id).as_str()
                ).await.map_err(AppError::Internal)?;

                if let Some(locations) = cache { return Ok(locations); }
            }

            let locations = self.location_repo.get_by_user(
                command.user_id, limit, offset
            ).await.map_err(AppError::Internal)?;

            let response: Vec<LocationResponse> = locations.into_iter().map(LocationResponse::from).collect();
            self.cache_locations_by_user(&command.user_id, &response).await?;
            Ok(response)
        }
    }
}

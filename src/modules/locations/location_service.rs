use anyhow::{Error, Result};
use async_trait::async_trait;
use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use time::macros::offset;
use uuid::Uuid;

use crate::modules::locations::{
    location_command::*,
    location_dto::*,
    location_model::Location,
    location_repo::{LocationRepository, LocationRepositoryInterface}
};
use crate::shared::db::redis::{delete_key, get_key, set_key};
use crate::shared::state::AppState;
use crate::shared::utils::extract_pagination_data;

#[async_trait]
pub trait LocationServiceInterface {
    
    async fn get(&self, command: LocationGetCommand) -> Result<Option<LocationResponse>, Error>;
    
    async fn create(&self, command: LocationCreateCommand) -> Result<LocationResponse, Error>;
    
    async fn update_name(&self, command: LocationUpdateNameCommand) -> Result<Option<LocationResponse>, Error>;
    
    async fn update(&self, command: LocationUpdateCommand) -> Result<Option<LocationResponse>, Error>;
    
    async fn update_lat_long(&self, command: LocationUpdateLatLongCommand) -> Result<Option<LocationResponse>, Error>;
    
    async fn archived(&self, command: LocationArchivedCommand) -> Result<Option<LocationResponse>, Error>;
    
    async fn delete(&self, command: LocationDeleteCommand) -> Result<(), Error>;
    
    async fn get_all(&self, command: LocationListCommand) -> Result<Vec<LocationResponse>, Error>;
    
    async fn get_by_user(&self, command: LocationListByUserCommand) -> Result<Vec<LocationResponse>, Error>;
    
}

#[derive(Clone)]
pub struct LocationService {
    location_repo: LocationRepository,
    redis_pool: Option<Pool<RedisConnectionManager>>,
}

impl From<&AppState> for LocationService {
    fn from(app_state: &AppState) -> Self {
        let location_repo = LocationRepository::from(app_state);
        let redis_pool = app_state.redis_pool.clone();
        Self { location_repo, redis_pool: Option::from(redis_pool) }
    }
}

impl LocationService {

    fn redis_key_ttl(&self) -> Option<u64> {
        Some(60*60)
    }
    
    fn form_redis_key_location(&self, key: &Uuid) -> String {
        format!("location:{}", key)
    }

    fn form_redis_key_list_by_user(&self, user: &Uuid) -> String {
        format!("user:{}:location", user)
    }

    async fn cache_location(&self, location: &LocationResponse) -> Result<(), Error> {
        if let Some(redis_pool) = &self.redis_pool {
            let _: () = set_key(
                &redis_pool,
                self.form_redis_key_location(&location.location_id).as_str(),
                &location,
                self.redis_key_ttl()
            ).await?;
        }
        Ok(())
    }


    async fn cache_location_by_user(&self, user: &Uuid, locations: &Vec<LocationResponse>) -> Result<(), Error> {
        if let Some(redis_pool) = &self.redis_pool {
            let _: () = set_key(
                &redis_pool,
                self.form_redis_key_list_by_user(user).as_str(),
                &locations,
                self.redis_key_ttl()
            ).await?;
        }
        Ok(())
    }
    
    async fn get_cache_location(&self, key: &Uuid) -> Result<Option<LocationResponse>, Error> {
        if let Some(redis_pool) = &self.redis_pool {
            let location_cache: Option<LocationResponse> = get_key(
                &redis_pool,
                self.form_redis_key_location(key).as_str()
            ).await?;
            if let Some(location) = location_cache {
                return Ok(Some(location))
            }
        }
        Ok(None)
    }
    
    async fn get_cache_location_by_user(&self, user: &Uuid) -> Result<Option<Vec<LocationResponse>>, Error> {
        if let Some(redis_pool) = &self.redis_pool {
            let locations_cache: Option<Vec<LocationResponse>> = get_key(
                &redis_pool,
                self.form_redis_key_list_by_user(user).as_str()
            ).await?;
            if let Some(locations) = locations_cache {
                return Ok(Some(locations))
            }
        }
        Ok(None)
    }
    
    async fn delete_cache(&self, key: &Uuid, user: &Uuid) -> Result<(), Error> {
        if let Some(redis_pool) = &self.redis_pool {
            let _: () = delete_key(&redis_pool, self.form_redis_key_location(key).as_str()).await?;
            let _: () = delete_key(&redis_pool, self.form_redis_key_list_by_user(user).as_str()).await?;
        }
        Ok(())
    }
    
    async fn handle_res_opt_location(&self, location: Result<Option<Location>, Error>) -> Result<Option<LocationResponse>, Error> {
        match location {
            Ok(location) => {
                match location {
                    Some(location) => {
                        let location_response = LocationResponse::from(location);
                        self.cache_location(&location_response).await?;
                        Ok(Some(location_response))
                    },
                    None => Ok(None)
                }
            }
            Err(_) => Err(Error::msg("Error from repository"))
        }
    }
}

#[async_trait]
impl LocationServiceInterface for LocationService {
    async fn get(&self, command: LocationGetCommand) -> Result<Option<LocationResponse>, Error> {
        let location_cache = self.get_cache_location(&command.location_id).await?;
        if let Some(location) = location_cache {
            return Ok(Some(location))
        }
        
        let location = self.location_repo.get(command.location_id, Some(command.auth_user.user_id)).await;
        self.handle_res_opt_location(location).await
    }

    async fn create(&self, command: LocationCreateCommand) -> Result<LocationResponse, Error> {
        let meta_user = command.auth_user.user_id.clone();
        let location_create = Location::from(command);
        
        let location = self.location_repo.create(location_create, Some(meta_user)).await;
        match location {
            Ok(location) => {
                let location_response = LocationResponse::from(location);
                self.cache_location(&location_response).await?;
                Ok(location_response)
            }
            Err(_) => Err(Error::msg("Error creating location"))
        }
    }

    async fn update_name(&self, command: LocationUpdateNameCommand) -> Result<Option<LocationResponse>, Error> {
        let location = self.location_repo.update_name(command.location_id, command.location_name, Some(command.auth_user.user_id)).await;
        self.handle_res_opt_location(location).await
    }

    async fn update(&self, command: LocationUpdateCommand) -> Result<Option<LocationResponse>, Error> {
        let location = self.location_repo.update(command.location_id, command.location_address, command.location_city, command.location_region, command.location_postal_code, command.location_country_code, Some(command.auth_user.user_id)).await;
        self.handle_res_opt_location(location).await
    }

    async fn update_lat_long(&self, command: LocationUpdateLatLongCommand) -> Result<Option<LocationResponse>, Error> {
        let location = self.location_repo.update_lat_long(command.location_id, command.location_latitude, command.location_longitude, Some(command.auth_user.user_id)).await;
        self.handle_res_opt_location(location).await
    }

    async fn archived(&self, command: LocationArchivedCommand) -> Result<Option<LocationResponse>, Error> {
        let location = self.location_repo.archived(command.location_id, command.location_archived, Some(command.auth_user.user_id)).await;
        self.handle_res_opt_location(location).await
    }

    async fn delete(&self, command: LocationDeleteCommand) -> Result<(), Error> {
        let result = self.location_repo.delete(command.location_id.clone(), Some(command.auth_user.user_id)).await;
        self.delete_cache(&command.location_id, &command.auth_user.user_id).await?;
        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(Error::msg("Error deleting location"))
        }
    }

    async fn get_all(&self, command: LocationListCommand) -> Result<Vec<LocationResponse>, Error> {
        let meta_user = command.auth_user.user_id.clone();
        
        let (limit, offset, search) = extract_pagination_data(command.pagination);

        let locations = if search.is_some() {
            self.location_repo.search(search.unwrap(), limit, offset, Some(meta_user)).await
        } else {
            self.location_repo.get_all(limit, offset, Some(meta_user)).await
        };
        match locations {
            Ok(locations) => Ok(locations.into_iter().map(LocationResponse::from).collect()),
            Err(_) => Err(Error::msg("Error getting locations"))
        }
    }

    async fn get_by_user(&self, command: LocationListByUserCommand) -> Result<Vec<LocationResponse>, Error> {
        if command.pagination.is_some() && command.pagination.as_ref().unwrap().search.is_some() {
            let locations = self.location_repo.search_by_user(command.user_id, command.pagination.as_ref().unwrap().search.clone().unwrap(), Some(command.auth_user.user_id)).await;
            match locations {
                Ok(locations) => {
                    let locations = locations.into_iter().map(LocationResponse::from).collect();
                    Ok(locations)
                },
                Err(_) => Err(Error::msg("Error getting locations"))
            }
        } else {
            let locations_cache = self.get_cache_location_by_user(&command.user_id).await?;
            if let Some(locations) = locations_cache {
                return Ok(locations);
            }

            let locations = self.location_repo.get_by_user(command.user_id, Some(command.auth_user.user_id)).await;
            match locations {
                Ok(locations) => {
                    let locations = locations.into_iter().map(LocationResponse::from).collect();
                    self.cache_location_by_user(&command.user_id, &locations).await?;
                    Ok(locations)
                },
                Err(_) => Err(Error::msg("Error getting locations"))
            }
        }
    }
}

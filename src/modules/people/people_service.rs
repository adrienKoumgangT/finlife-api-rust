use anyhow::Result;
use async_trait::async_trait;
use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use uuid::Uuid;

use crate::modules::people::{
    people_command::*,
    people_dto::*,
    people_model::People,
    people_repo::{PeopleRepository, PeopleRepositoryInterface}
};
use crate::shared::{
    db::redis::{delete_key, get_key, set_key},
    errors::AppError,
    state::AppState,
};

#[async_trait]
pub trait PeopleInterface {

    async fn get(&self, command: PeopleGetCommand) -> Result<Option<PeopleResponse>, AppError>;

    async fn create(&self, command: PeopleCreateCommand) -> Result<PeopleResponse, AppError>;

    async fn update_image(&self, command: PeopleUpdateImageCommand) -> Result<Option<PeopleResponse>, AppError>;

    async fn update(&self, command: PeopleUpdateCommand) -> Result<Option<PeopleResponse>, AppError>;

    async fn archived(&self, command: PeopleArchivedCommand) -> Result<Option<PeopleResponse>, AppError>;

    async fn delete(&self, command: PeopleDeleteCommand) -> Result<(), AppError>;

    async fn get_by_user(&self, command: PeopleListByUserCommand) -> Result<Vec<PeopleResponse>, AppError>;

}

#[derive(Clone)]
pub struct PeopleService {
    people_repo: PeopleRepository,
    redis_pool: Option<Pool<RedisConnectionManager>>,
}

impl From<&AppState> for PeopleService {
    fn from(app_state: &AppState) -> Self {
        Self {
            people_repo: PeopleRepository::from(app_state),
            redis_pool: app_state.redis_pool.clone()
        }
    }
}

impl PeopleService {
    fn redis_key_ttl(&self) -> Option<u64> {
        Some(60*60)
    }

    fn form_redis_key_person(&self, key: &Uuid) -> String {
        format!("person:{}", key)
    }

    fn form_redis_key_list_by_user(&self, user: &Uuid) -> String {
        format!("user:{}:people", user)
    }

    async fn cache_person(&self, person: &PeopleResponse) -> Result<(), AppError> {
        if let Some(redis_pool) = &self.redis_pool {
            let _: () = set_key(
                &redis_pool,
                self.form_redis_key_person(&person.people_id).as_str(),
                &person,
                self.redis_key_ttl()
            ).await.map_err(AppError::Internal)?;
        }
        Ok(())
    }
    
    async fn cache_people_by_user(&self, user: &Uuid, people: &Vec<PeopleResponse>) -> Result<(), AppError> {
        if let Some(redis_pool) = &self.redis_pool {
            let _: () = set_key(
                &redis_pool,
                self.form_redis_key_list_by_user(user).as_str(),
                &people,
                self.redis_key_ttl()
            ).await.map_err(AppError::Internal)?;
        }
        Ok(())
    }

    async fn get_cache_person(&self, key: &Uuid) -> Result<Option<PeopleResponse>, AppError> {
        if let Some(redis_pool) = &self.redis_pool {
            let people_cache: Option<PeopleResponse> = get_key(
                &redis_pool,
                self.form_redis_key_person(key).as_str()
            ).await.map_err(AppError::Internal)?;
            if let Some(people) = people_cache {
                return Ok(Some(people));
            }
        }
        Ok(None)
    }
    
    async fn get_cache_people_by_user(&self, user: &Uuid) -> Result<Option<Vec<PeopleResponse>>, AppError> {
        if let Some(redis_pool) = &self.redis_pool {
            let people_cache: Option<Vec<PeopleResponse>> = get_key(
                &redis_pool,
                self.form_redis_key_list_by_user(user).as_str()
            ).await.map_err(AppError::Internal)?;
            if let Some(people) = people_cache {
                return Ok(Some(people));
            }
        }
        Ok(None)
    }

    async fn delete_cache(&self, key: &Uuid, user: &Uuid) -> Result<(), AppError> {
        if let Some(redis_pool) = &self.redis_pool {
            let _: () = delete_key(&redis_pool, self.form_redis_key_person(key).as_str()).await
                .map_err(AppError::Internal)?;
            let _: () = delete_key(&redis_pool, self.form_redis_key_list_by_user(user).as_str()).await
                .map_err(AppError::Internal)?;
        }
        Ok(())
    }

    async fn handle_res_opt_person(&self, people: Option<People>) -> Result<Option<PeopleResponse>, AppError> {
        match people {
            Some(people) => {
                let people_response = PeopleResponse::from(people);
                self.cache_person(&people_response).await?;
                Ok(Some(people_response))
            },
            None => Ok(None)
        }
    }
}

#[async_trait]
impl PeopleInterface for PeopleService {
    async fn get(&self, command: PeopleGetCommand) -> Result<Option<PeopleResponse>, AppError> {
        let person_cache = self.get_cache_person(&command.people_id).await?;
        if let Some(person) = person_cache {
            return Ok(Some(person));
        }

        let person = self.people_repo.get(command.people_id, Some(command.auth_user.user_id)).await?;
        self.handle_res_opt_person(person).await
    }

    async fn create(&self, command: PeopleCreateCommand) -> Result<PeopleResponse, AppError> {
        let meta_user = command.auth_user.user_id.clone();
        let person_create = People::from(command);

        let person = self.people_repo.create(person_create, Some(meta_user.clone())).await?;
        
        let person_response = PeopleResponse::from(person);
        self.delete_cache(&person_response.people_id, &meta_user).await?;
        self.cache_person(&person_response).await?;
        
        Ok(person_response)
    }

    async fn update_image(&self, command: PeopleUpdateImageCommand) -> Result<Option<PeopleResponse>, AppError> {
        let meta_user = Some(command.auth_user.user_id.clone());

        let person = self.people_repo.update_image(command.people_id, command.image, meta_user).await?;
        self.handle_res_opt_person(person).await
    }

    async fn update(&self, command: PeopleUpdateCommand) -> Result<Option<PeopleResponse>, AppError> {
        let meta_user = Some(command.auth_user.user_id.clone());

        let person = self.people_repo.update(command.people_id, command.name, command.email, command.phone, command.note, meta_user).await?;
        self.handle_res_opt_person(person).await
    }

    async fn archived(&self, command: PeopleArchivedCommand) -> Result<Option<PeopleResponse>, AppError> {
        let meta_user = Some(command.auth_user.user_id.clone());

        let person = self.people_repo.archived(command.people_id, command.archived, meta_user).await?;
        
        self.handle_res_opt_person(person).await
    }

    async fn delete(&self, command: PeopleDeleteCommand) -> Result<(), AppError> {
        self.people_repo.delete(command.people_id.clone(), Some(command.auth_user.user_id)).await?;
        
        self.delete_cache(&command.people_id, &command.auth_user.user_id).await?;
        
        Ok(())
    }

    async fn get_by_user(&self, command: PeopleListByUserCommand) -> Result<Vec<PeopleResponse>, AppError> {
        if command.pagination.is_some() && command.pagination.as_ref().unwrap().search.is_some() {
            let people = self.people_repo.search_by_user(
                command.user_id, 
                command.pagination.as_ref().unwrap().search.clone().unwrap()
            ).await?;
            
            let people_response = people.into_iter().map(PeopleResponse::from).collect();
            Ok(people_response)
        } else {
            let people_cache = self.get_cache_people_by_user(&command.user_id).await?;
            if let Some(people) = people_cache {
                return Ok(people);
            }

            let people = self.people_repo.get_by_user(command.user_id).await?;
            
            let people_response = people.into_iter().map(PeopleResponse::from).collect();
            self.cache_people_by_user(&command.user_id, &people_response).await?;
            Ok(people_response)
        }
    }
}

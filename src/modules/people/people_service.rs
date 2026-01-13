use anyhow::{Error, Result};
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
use crate::shared::db::redis::{delete_key, get_key, set_key};
use crate::shared::state::AppState;

#[async_trait]
pub trait PeopleInterface {

    async fn get(&self, command: PeopleGetCommand) -> Result<Option<PeopleResponse>, Error>;

    async fn create(&self, command: PeopleCreateCommand) -> Result<PeopleResponse, Error>;

    async fn update_image(&self, command: PeopleUpdateImageCommand) -> Result<Option<PeopleResponse>, Error>;

    async fn update(&self, command: PeopleUpdateCommand) -> Result<Option<PeopleResponse>, Error>;

    async fn archived(&self, command: PeopleArchivedCommand) -> Result<Option<PeopleResponse>, Error>;

    async fn delete(&self, command: PeopleDeleteCommand) -> Result<(), Error>;

    async fn get_all(&self, command: PeopleListCommand) -> Result<Vec<PeopleResponse>, Error>;

    async fn get_by_user(&self, command: PeopleListByUserCommand) -> Result<Vec<PeopleResponse>, Error>;

}

#[derive(Clone)]
pub struct PeopleService {
    people_repo: PeopleRepository,
    redis_pool: Option<Pool<RedisConnectionManager>>,
}

impl From<&AppState> for PeopleService {
    fn from(app_state: &AppState) -> Self {
        let people_repo = PeopleRepository::from(app_state);
        let redis_pool = app_state.redis_pool.clone();
        Self { people_repo, redis_pool: Option::from(redis_pool) }
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

    async fn cache_person(&self, person: &PeopleResponse) -> Result<(), Error> {
        if let Some(redis_pool) = &self.redis_pool {
            let _: () = set_key(
                &redis_pool,
                self.form_redis_key_person(&person.people_id).as_str(),
                &person,
                self.redis_key_ttl()
            ).await?;
        }
        Ok(())
    }
    
    async fn cache_people_by_user(&self, user: &Uuid, people: &Vec<PeopleResponse>) -> Result<(), Error> {
        if let Some(redis_pool) = &self.redis_pool {
            let _: () = set_key(
                &redis_pool,
                self.form_redis_key_list_by_user(user).as_str(),
                &people,
                self.redis_key_ttl()
            ).await?;
        }
        Ok(())
    }

    async fn get_cache_person(&self, key: &Uuid) -> Result<Option<PeopleResponse>, Error> {
        if let Some(redis_pool) = &self.redis_pool {
            let people_cache: Option<PeopleResponse> = get_key(
                &redis_pool,
                self.form_redis_key_person(key).as_str()
            ).await?;
            if let Some(people) = people_cache {
                return Ok(Some(people));
            }
        }
        Ok(None)
    }
    
    async fn get_cache_people_by_user(&self, user: &Uuid) -> Result<Option<Vec<PeopleResponse>>, Error> {
        if let Some(redis_pool) = &self.redis_pool {
            let people_cache: Option<Vec<PeopleResponse>> = get_key(
                &redis_pool,
                self.form_redis_key_list_by_user(user).as_str()
            ).await?;
            if let Some(people) = people_cache {
                return Ok(Some(people));
            }
        }
        Ok(None)
    }

    async fn delete_cache(&self, key: &Uuid, user: &Uuid) -> Result<(), Error> {
        if let Some(redis_pool) = &self.redis_pool {
            let _: () = delete_key(&redis_pool, self.form_redis_key_person(key).as_str()).await?;
            let _: () = delete_key(&redis_pool, self.form_redis_key_list_by_user(user).as_str()).await?;
        }
        Ok(())
    }

    async fn delete_cache_by_user(&self, user: &Uuid) -> Result<(), Error> {
        if let Some(redis_pool) = &self.redis_pool {
            let _: () = delete_key(&redis_pool, self.form_redis_key_list_by_user(user).as_str()).await?;
        }
        Ok(())
    }

    async fn handle_res_opt_person(&self, people: Result<Option<People>, Error>) -> Result<Option<PeopleResponse>, Error> {
        match people {
            Ok(people) => {
                match people {
                    Some(people) => {
                        let people_response = PeopleResponse::from(people);
                        self.cache_person(&people_response).await?;
                        Ok(Some(people_response))
                    },
                    None => Ok(None)
                }
            },
            Err(_) => Err(Error::msg("Error from repository")),
        }
    }
}

#[async_trait]
impl PeopleInterface for PeopleService {
    async fn get(&self, command: PeopleGetCommand) -> Result<Option<PeopleResponse>, Error> {
        let person_cache = self.get_cache_person(&command.people_id).await?;
        if let Some(person) = person_cache {
            return Ok(Some(person));
        }

        let person = self.people_repo.get(command.people_id, Some(command.auth_user.user_id)).await;
        self.handle_res_opt_person(person).await
    }

    async fn create(&self, command: PeopleCreateCommand) -> Result<PeopleResponse, Error> {
        let meta_user = command.auth_user.user_id.clone();
        let person_create = People::from(command);

        let person = self.people_repo.create(person_create, Some(meta_user)).await;
        match person {
            Ok(person) => {
                let person_response = PeopleResponse::from(person);
                self.cache_person(&person_response).await?;
                Ok(person_response)
            }
            Err(_) => Err(Error::msg("Error creating person"))
        }
    }

    async fn update_image(&self, command: PeopleUpdateImageCommand) -> Result<Option<PeopleResponse>, Error> {
        let meta_user = Some(command.auth_user.user_id.clone());

        let person = self.people_repo.update_image(command.people_id, command.people_image_url, meta_user).await;
        self.handle_res_opt_person(person).await
    }

    async fn update(&self, command: PeopleUpdateCommand) -> Result<Option<PeopleResponse>, Error> {
        let meta_user = Some(command.auth_user.user_id.clone());

        let person = self.people_repo.update(command.people_id, command.people_name, command.people_email, command.people_phone, command.people_note, meta_user).await;
        self.handle_res_opt_person(person).await
    }

    async fn archived(&self, command: PeopleArchivedCommand) -> Result<Option<PeopleResponse>, Error> {
        let meta_user = Some(command.auth_user.user_id.clone());

        let person = self.people_repo.archived(command.people_id, command.people_archived, meta_user).await;
        self.handle_res_opt_person(person).await
    }

    async fn delete(&self, command: PeopleDeleteCommand) -> Result<(), Error> {
        let result = self.people_repo.delete(command.people_id.clone(), Some(command.auth_user.user_id)).await;
        self.delete_cache(&command.people_id, &command.auth_user.user_id).await?;
        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(Error::msg("Error deleting person")),
        }
    }

    async fn get_all(&self, command: PeopleListCommand) -> Result<Vec<PeopleResponse>, Error> {
        let mut limit: Option<u32> = None;
        let mut offset: Option<u32> = None;

        if let Some(pagination) = command.pagination {
            limit = pagination.page_size;

            if let (Some(page_size), Some(page)) = (pagination.page_size, pagination.page) {
                offset = Some(page * page_size);
            }
        }
        
        let people = self.people_repo.get_all(limit, offset, Some(command.auth_user.user_id)).await;
        match people {
            Ok(people) => Ok(people.into_iter().map(PeopleResponse::from).collect()),
            Err(_) => return Err(Error::msg("Error getting people")),
        }
    }

    async fn get_by_user(&self, command: PeopleListByUserCommand) -> Result<Vec<PeopleResponse>, Error> {
        let people_cache = self.get_cache_people_by_user(&command.user_id).await?;
        if let Some(people) = people_cache {
            return Ok(people);
        }
        
        let people = self.people_repo.get_by_user(command.user_id, Some(command.auth_user.user_id)).await;
        match people {
            Ok(people) => {
                let people_response = people.into_iter().map(PeopleResponse::from).collect();
                self.cache_people_by_user(&command.user_id, &people_response).await?;
                Ok(people_response)
            },
            Err(_) => Err(Error::msg("Error getting people")),
        }
    }
}

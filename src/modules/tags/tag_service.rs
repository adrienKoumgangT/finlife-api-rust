use anyhow::Result;
use async_trait::async_trait;
use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use uuid::Uuid;

use crate::modules::tags::{
    tag_command::*,
    tag_dto::*,
    tag_model::Tag,
    tag_repo::{TagRepository, TagRepositoryInterface}
};
use crate::shared::{
    db::redis::{delete_key, get_key, set_key},
    errors::AppError,
    state::AppState
};

#[async_trait]
pub trait TagInterface {
    
    async fn get(&self, command: TagGetCommand) -> Result<Option<TagResponse>, AppError>;
    
    async fn create(&self, command: TagCreateCommand) -> Result<TagResponse, AppError>;
    
    async fn delete(&self, command: TagDeleteCommand) -> Result<(), AppError>;
    
    async fn get_by_user(&self, command: TagListByUserCommand) -> Result<Vec<TagResponse>, AppError>;
    
}

#[derive(Clone)]
pub struct TagService {
    tag_repo: TagRepository,
    redis_pool: Option<Pool<RedisConnectionManager>>,
}

impl From<&AppState> for TagService {
    fn from(app_state: &AppState) -> Self {
        Self {
            tag_repo: TagRepository::from(app_state),
            redis_pool: app_state.redis_pool.clone()
        }
    }
}

impl TagService {
    fn redis_key_ttl(&self) -> Option<u64> { Some(60 * 60) }

    fn form_redis_key_tag(&self, key: &Uuid) -> String { format!("tag:{}", key) }

    fn form_redis_key_list_by_user(&self, user: &Uuid) -> String { format!("user:{}:tags", user) }

    async fn cache_tag(&self, category: &TagResponse) -> Result<(), AppError> {
        if let Some(redis_pool) = &self.redis_pool {
            let _: () = set_key(
                &redis_pool,
                self.form_redis_key_tag(&category.tag_id).as_str(),
                &category,
                self.redis_key_ttl()
            ).await.map_err(AppError::Internal)?;
        }
        Ok(())
    }

    async fn cache_tags_by_user(&self, user: &Uuid, tags: &Vec<TagResponse>) -> Result<(), AppError> {
        if let Some(redis_pool) = &self.redis_pool {
            let _: () = set_key(
                &redis_pool,
                self.form_redis_key_list_by_user(user).as_str(),
                &tags,
                self.redis_key_ttl()
            ).await.map_err(AppError::Internal)?;
        }
        Ok(())
    }

    async fn get_cache_tag(&self, key: &Uuid) -> Result<Option<TagResponse>, AppError> {
        if let Some(redis_pool) = &self.redis_pool {
            let category_cache: Option<TagResponse> = get_key(
                &redis_pool,
                self.form_redis_key_tag(key).as_str()
            ).await.map_err(AppError::Internal)?;
            return Ok(category_cache);
        }
        Ok(None)
    }
    
    async fn get_cache_tag_by_user(&self, user: &Uuid) -> Result<Option<Vec<TagResponse>>, AppError> {
        if let Some(redis_pool) = &self.redis_pool {
            let category_cache: Option<Vec<TagResponse>> = get_key(
                &redis_pool,
                self.form_redis_key_list_by_user(user).as_str()
            ).await.map_err(AppError::Internal)?;
            return Ok(category_cache);
        }
        Ok(None)
    }

    async fn delete_cache(&self, key: &Uuid, user: &Uuid) -> Result<(), AppError> {
        if let Some(redis_pool) = &self.redis_pool {
            let _: () = delete_key(&redis_pool, self.form_redis_key_tag(key).as_str()).await
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

    async fn handle_res_opt_tag(&self, tag: Result<Option<Tag>>, user: &Uuid) -> Result<Option<TagResponse>, AppError> {
        let category = tag.map_err(AppError::Internal)?;

        if let Some(cat) = category {
            let response = TagResponse::from(cat);
            self.cache_tag(&response).await?;
            self.delete_cache_list(user).await?;

            Ok(Some(response))
        } else {
            Ok(None)
        }
    }
}

#[async_trait]
impl TagInterface for TagService {
    async fn get(&self, command: TagGetCommand) -> Result<Option<TagResponse>, AppError> {
        let cache = self.get_cache_tag(&command.tag_id).await?;
        if let Some(tag) = cache {
            return Ok(Some(tag));
        }
        
        let tag = self.tag_repo.get(command.tag_id, command.auth_user.user_id).await;
        self.handle_res_opt_tag(tag, &command.auth_user.user_id).await
    }

    async fn create(&self, command: TagCreateCommand) -> Result<TagResponse, AppError> {
        let meta_user = command.auth_user.user_id.clone();
        let tag_create = Tag::from(command);
        
        let tag = self.tag_repo.create(tag_create, meta_user).await
            .map_err(AppError::Internal)?;
        let response = TagResponse::from(tag);
        
        self.cache_tag(&response).await?;
        self.delete_cache_list(&meta_user).await?;
        
        Ok(response)
    }

    async fn delete(&self, command: TagDeleteCommand) -> Result<(), AppError> {
        self.tag_repo.delete(command.tag_id.clone(), command.auth_user.user_id.clone()).await
            .map_err(AppError::Internal)?;
        
        self.delete_cache(&command.tag_id, &command.auth_user.user_id).await?;

        Ok(())
    }

    async fn get_by_user(&self, command: TagListByUserCommand) -> Result<Vec<TagResponse>, AppError> {
        let cache = self.get_cache_tag_by_user(&command.user_id).await?;
        if let Some(tags) = cache { return Ok(tags); }
        
        let tags = self.tag_repo.get_by_user(command.user_id).await
            .map_err(AppError::Internal)?;
        
        let response: Vec<TagResponse> = tags.into_iter().map(TagResponse::from).collect();
        self.cache_tags_by_user(&command.user_id, &response).await?;
        
        Ok(response)
    }
}

use anyhow::Result;
use async_trait::async_trait;
use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use uuid::Uuid;

use crate::modules::categories::{
    category_command::*,
    category_dto::*,
    category_model::Category,
    category_repo::{CategoryRepository, CategoryRepositoryInterface}
};
use crate::shared::{
    db::redis::{delete_key, get_key, set_key},
    errors::AppError,
    state::AppState,
    utils::extract_pagination_data
};

#[async_trait]
pub trait CategoryInterface {

    async fn get(&self, command: CategoryGetCommand) -> Result<Option<CategoryResponse>, AppError>;

    async fn create(&self, command: CategoryCreateCommand) -> Result<CategoryResponse, AppError>;

    async fn update(&self, command: CategoryUpdateCommand) -> Result<Option<CategoryResponse>, AppError>;

    async fn archived(&self, command: CategoryArchivedCommand) -> Result<Option<CategoryResponse>, AppError>;

    async fn delete(&self, command: CategoryDeleteCommand) -> Result<(), AppError>;

    async fn get_by_user(&self, command: CategoryListByUserCommand) -> Result<Vec<CategoryResponse>, AppError>;

}

#[derive(Clone)]
pub struct CategoryService {
    category_repo: CategoryRepository,
    redis_pool: Option<Pool<RedisConnectionManager>>,
}

impl From<&AppState> for CategoryService {
    fn from(app_state: &AppState) -> Self {
        Self {
            category_repo: CategoryRepository::from(app_state),
            redis_pool: app_state.redis_pool.clone()
        }
    }
}

impl CategoryService {
    fn redis_key_ttl(&self) -> Option<u64> { Some(60 * 60) }

    fn form_redis_key_category(&self, key: &Uuid) -> String { format!("category:{}", key) }

    fn form_redis_key_list_by_user(&self, user: &Uuid) -> String { format!("user:{}:categories", user) }

    async fn cache_category(&self, category: &CategoryResponse) -> Result<(), AppError> {
        if let Some(redis_pool) = &self.redis_pool {
            let _: () = set_key(
                &redis_pool,
                self.form_redis_key_category(&category.category_id).as_str(),
                &category,
                self.redis_key_ttl()
            ).await.map_err(AppError::Internal)?;
        }
        Ok(())
    }

    async fn cache_categories_by_user(&self, user: &Uuid, categories: &Vec<CategoryResponse>) -> Result<(), AppError> {
        if let Some(redis_pool) = &self.redis_pool {
            let _: () = set_key(
                &redis_pool,
                self.form_redis_key_list_by_user(user).as_str(),
                &categories,
                self.redis_key_ttl()
            ).await.map_err(AppError::Internal)?;
        }
        Ok(())
    }

    async fn get_cache_category(&self, key: &Uuid) -> Result<Option<CategoryResponse>, AppError> {
        if let Some(redis_pool) = &self.redis_pool {
            let category_cache: Option<CategoryResponse> = get_key(
                &redis_pool,
                self.form_redis_key_category(key).as_str()
            ).await.map_err(AppError::Internal)?;
            return Ok(category_cache);
        }
        Ok(None)
    }

    async fn get_cache_category_by_user(&self, user: &Uuid) -> Result<Option<Vec<CategoryResponse>>, AppError> {
        if let Some(redis_pool) = &self.redis_pool {
            let category_cache: Option<Vec<CategoryResponse>> = get_key(
                &redis_pool,
                self.form_redis_key_list_by_user(user).as_str()
            ).await.map_err(AppError::Internal)?;
            return Ok(category_cache);
        }
        Ok(None)
    }

    async fn delete_cache(&self, key: &Uuid, user: &Uuid) -> Result<(), AppError> {
        if let Some(redis_pool) = &self.redis_pool {
            let _: () = delete_key(&redis_pool, self.form_redis_key_category(key).as_str()).await
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

    async fn handle_res_opt_category(&self, category: Result<Option<Category>>, delete_cache_list: bool, auth_user: &Uuid) -> Result<Option<CategoryResponse>, AppError> {
        let category = category.map_err(AppError::Internal)?;

        if let Some(cat) = category {
            let response = CategoryResponse::from(cat);
            self.cache_category(&response).await?;

            // Invalidate the list cache whenever a category is updated
            if delete_cache_list { self.delete_cache_list(auth_user).await?; }

            Ok(Some(response))
        } else {
            Ok(None)
        }
    }
}

#[async_trait]
impl CategoryInterface for CategoryService {
    async fn get(&self, command: CategoryGetCommand) -> Result<Option<CategoryResponse>, AppError> {
        let cache = self.get_cache_category(&command.category_id).await?;
        if let Some(category) = cache {
            return Ok(Some(category));
        }

        let category = self.category_repo.get(command.category_id, command.auth_user.user_id).await;
        self.handle_res_opt_category(category, false, &command.auth_user.user_id).await
    }

    async fn create(&self, command: CategoryCreateCommand) -> Result<CategoryResponse, AppError> {
        let meta_user = command.auth_user.user_id.clone();
        let category_create = Category::from(command);

        let category = self.category_repo.create(category_create, meta_user).await
            .map_err(AppError::Internal)?;
        let response = CategoryResponse::from(category);

        self.cache_category(&response).await?;
        self.delete_cache_list(&meta_user).await?;

        Ok(response)
    }

    async fn update(&self, command: CategoryUpdateCommand) -> Result<Option<CategoryResponse>, AppError> {
        let category = self.category_repo.update(
            command.category_id, command.name, command.parent_id,
            command.sort_order.unwrap_or(0), command.auth_user.user_id
        ).await;

        self.handle_res_opt_category(category, true, &command.auth_user.user_id).await
    }

    async fn archived(&self, command: CategoryArchivedCommand) -> Result<Option<CategoryResponse>, AppError> {
        let category = self.category_repo.archived(
            command.category_id, command.archived, command.auth_user.user_id
        ).await;

        self.handle_res_opt_category(category, true, &command.auth_user.user_id).await
    }

    async fn delete(&self, command: CategoryDeleteCommand) -> Result<(), AppError> {
        self.category_repo.delete(command.category_id.clone(), command.auth_user.user_id).await
            .map_err(AppError::Internal)?;

        self.delete_cache(&command.category_id, &command.auth_user.user_id).await?;

        Ok(())
    }

    async fn get_by_user(&self, command: CategoryListByUserCommand) -> Result<Vec<CategoryResponse>, AppError> {
        let cache = self.get_cache_category_by_user(&command.user_id).await?;
        if let Some(categories) = cache { return Ok(categories); }

        let categories = self.category_repo.get_by_user(command.user_id).await
            .map_err(AppError::Internal)?;

        let response = categories.into_iter().map(CategoryResponse::from).collect();
        self.cache_categories_by_user(&command.user_id, &response).await?;

        Ok(response)
    }
}

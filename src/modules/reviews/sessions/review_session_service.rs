use anyhow::Result;
use async_trait::async_trait;
use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use uuid::Uuid;

use crate::modules::reviews::sessions::{
    review_session_command::*,
    review_session_dto::*,
    review_session_model::ReviewSession,
    review_session_repo::{ReviewSessionRepository, ReviewSessionRepositoryInterface}
};
use crate::shared::{
    db::redis::{delete_key, get_key, set_key},
    errors::AppError,
    state::AppState,
    utils::extract_pagination_data
};

#[async_trait]
pub trait ReviewSessionInterface {

    async fn get(&self, command: ReviewSessionGetCommand) -> Result<Option<ReviewSessionResponse>, AppError>;

    async fn create(&self, command: ReviewSessionCreateCommand) -> Result<ReviewSessionResponse, AppError>;

    async fn update(&self, command: ReviewSessionUpdateCommand) -> Result<Option<ReviewSessionResponse>, AppError>;

    async fn delete(&self, command: ReviewSessionDeleteCommand) -> Result<(), AppError>;

    async fn get_by_user(&self, command: ReviewSessionListByUserCommand) -> Result<Vec<ReviewSessionResponse>, AppError>;

}

#[derive(Clone)]
pub struct ReviewSessionService {
    review_repo: ReviewSessionRepository,
    redis_pool: Option<Pool<RedisConnectionManager>>,
}

impl From<&AppState> for ReviewSessionService {
    fn from(app_state: &AppState) -> Self {
        Self {
            review_repo: ReviewSessionRepository::from(app_state),
            redis_pool: app_state.redis_pool.clone()
        }
    }
}

impl ReviewSessionService {
    fn redis_key_ttl(&self) -> Option<u64> { Some(60 * 60) }

    fn form_redis_key_review(&self, key: &Uuid) -> String { format!("review_session:{}", key) }

    fn form_redis_key_list_by_user(&self, user: &Uuid) -> String { format!("user:{}:review_sessions", user) }

    async fn cache_review(&self, review: &ReviewSessionResponse) -> Result<(), AppError> {
        if let Some(redis_pool) = &self.redis_pool {
            let _: () = set_key(
                &redis_pool,
                self.form_redis_key_review(&review.review_session_id).as_str(),
                &review,
                self.redis_key_ttl()
            ).await.map_err(AppError::Internal)?;
        }
        Ok(())
    }

    async fn cache_reviews_by_user(&self, user: &Uuid, reviews: &Vec<ReviewSessionResponse>) -> Result<(), AppError> {
        if let Some(redis_pool) = &self.redis_pool {
            let _: () = set_key(
                &redis_pool,
                self.form_redis_key_list_by_user(user).as_str(),
                &reviews,
                self.redis_key_ttl()
            ).await.map_err(AppError::Internal)?;
        }
        Ok(())
    }

    async fn get_cache_review(&self, key: &Uuid) -> Result<Option<ReviewSessionResponse>, AppError> {
        if let Some(redis_pool) = &self.redis_pool {
            let cache: Option<ReviewSessionResponse> = get_key(
                &redis_pool,
                self.form_redis_key_review(key).as_str()
            ).await.map_err(AppError::Internal)?;
            return Ok(cache);
        }
        Ok(None)
    }

    async fn delete_cache(&self, key: &Uuid, user: &Uuid) -> Result<(), AppError> {
        if let Some(redis_pool) = &self.redis_pool {
            let _: () = delete_key(&redis_pool, self.form_redis_key_review(key).as_str()).await
                .map_err(AppError::Internal)?;
            let _: () = delete_key(&redis_pool, self.form_redis_key_list_by_user(user).as_str()).await
                .map_err(AppError::Internal)?;
        }
        Ok(())
    }

    async fn handle_res_opt_review(&self, review: anyhow::Result<Option<ReviewSession>>, auth_user: &Uuid) -> Result<Option<ReviewSessionResponse>, AppError> {
        let review = review.map_err(AppError::Internal)?;

        if let Some(r) = review {
            let response = ReviewSessionResponse::from(r);
            self.cache_review(&response).await?;

            // Invalidate user list cache whenever a review is updated
            if let Some(redis_pool) = &self.redis_pool {
                let _: () = delete_key(&redis_pool, self.form_redis_key_list_by_user(auth_user).as_str()).await
                    .map_err(AppError::Internal)?;
            }

            Ok(Some(response))
        } else {
            Ok(None)
        }
    }
}

#[async_trait]
impl ReviewSessionInterface for ReviewSessionService {
    async fn get(&self, command: ReviewSessionGetCommand) -> Result<Option<ReviewSessionResponse>, AppError> {
        let cache = self.get_cache_review(&command.review_session_id).await?;
        if let Some(review) = cache {
            return Ok(Some(review));
        }

        let review = self.review_repo.get(command.review_session_id, command.auth_user.user_id).await;
        self.handle_res_opt_review(review, &command.auth_user.user_id).await
    }

    async fn create(&self, command: ReviewSessionCreateCommand) -> Result<ReviewSessionResponse, AppError> {
        let meta_user = command.auth_user.user_id.clone();
        let review_create = ReviewSession::from(command);

        let review = self.review_repo.create(review_create, meta_user).await
            .map_err(AppError::Internal)?;
        let response = ReviewSessionResponse::from(review);

        self.cache_review(&response).await?;

        // Invalidate list cache
        if let Some(redis_pool) = &self.redis_pool {
            let _: () = delete_key(&redis_pool, self.form_redis_key_list_by_user(&meta_user).as_str()).await
                .map_err(AppError::Internal)?;
        }

        Ok(response)
    }

    async fn update(&self, command: ReviewSessionUpdateCommand) -> Result<Option<ReviewSessionResponse>, AppError> {
        let review = self.review_repo.update(
            command.review_session_id, command.period_start, command.period_end,
            command.status, command.notes, command.actions, command.decisions,
            command.auth_user.user_id
        ).await;

        self.handle_res_opt_review(review, &command.auth_user.user_id).await
    }

    async fn delete(&self, command: ReviewSessionDeleteCommand) -> Result<(), AppError> {
        self.review_repo.delete(command.review_session_id.clone(), command.auth_user.user_id).await
            .map_err(AppError::Internal)?;
        self.delete_cache(&command.review_session_id, &command.auth_user.user_id).await?;
        Ok(())
    }

    async fn get_by_user(&self, command: ReviewSessionListByUserCommand) -> Result<Vec<ReviewSessionResponse>, AppError> {
        let (limit, offset, _search) = extract_pagination_data(command.pagination);

        if let Some(redis_pool) = &self.redis_pool {
            let cache: Option<Vec<ReviewSessionResponse>> = get_key(
                &redis_pool, self.form_redis_key_list_by_user(&command.user_id).as_str()
            ).await.map_err(AppError::Internal)?;

            if let Some(reviews) = cache { return Ok(reviews); }
        }

        let reviews = self.review_repo.get_by_user(
            command.user_id, limit, offset
        ).await.map_err(AppError::Internal)?;

        let response: Vec<ReviewSessionResponse> = reviews.into_iter().map(ReviewSessionResponse::from).collect();
        self.cache_reviews_by_user(&command.user_id, &response).await?;

        Ok(response)
    }
}

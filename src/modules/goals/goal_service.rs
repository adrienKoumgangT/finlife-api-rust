use anyhow::Result;
use async_trait::async_trait;
use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use uuid::Uuid;

use crate::modules::goals::{
    goal_command::*,
    goal_dto::*,
    goal_model::Goal,
    goal_repo::{GoalRepository, GoalRepositoryInterface}
};
use crate::shared::{
    db::redis::{delete_key, get_key, set_key},
    errors::AppError,
    state::AppState,
    utils::extract_pagination_data
};

#[async_trait]
pub trait GoalInterface {

    async fn get(&self, command: GoalGetCommand) -> Result<Option<GoalResponse>, AppError>;

    async fn create(&self, command: GoalCreateCommand) -> Result<GoalResponse, AppError>;

    async fn update(&self, command: GoalUpdateCommand) -> Result<Option<GoalResponse>, AppError>;

    async fn delete(&self, command: GoalDeleteCommand) -> Result<(), AppError>;

    async fn get_by_user(&self, command: GoalListByUserCommand) -> Result<Vec<GoalResponse>, AppError>;

}

#[derive(Clone)]
pub struct GoalService {
    goal_repo: GoalRepository,
    redis_pool: Option<Pool<RedisConnectionManager>>,
}

impl From<&AppState> for GoalService {
    fn from(app_state: &AppState) -> Self {
        let goal_repo = GoalRepository::from(app_state);
        let redis_pool = app_state.redis_pool.clone();
        Self { goal_repo, redis_pool: Option::from(redis_pool) }
    }
}

impl GoalService {
    fn redis_key_ttl(&self) -> Option<u64> { Some(60 * 60) }

    fn form_redis_key_goal(&self, key: &Uuid) -> String { format!("goal:{}", key) }

    fn form_redis_key_list_by_user(&self, user: &Uuid) -> String { format!("user:{}:goals", user) }

    async fn cache_goal(&self, goal: &GoalResponse) -> Result<(), AppError> {
        if let Some(redis_pool) = &self.redis_pool {
            let _: () = set_key(
                &redis_pool,
                self.form_redis_key_goal(&goal.goal_id).as_str(),
                &goal,
                self.redis_key_ttl()
            ).await.map_err(AppError::Internal)?;
        }
        Ok(())
    }

    async fn cache_goals_by_user(&self, user: &Uuid, goals: &Vec<GoalResponse>) -> Result<(), AppError> {
        if let Some(redis_pool) = &self.redis_pool {
            let _: () = set_key(
                &redis_pool,
                self.form_redis_key_list_by_user(user).as_str(),
                &goals,
                self.redis_key_ttl()
            ).await.map_err(AppError::Internal)?;
        }
        Ok(())
    }

    async fn get_cache_goal(&self, key: &Uuid) -> Result<Option<GoalResponse>, AppError> {
        if let Some(redis_pool) = &self.redis_pool {
            let goal_cache: Option<GoalResponse> = get_key(
                &redis_pool,
                self.form_redis_key_goal(key).as_str()
            ).await.map_err(AppError::Internal)?;
            return Ok(goal_cache);
        }
        Ok(None)
    }

    async fn delete_cache(&self, key: &Uuid, user: &Uuid) -> Result<(), AppError> {
        if let Some(redis_pool) = &self.redis_pool {
            let _: () = delete_key(&redis_pool, self.form_redis_key_goal(key).as_str()).await
                .map_err(AppError::Internal)?;
            let _: () = delete_key(&redis_pool, self.form_redis_key_list_by_user(user).as_str()).await
                .map_err(AppError::Internal)?;
        }
        Ok(())
    }

    async fn handle_res_opt_goal(&self, goal: anyhow::Result<Option<Goal>>, auth_user: &Uuid) -> Result<Option<GoalResponse>, AppError> {
        let goal = goal.map_err(AppError::Internal)?;

        if let Some(g) = goal {
            let response = GoalResponse::from(g);
            self.cache_goal(&response).await?;

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
impl GoalInterface for GoalService {
    async fn get(&self, command: GoalGetCommand) -> Result<Option<GoalResponse>, AppError> {
        let cache = self.get_cache_goal(&command.goal_id).await?;
        if let Some(goal) = cache {
            return Ok(Some(goal));
        }

        let goal = self.goal_repo.get(command.goal_id, Some(command.auth_user.user_id)).await;
        self.handle_res_opt_goal(goal, &command.auth_user.user_id).await
    }

    async fn create(&self, command: GoalCreateCommand) -> Result<GoalResponse, AppError> {
        let meta_user = command.auth_user.user_id.clone();
        let goal_create = Goal::from(command);

        let goal = self.goal_repo.create(goal_create, Some(meta_user)).await
            .map_err(AppError::Internal)?;
        let response = GoalResponse::from(goal);

        self.cache_goal(&response).await?;

        if let Some(redis_pool) = &self.redis_pool {
            let _: () = delete_key(&redis_pool, self.form_redis_key_list_by_user(&meta_user).as_str()).await
                .map_err(AppError::Internal)?;
        }

        Ok(response)
    }

    async fn update(&self, command: GoalUpdateCommand) -> Result<Option<GoalResponse>, AppError> {
        let goal = self.goal_repo.update(
            command.goal_id, command.name, command.goal_type,
            command.target_base_minor, command.target_date, command.priority,
            command.linked_account_id, Some(command.auth_user.user_id)
        ).await;

        self.handle_res_opt_goal(goal, &command.auth_user.user_id).await
    }

    async fn delete(&self, command: GoalDeleteCommand) -> Result<(), AppError> {
        self.goal_repo.delete(command.goal_id.clone(), Some(command.auth_user.user_id)).await
            .map_err(AppError::Internal)?;
        self.delete_cache(&command.goal_id, &command.auth_user.user_id).await?;
        Ok(())
    }

    async fn get_by_user(&self, command: GoalListByUserCommand) -> Result<Vec<GoalResponse>, AppError> {
        let (limit, offset, _search) = extract_pagination_data(command.pagination);

        if let Some(redis_pool) = &self.redis_pool {
            let cache: Option<Vec<GoalResponse>> = get_key(
                &redis_pool, self.form_redis_key_list_by_user(&command.user_id).as_str()
            ).await.map_err(AppError::Internal)?;

            if let Some(goals) = cache { return Ok(goals); }
        }

        let goals = self.goal_repo.get_by_user(
            command.user_id, limit, offset
        ).await.map_err(AppError::Internal)?;

        let response: Vec<GoalResponse> = goals.into_iter().map(GoalResponse::from).collect();
        self.cache_goals_by_user(&command.user_id, &response).await?;

        Ok(response)
    }
}

use anyhow::Result;
use async_trait::async_trait;
use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use uuid::Uuid;

use crate::modules::projects::{
    project_milestone_command::*,
    project_milestone_dto::*,
    project_milestone_model::ProjectMilestone,
    project_milestone_repo::{ProjectMilestoneRepository, ProjectMilestoneRepositoryInterface}
};
use crate::shared::{
    db::redis::{delete_key, get_key, set_key},
    errors::AppError,
    state::AppState,
    utils::extract_pagination_data
};

#[async_trait]
pub trait ProjectMilestoneInterface {

    async fn get(&self, command: ProjectMilestoneGetCommand) -> Result<Option<ProjectMilestoneResponse>, AppError>;

    async fn create(&self, command: ProjectMilestoneCreateCommand) -> Result<ProjectMilestoneResponse, AppError>;

    async fn update(&self, command: ProjectMilestoneUpdateCommand) -> Result<Option<ProjectMilestoneResponse>, AppError>;

    async fn delete(&self, command: ProjectMilestoneDeleteCommand) -> Result<(), AppError>;

    async fn get_by_project(&self, command: ProjectMilestoneListByProjectCommand) -> Result<Vec<ProjectMilestoneResponse>, AppError>;

}

#[derive(Clone)]
pub struct ProjectMilestoneService {
    milestone_repo: ProjectMilestoneRepository,
    redis_pool: Option<Pool<RedisConnectionManager>>,
}

impl From<&AppState> for ProjectMilestoneService {
    fn from(app_state: &AppState) -> Self {
        let milestone_repo = ProjectMilestoneRepository::from(app_state);
        let redis_pool = app_state.redis_pool.clone();
        Self { milestone_repo, redis_pool: Option::from(redis_pool) }
    }
}

impl ProjectMilestoneService {
    fn redis_key_ttl(&self) -> Option<u64> { Some(60 * 60) }

    fn form_redis_key_milestone(&self, key: &Uuid) -> String { format!("milestone:{}", key) }

    fn form_redis_key_list_by_project(&self, project_id: &Uuid) -> String { format!("project:{}:milestones", project_id) }

    async fn cache_milestone(&self, milestone: &ProjectMilestoneResponse) -> Result<(), AppError> {
        if let Some(redis_pool) = &self.redis_pool {
            let _: () = set_key(
                &redis_pool,
                self.form_redis_key_milestone(&milestone.milestone_id).as_str(),
                &milestone,
                self.redis_key_ttl()
            ).await.map_err(AppError::Internal)?;
        }
        Ok(())
    }

    async fn cache_milestones_by_project(&self, project_id: &Uuid, milestones: &Vec<ProjectMilestoneResponse>) -> Result<(), AppError> {
        if let Some(redis_pool) = &self.redis_pool {
            let _: () = set_key(
                &redis_pool,
                self.form_redis_key_list_by_project(project_id).as_str(),
                &milestones,
                self.redis_key_ttl()
            ).await.map_err(AppError::Internal)?;
        }
        Ok(())
    }

    async fn get_cache_milestone(&self, key: &Uuid) -> Result<Option<ProjectMilestoneResponse>, AppError> {
        if let Some(redis_pool) = &self.redis_pool {
            let milestone_cache: Option<ProjectMilestoneResponse> = get_key(
                &redis_pool,
                self.form_redis_key_milestone(key).as_str()
            ).await.map_err(AppError::Internal)?;
            return Ok(milestone_cache);
        }
        Ok(None)
    }

    async fn delete_cache(&self, milestone_id: &Uuid, project_id: &Uuid) -> Result<(), AppError> {
        if let Some(redis_pool) = &self.redis_pool {
            let _: () = delete_key(&redis_pool, self.form_redis_key_milestone(milestone_id).as_str()).await
                .map_err(AppError::Internal)?;
            let _: () = delete_key(&redis_pool, self.form_redis_key_list_by_project(project_id).as_str()).await
                .map_err(AppError::Internal)?;
        }
        Ok(())
    }

    async fn handle_res_opt_milestone(&self, milestone: anyhow::Result<Option<ProjectMilestone>>) -> Result<Option<ProjectMilestoneResponse>, AppError> {
        let milestone = milestone.map_err(AppError::Internal)?;

        if let Some(m) = milestone {
            let response = ProjectMilestoneResponse::from(m);
            self.cache_milestone(&response).await?;

            // Invalidate the list cache for this specific project
            if let Some(redis_pool) = &self.redis_pool {
                let _: () = delete_key(&redis_pool, self.form_redis_key_list_by_project(&response.project_id).as_str()).await
                    .map_err(AppError::Internal)?;
            }

            Ok(Some(response))
        } else {
            Ok(None)
        }
    }
}

#[async_trait]
impl ProjectMilestoneInterface for ProjectMilestoneService {
    async fn get(&self, command: ProjectMilestoneGetCommand) -> Result<Option<ProjectMilestoneResponse>, AppError> {
        let cache = self.get_cache_milestone(&command.milestone_id).await?;
        if let Some(milestone) = cache {
            return Ok(Some(milestone));
        }

        let milestone = self.milestone_repo.get(command.milestone_id, Some(command.auth_user.user_id)).await;
        self.handle_res_opt_milestone(milestone).await
    }

    async fn create(&self, command: ProjectMilestoneCreateCommand) -> Result<ProjectMilestoneResponse, AppError> {
        let meta_user = command.auth_user.user_id.clone();
        let milestone_create = ProjectMilestone::from(command);

        // Repo automatically verifies project ownership
        let milestone = self.milestone_repo.create(milestone_create, Some(meta_user)).await
            .map_err(AppError::Internal)?;
        let response = ProjectMilestoneResponse::from(milestone);

        self.cache_milestone(&response).await?;

        if let Some(redis_pool) = &self.redis_pool {
            let _: () = delete_key(&redis_pool, self.form_redis_key_list_by_project(&response.project_id).as_str()).await
                .map_err(AppError::Internal)?;
        }

        Ok(response)
    }

    async fn update(&self, command: ProjectMilestoneUpdateCommand) -> Result<Option<ProjectMilestoneResponse>, AppError> {
        let milestone = self.milestone_repo.update(
            command.milestone_id, command.title, command.due_date, command.status,
            command.person_id, command.location_id, command.note,
            Some(command.auth_user.user_id)
        ).await;

        self.handle_res_opt_milestone(milestone).await
    }

    async fn delete(&self, command: ProjectMilestoneDeleteCommand) -> Result<(), AppError> {
        let milestone_opt = self.milestone_repo.get(command.milestone_id.clone(), Some(command.auth_user.user_id.clone())).await
            .map_err(AppError::Internal)?;

        if let Some(milestone) = milestone_opt {
            self.milestone_repo.delete(command.milestone_id.clone(), Some(command.auth_user.user_id)).await
                .map_err(AppError::Internal)?;
            self.delete_cache(&command.milestone_id, &milestone.project_id).await?;
        }
        Ok(())
    }

    async fn get_by_project(&self, command: ProjectMilestoneListByProjectCommand) -> Result<Vec<ProjectMilestoneResponse>, AppError> {
        let (limit, offset, _search) = extract_pagination_data(command.pagination);

        if let Some(redis_pool) = &self.redis_pool {
            let cache: Option<Vec<ProjectMilestoneResponse>> = get_key(
                &redis_pool, self.form_redis_key_list_by_project(&command.project_id).as_str()
            ).await.map_err(AppError::Internal)?;

            if let Some(milestones) = cache { return Ok(milestones); }
        }

        let milestones = self.milestone_repo.get_by_project(
            command.project_id, limit, offset, Some(command.auth_user.user_id)
        ).await.map_err(AppError::Internal)?;

        let response: Vec<ProjectMilestoneResponse> = milestones.into_iter().map(ProjectMilestoneResponse::from).collect();
        self.cache_milestones_by_project(&command.project_id, &response).await?;

        Ok(response)
    }
}

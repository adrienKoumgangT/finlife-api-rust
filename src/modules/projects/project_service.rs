use anyhow::Result;
use async_trait::async_trait;
use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use uuid::Uuid;

use crate::modules::projects::{
    project_command::*,
    project_dto::*,
    project_model::Project,
    project_repo::{ProjectRepository, ProjectRepositoryInterface}
};
use crate::shared::{
    db::redis::{delete_key, get_key, set_key},
    errors::AppError,
    state::AppState,
    utils::extract_pagination_data
};

#[async_trait]
pub trait ProjectInterface {

    async fn get(&self, command: ProjectGetCommand) -> Result<Option<ProjectResponse>, AppError>;

    async fn create(&self, command: ProjectCreateCommand) -> Result<ProjectResponse, AppError>;

    async fn update(&self, command: ProjectUpdateCommand) -> Result<Option<ProjectResponse>, AppError>;

    async fn delete(&self, command: ProjectDeleteCommand) -> Result<(), AppError>;

    async fn get_by_user(&self, command: ProjectListByUserCommand) -> Result<Vec<ProjectResponse>, AppError>;

}

#[derive(Clone)]
pub struct ProjectService {
    project_repo: ProjectRepository,
    redis_pool: Option<Pool<RedisConnectionManager>>,
}

impl From<&AppState> for ProjectService {
    fn from(app_state: &AppState) -> Self {
        Self {
            project_repo: ProjectRepository::from(app_state),
            redis_pool: app_state.redis_pool.clone()
        }
    }
}

impl ProjectService {
    fn redis_key_ttl(&self) -> Option<u64> { Some(60 * 60) }

    fn form_redis_key_project(&self, key: &Uuid) -> String { format!("project:{}", key) }

    fn form_redis_key_list_by_user(&self, user: &Uuid) -> String { format!("user:{}:projects", user) }

    async fn cache_project(&self, project: &ProjectResponse) -> Result<(), AppError> {
        if let Some(redis_pool) = &self.redis_pool {
            let _: () = set_key(
                &redis_pool,
                self.form_redis_key_project(&project.project_id).as_str(),
                &project,
                self.redis_key_ttl()
            ).await.map_err(AppError::Internal)?;
        }
        Ok(())
    }

    async fn cache_projects_by_user(&self, user: &Uuid, projects: &Vec<ProjectResponse>) -> Result<(), AppError> {
        if let Some(redis_pool) = &self.redis_pool {
            let _: () = set_key(
                &redis_pool,
                self.form_redis_key_list_by_user(user).as_str(),
                &projects,
                self.redis_key_ttl()
            ).await.map_err(AppError::Internal)?;
        }
        Ok(())
    }

    async fn get_cache_project(&self, key: &Uuid) -> Result<Option<ProjectResponse>, AppError> {
        if let Some(redis_pool) = &self.redis_pool {
            let project_cache: Option<ProjectResponse> = get_key(
                &redis_pool,
                self.form_redis_key_project(key).as_str()
            ).await.map_err(AppError::Internal)?;
            return Ok(project_cache);
        }
        Ok(None)
    }

    async fn delete_cache(&self, key: &Uuid, user: &Uuid) -> Result<(), AppError> {
        if let Some(redis_pool) = &self.redis_pool {
            let _: () = delete_key(&redis_pool, self.form_redis_key_project(key).as_str()).await
                .map_err(AppError::Internal)?;
            let _: () = delete_key(&redis_pool, self.form_redis_key_list_by_user(user).as_str()).await
                .map_err(AppError::Internal)?;
        }
        Ok(())
    }

    async fn handle_res_opt_project(&self, project: anyhow::Result<Option<Project>>, auth_user: &Uuid) -> Result<Option<ProjectResponse>, AppError> {
        let project = project.map_err(AppError::Internal)?;

        if let Some(p) = project {
            let response = ProjectResponse::from(p);
            self.cache_project(&response).await?;

            // Invalidate the list cache whenever a project is updated
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
impl ProjectInterface for ProjectService {
    async fn get(&self, command: ProjectGetCommand) -> Result<Option<ProjectResponse>, AppError> {
        let cache = self.get_cache_project(&command.project_id).await?;
        if let Some(project) = cache {
            return Ok(Some(project));
        }

        let project = self.project_repo.get(command.project_id, Some(command.auth_user.user_id)).await;
        self.handle_res_opt_project(project, &command.auth_user.user_id).await
    }

    async fn create(&self, command: ProjectCreateCommand) -> Result<ProjectResponse, AppError> {
        let meta_user = command.auth_user.user_id.clone();
        let project_create = Project::from(command);

        let project = self.project_repo.create(project_create, Some(meta_user)).await
            .map_err(AppError::Internal)?;
        let response = ProjectResponse::from(project);

        self.cache_project(&response).await?;

        // Invalidate list cache
        if let Some(redis_pool) = &self.redis_pool {
            let _: () = delete_key(&redis_pool, self.form_redis_key_list_by_user(&meta_user).as_str()).await
                .map_err(AppError::Internal)?;
        }

        Ok(response)
    }

    async fn update(&self, command: ProjectUpdateCommand) -> Result<Option<ProjectResponse>, AppError> {
        let project = self.project_repo.update(
            command.project_id, command.name, command.status, command.priority,
            command.start_date, command.due_date, command.budget_base_minor,
            command.goal_id, command.person_id, command.location_id, command.description,
            Some(command.auth_user.user_id)
        ).await;

        self.handle_res_opt_project(project, &command.auth_user.user_id).await
    }

    async fn delete(&self, command: ProjectDeleteCommand) -> Result<(), AppError> {
        self.project_repo.delete(command.project_id.clone(), Some(command.auth_user.user_id)).await
            .map_err(AppError::Internal)?;
        self.delete_cache(&command.project_id, &command.auth_user.user_id).await?;
        Ok(())
    }

    async fn get_by_user(&self, command: ProjectListByUserCommand) -> Result<Vec<ProjectResponse>, AppError> {
        let (limit, offset, _search) = extract_pagination_data(command.pagination);

        if let Some(redis_pool) = &self.redis_pool {
            let cache: Option<Vec<ProjectResponse>> = get_key(
                &redis_pool, self.form_redis_key_list_by_user(&command.user_id).as_str()
            ).await.map_err(AppError::Internal)?;

            if let Some(projects) = cache { return Ok(projects); }
        }

        let projects = self.project_repo.get_by_user(
            command.user_id, limit, offset
        ).await.map_err(AppError::Internal)?;

        let response: Vec<ProjectResponse> = projects.into_iter().map(ProjectResponse::from).collect();
        self.cache_projects_by_user(&command.user_id, &response).await?;

        Ok(response)
    }
}

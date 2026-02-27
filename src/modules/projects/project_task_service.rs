use anyhow::Result;
use async_trait::async_trait;
use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use uuid::Uuid;

use crate::modules::projects::{
    project_task_command::*,
    project_task_dto::*,
    project_task_model::ProjectTask,
    project_task_repo::{ProjectTaskRepository, ProjectTaskRepositoryInterface}
};
use crate::shared::{
    db::redis::{delete_key, get_key, set_key},
    errors::AppError,
    state::AppState,
    utils::extract_pagination_data
};

#[async_trait]
pub trait ProjectTaskInterface {

    async fn get(&self, command: ProjectTaskGetCommand) -> Result<Option<ProjectTaskResponse>, AppError>;

    async fn create(&self, command: ProjectTaskCreateCommand) -> Result<ProjectTaskResponse, AppError>;

    async fn update(&self, command: ProjectTaskUpdateCommand) -> Result<Option<ProjectTaskResponse>, AppError>;

    async fn delete(&self, command: ProjectTaskDeleteCommand) -> Result<(), AppError>;

    async fn get_by_project(&self, command: ProjectTaskListByProjectCommand) -> Result<Vec<ProjectTaskResponse>, AppError>;

}

#[derive(Clone)]
pub struct ProjectTaskService {
    task_repo: ProjectTaskRepository,
    redis_pool: Option<Pool<RedisConnectionManager>>,
}

impl From<&AppState> for ProjectTaskService {
    fn from(app_state: &AppState) -> Self {
        let task_repo = ProjectTaskRepository::from(app_state);
        let redis_pool = app_state.redis_pool.clone();
        Self { task_repo, redis_pool: Option::from(redis_pool) }
    }
}

impl ProjectTaskService {
    fn redis_key_ttl(&self) -> Option<u64> { Some(60 * 60) }

    fn form_redis_key_task(&self, key: &Uuid) -> String { format!("task:{}", key) }

    // Scoped list caching by project!
    fn form_redis_key_list_by_project(&self, project_id: &Uuid) -> String { format!("project:{}:tasks", project_id) }

    async fn cache_task(&self, task: &ProjectTaskResponse) -> Result<(), AppError> {
        if let Some(redis_pool) = &self.redis_pool {
            let _: () = set_key(
                &redis_pool,
                self.form_redis_key_task(&task.task_id).as_str(),
                &task,
                self.redis_key_ttl()
            ).await.map_err(AppError::Internal)?;
        }
        Ok(())
    }

    async fn cache_tasks_by_project(&self, project_id: &Uuid, tasks: &Vec<ProjectTaskResponse>) -> Result<(), AppError> {
        if let Some(redis_pool) = &self.redis_pool {
            let _: () = set_key(
                &redis_pool,
                self.form_redis_key_list_by_project(project_id).as_str(),
                &tasks,
                self.redis_key_ttl()
            ).await.map_err(AppError::Internal)?;
        }
        Ok(())
    }

    async fn get_cache_task(&self, key: &Uuid) -> Result<Option<ProjectTaskResponse>, AppError> {
        if let Some(redis_pool) = &self.redis_pool {
            let task_cache: Option<ProjectTaskResponse> = get_key(
                &redis_pool,
                self.form_redis_key_task(key).as_str()
            ).await.map_err(AppError::Internal)?;
            return Ok(task_cache);
        }
        Ok(None)
    }

    async fn delete_cache(&self, task_id: &Uuid, project_id: &Uuid) -> Result<(), AppError> {
        if let Some(redis_pool) = &self.redis_pool {
            let _: () = delete_key(&redis_pool, self.form_redis_key_task(task_id).as_str()).await
                .map_err(AppError::Internal)?;
            let _: () = delete_key(&redis_pool, self.form_redis_key_list_by_project(project_id).as_str()).await
                .map_err(AppError::Internal)?;
        }
        Ok(())
    }

    async fn handle_res_opt_task(&self, task: anyhow::Result<Option<ProjectTask>>) -> Result<Option<ProjectTaskResponse>, AppError> {
        let task = task.map_err(AppError::Internal)?;

        if let Some(t) = task {
            let response = ProjectTaskResponse::from(t);
            self.cache_task(&response).await?;

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
impl ProjectTaskInterface for ProjectTaskService {
    async fn get(&self, command: ProjectTaskGetCommand) -> Result<Option<ProjectTaskResponse>, AppError> {
        let cache = self.get_cache_task(&command.task_id).await?;
        if let Some(task) = cache {
            return Ok(Some(task));
        }

        let task = self.task_repo.get(command.task_id, Some(command.auth_user.user_id)).await;
        self.handle_res_opt_task(task).await
    }

    async fn create(&self, command: ProjectTaskCreateCommand) -> Result<ProjectTaskResponse, AppError> {
        let meta_user = command.auth_user.user_id.clone();
        let task_create = ProjectTask::from(command);

        // This automatically verifies the project belongs to the user via the repo
        let task = self.task_repo.create(task_create, Some(meta_user)).await
            .map_err(AppError::Internal)?;
        let response = ProjectTaskResponse::from(task);

        self.cache_task(&response).await?;

        // Invalidate list cache for the project
        if let Some(redis_pool) = &self.redis_pool {
            let _: () = delete_key(&redis_pool, self.form_redis_key_list_by_project(&response.project_id).as_str()).await
                .map_err(AppError::Internal)?;
        }

        Ok(response)
    }

    async fn update(&self, command: ProjectTaskUpdateCommand) -> Result<Option<ProjectTaskResponse>, AppError> {
        let task = self.task_repo.update(
            command.task_id, command.title, command.status, command.due_date,
            command.parent_task_id, command.order_idx, command.estimate_minutes,
            command.actual_minutes, command.assigned_person_id, command.location_id, command.note,
            Some(command.auth_user.user_id)
        ).await;

        self.handle_res_opt_task(task).await
    }

    async fn delete(&self, command: ProjectTaskDeleteCommand) -> Result<(), AppError> {
        // Fetch the task first to determine which project cache to invalidate
        let task_opt = self.task_repo.get(command.task_id.clone(), Some(command.auth_user.user_id.clone())).await
            .map_err(AppError::Internal)?;

        if let Some(task) = task_opt {
            self.task_repo.delete(command.task_id.clone(), Some(command.auth_user.user_id)).await
                .map_err(AppError::Internal)?;
            self.delete_cache(&command.task_id, &task.project_id).await?;
        }
        Ok(())
    }

    async fn get_by_project(&self, command: ProjectTaskListByProjectCommand) -> Result<Vec<ProjectTaskResponse>, AppError> {
        let (limit, offset, _search) = extract_pagination_data(command.pagination);

        if let Some(redis_pool) = &self.redis_pool {
            let cache: Option<Vec<ProjectTaskResponse>> = get_key(
                &redis_pool, self.form_redis_key_list_by_project(&command.project_id).as_str()
            ).await.map_err(AppError::Internal)?;

            if let Some(tasks) = cache { return Ok(tasks); }
        }

        let tasks = self.task_repo.get_by_project(
            command.project_id, limit, offset, Some(command.auth_user.user_id)
        ).await.map_err(AppError::Internal)?;

        let response: Vec<ProjectTaskResponse> = tasks.into_iter().map(ProjectTaskResponse::from).collect();
        self.cache_tasks_by_project(&command.project_id, &response).await?;

        Ok(response)
    }
}

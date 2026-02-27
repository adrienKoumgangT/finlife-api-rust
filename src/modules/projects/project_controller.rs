use axum::{extract::{Path, State}, http::StatusCode, routing::{get}, Json, Router};
use uuid::Uuid;

use crate::modules::projects::{
    project_command::*,
    project_dto::*,
    project_service::{ProjectService, ProjectInterface},
    project_task_command::*,
    project_task_dto::*,
    project_task_service::{ProjectTaskService, ProjectTaskInterface},
    project_milestone_command::*,
    project_milestone_dto::*,
    project_milestone_service::{ProjectMilestoneService, ProjectMilestoneInterface},
};

use crate::shared::{
    auth::jwt::AuthUser,
    state::AppState,
    errors::AppError
};

pub fn routes() -> Router<AppState> {
    Router::new()
        // --- Project Routes ---
        .route("/", get(get_projects).post(post_project))
        .route("/{project_id}", get(get_project).put(put_project).delete(delete_project))

        // --- Project Task Routes ---
        .route("/{project_id}/tasks", get(get_project_tasks).post(post_project_task))
        .route("/{project_id}/tasks/{task_id}", get(get_project_task).put(put_project_task).delete(delete_project_task))

        // --- Project Milestone Routes ---
        .route("/{project_id}/milestones", get(get_project_milestones).post(post_project_milestone))
        .route("/{project_id}/milestones/{milestone_id}", get(get_project_milestone).put(put_project_milestone).delete(delete_project_milestone))
}

// ==========================================
//                 PROJECTS
// ==========================================

#[utoipa::path(
    get,
    path = "/api/services/projects",
    responses(
        (status = StatusCode::OK, description = "List of Projects for current user", body = Vec<ProjectResponse>),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "Project"
)]
pub async fn get_projects(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<Json<Vec<ProjectResponse>>, AppError> {
    let command = ProjectListByUserCommand::new(auth_user.user_id.clone(), None, auth_user);
    let project_service = ProjectService::from(&state);

    let projects = project_service.get_by_user(command).await?;
    Ok(Json(projects))
}


#[utoipa::path(
    post,
    path = "/api/services/projects",
    responses(
        (status = StatusCode::CREATED, description = "Project successfully created", body = ProjectResponse),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "Project"
)]
pub async fn post_project(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(create_request): Json<ProjectCreateRequest>
) -> Result<Json<ProjectResponse>, AppError> {
    let command = ProjectCreateCommand::new(create_request, auth_user);
    let project_service = ProjectService::from(&state);

    let project = project_service.create(command).await?;
    Ok(Json(project))
}


#[utoipa::path(
    get,
    path = "/api/services/projects/{project_id}",
    params(
        ("project_id", description = "project identifier in uuid")
    ),
    responses(
        (status = StatusCode::OK, description = "Project found successfully", body = ProjectResponse),
        (status = StatusCode::NOT_FOUND, description = "Project not found"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "Project"
)]
pub async fn get_project(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(project_id): Path<Uuid>,
) -> Result<Json<ProjectResponse>, AppError> {
    let command = ProjectGetCommand::new(project_id, auth_user);
    let project_service = ProjectService::from(&state);

    let project = project_service.get(command).await?
        .ok_or_else(|| AppError::NotFound(format!("Project {} not found", project_id)))?;

    Ok(Json(project))
}


#[utoipa::path(
    put,
    path = "/api/services/projects/{project_id}",
    params(
        ("project_id", description = "project identifier in uuid")
    ),
    responses(
        (status = StatusCode::OK, description = "Project updated successfully", body = ProjectResponse),
        (status = StatusCode::NOT_FOUND, description = "Project not found"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "Project"
)]
pub async fn put_project(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(project_id): Path<Uuid>,
    Json(update_request): Json<ProjectUpdateRequest>
) -> Result<Json<ProjectResponse>, AppError> {
    let command = ProjectUpdateCommand::new(project_id, update_request, auth_user);
    let project_service = ProjectService::from(&state);

    let project = project_service.update(command).await?
        .ok_or_else(|| AppError::NotFound(format!("Project {} not found", project_id)))?;

    Ok(Json(project))
}


#[utoipa::path(
    delete,
    path = "/api/services/projects/{project_id}",
    params(
        ("project_id", description = "project identifier in uuid")
    ),
    responses(
        (status = StatusCode::OK, description = "Project deleted"),
        (status = StatusCode::NOT_FOUND, description = "Project not found"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "Project"
)]
pub async fn delete_project(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(project_id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    let command = ProjectDeleteCommand::new(project_id, auth_user);
    let project_service = ProjectService::from(&state);

    project_service.delete(command).await?;

    Ok(StatusCode::OK)
}


// ==========================================
//              PROJECT TASKS
// ==========================================

#[utoipa::path(
    get,
    path = "/api/services/projects/{project_id}/tasks",
    params(
        ("project_id", description = "project identifier in uuid")
    ),
    responses(
        (status = StatusCode::OK, description = "List of Tasks for project", body = Vec<ProjectTaskResponse>),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "Project"
)]
pub async fn get_project_tasks(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(project_id): Path<Uuid>,
) -> Result<Json<Vec<ProjectTaskResponse>>, AppError> {
    let command = ProjectTaskListByProjectCommand::new(project_id, None, auth_user);
    let task_service = ProjectTaskService::from(&state);

    let tasks = task_service.get_by_project(command).await?;
    Ok(Json(tasks))
}


#[utoipa::path(
    post,
    path = "/api/services/projects/{project_id}/tasks",
    params(
        ("project_id", description = "project identifier in uuid")
    ),
    responses(
        (status = StatusCode::CREATED, description = "Task successfully created", body = ProjectTaskResponse),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "Project"
)]
pub async fn post_project_task(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(project_id): Path<Uuid>,
    Json(mut create_request): Json<ProjectTaskCreateRequest>
) -> Result<Json<ProjectTaskResponse>, AppError> {
    // Override the project_id in the body with the one from the URL path for security/consistency
    create_request.project_id = project_id;

    let command = ProjectTaskCreateCommand::new(create_request, auth_user);
    let task_service = ProjectTaskService::from(&state);

    let task = task_service.create(command).await?;
    Ok(Json(task))
}


#[utoipa::path(
    get,
    path = "/api/services/projects/{project_id}/tasks/{task_id}",
    params(
        ("project_id", description = "project identifier in uuid"),
        ("task_id", description = "task identifier in uuid")
    ),
    responses(
        (status = StatusCode::OK, description = "Task found successfully", body = ProjectTaskResponse),
        (status = StatusCode::NOT_FOUND, description = "Task not found"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "Project"
)]
pub async fn get_project_task(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path((_project_id, task_id)): Path<(Uuid, Uuid)>, // Tuple extraction!
) -> Result<Json<ProjectTaskResponse>, AppError> {
    let command = ProjectTaskGetCommand::new(task_id, auth_user);
    let task_service = ProjectTaskService::from(&state);

    let task = task_service.get(command).await?
        .ok_or_else(|| AppError::NotFound(format!("Task {} not found", task_id)))?;

    Ok(Json(task))
}


#[utoipa::path(
    put,
    path = "/api/services/projects/{project_id}/tasks/{task_id}",
    params(
        ("project_id", description = "project identifier in uuid"),
        ("task_id", description = "task identifier in uuid")
    ),
    responses(
        (status = StatusCode::OK, description = "Task updated successfully", body = ProjectTaskResponse),
        (status = StatusCode::NOT_FOUND, description = "Task not found"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "Project"
)]
pub async fn put_project_task(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path((_project_id, task_id)): Path<(Uuid, Uuid)>,
    Json(update_request): Json<ProjectTaskUpdateRequest>
) -> Result<Json<ProjectTaskResponse>, AppError> {
    let command = ProjectTaskUpdateCommand::new(task_id, update_request, auth_user);
    let task_service = ProjectTaskService::from(&state);

    let task = task_service.update(command).await?
        .ok_or_else(|| AppError::NotFound(format!("Task {} not found", task_id)))?;

    Ok(Json(task))
}


#[utoipa::path(
    delete,
    path = "/api/services/projects/{project_id}/tasks/{task_id}",
    params(
        ("project_id", description = "project identifier in uuid"),
        ("task_id", description = "task identifier in uuid")
    ),
    responses(
        (status = StatusCode::OK, description = "Task deleted"),
        (status = StatusCode::NOT_FOUND, description = "Task not found"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "Project"
)]
pub async fn delete_project_task(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path((_project_id, task_id)): Path<(Uuid, Uuid)>,
) -> Result<StatusCode, AppError> {
    let command = ProjectTaskDeleteCommand::new(task_id, auth_user);
    let task_service = ProjectTaskService::from(&state);

    task_service.delete(command).await?;

    Ok(StatusCode::OK)
}





// ==========================================
//            PROJECT MILESTONES
// ==========================================

#[utoipa::path(
    get,
    path = "/api/services/projects/{project_id}/milestones",
    params(
        ("project_id", description = "project identifier in uuid")
    ),
    responses(
        (status = StatusCode::OK, description = "List of Milestones for project", body = Vec<ProjectMilestoneResponse>),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "Project"
)]
pub async fn get_project_milestones(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(project_id): Path<Uuid>,
) -> Result<Json<Vec<ProjectMilestoneResponse>>, AppError> {
    let command = ProjectMilestoneListByProjectCommand::new(project_id, None, auth_user);
    let milestone_service = ProjectMilestoneService::from(&state);

    let milestones = milestone_service.get_by_project(command).await?;
    Ok(Json(milestones))
}


#[utoipa::path(
    post,
    path = "/api/services/projects/{project_id}/milestones",
    params(
        ("project_id", description = "project identifier in uuid")
    ),
    responses(
        (status = StatusCode::CREATED, description = "Milestone successfully created", body = ProjectMilestoneResponse),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "Project"
)]
pub async fn post_project_milestone(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(project_id): Path<Uuid>,
    Json(mut create_request): Json<ProjectMilestoneCreateRequest>
) -> Result<Json<ProjectMilestoneResponse>, AppError> {
    // Override the project_id in the body with the one from the URL path
    create_request.project_id = project_id;

    let command = ProjectMilestoneCreateCommand::new(create_request, auth_user);
    let milestone_service = ProjectMilestoneService::from(&state);

    let milestone = milestone_service.create(command).await?;
    Ok(Json(milestone))
}


#[utoipa::path(
    get,
    path = "/api/services/projects/{project_id}/milestones/{milestone_id}",
    params(
        ("project_id", description = "project identifier in uuid"),
        ("milestone_id", description = "milestone identifier in uuid")
    ),
    responses(
        (status = StatusCode::OK, description = "Milestone found successfully", body = ProjectMilestoneResponse),
        (status = StatusCode::NOT_FOUND, description = "Milestone not found"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "Project"
)]
pub async fn get_project_milestone(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path((_project_id, milestone_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<ProjectMilestoneResponse>, AppError> {
    let command = ProjectMilestoneGetCommand::new(milestone_id, auth_user);
    let milestone_service = ProjectMilestoneService::from(&state);

    let milestone = milestone_service.get(command).await?
        .ok_or_else(|| AppError::NotFound(format!("Milestone {} not found", milestone_id)))?;

    Ok(Json(milestone))
}


#[utoipa::path(
    put,
    path = "/api/services/projects/{project_id}/milestones/{milestone_id}",
    params(
        ("project_id", description = "project identifier in uuid"),
        ("milestone_id", description = "milestone identifier in uuid")
    ),
    responses(
        (status = StatusCode::OK, description = "Milestone updated successfully", body = ProjectMilestoneResponse),
        (status = StatusCode::NOT_FOUND, description = "Milestone not found"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "Project"
)]
pub async fn put_project_milestone(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path((_project_id, milestone_id)): Path<(Uuid, Uuid)>,
    Json(update_request): Json<ProjectMilestoneUpdateRequest>
) -> Result<Json<ProjectMilestoneResponse>, AppError> {
    let command = ProjectMilestoneUpdateCommand::new(milestone_id, update_request, auth_user);
    let milestone_service = ProjectMilestoneService::from(&state);

    let milestone = milestone_service.update(command).await?
        .ok_or_else(|| AppError::NotFound(format!("Milestone {} not found", milestone_id)))?;

    Ok(Json(milestone))
}


#[utoipa::path(
    delete,
    path = "/api/services/projects/{project_id}/milestones/{milestone_id}",
    params(
        ("project_id", description = "project identifier in uuid"),
        ("milestone_id", description = "milestone identifier in uuid")
    ),
    responses(
        (status = StatusCode::OK, description = "Milestone deleted"),
        (status = StatusCode::NOT_FOUND, description = "Milestone not found"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
    tag = "Project"
)]
pub async fn delete_project_milestone(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path((_project_id, milestone_id)): Path<(Uuid, Uuid)>,
) -> Result<StatusCode, AppError> {
    let command = ProjectMilestoneDeleteCommand::new(milestone_id, auth_user);
    let milestone_service = ProjectMilestoneService::from(&state);

    milestone_service.delete(command).await?;

    Ok(StatusCode::OK)
}

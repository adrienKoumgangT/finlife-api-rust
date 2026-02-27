use axum::{extract::{Path, State}, http::StatusCode, routing::{get, put}, Json, Router};
use axum::extract::Query;
use uuid::Uuid;

use crate::modules::notifications::{
    notification_command::*,
    notification_dto::*,
    notification_service::{NotificationService, NotificationInterface},
};
use crate::modules::users::user::user_model::UserRole;
use crate::shared::{auth::jwt::AuthUser, state::AppState, errors::AppError};

pub fn routes() -> Router<AppState> {
    Router::new()

        // --- Types ---
        .route("/types", get(list_notification_types).post(create_notification_type))
        .route("/types/{type_id}", get(get_notification_type).put(update_notification_type))

        // --- Preferences ---
        .route("/preferences", get(get_preferences).put(put_preference))

        // --- Notifications ---
        .route("/", get(get_notifications))
        .route("/{notification_id}", get(get_notification))
        .route("/{notification_id}/read", put(mark_as_read))
        .route("/{notification_id}/archive", put(archive_notification))
}


// ==========================================
//                  TYPES
// ==========================================


#[utoipa::path(
    get, path = "/api/services/notification/types",
    params(
        ("active_only" = Option<bool>, Query, description = "Filter by active status")
    ),
    responses((status = StatusCode::OK, description = "List Notification Types", body = Vec<NotificationTypeResponse>)),
    tag = "Notification Type"
)]
pub async fn list_notification_types(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Query(filter): Query<ListFilter>
) -> Result<Json<Vec<NotificationTypeResponse>>, AppError> {
    let command = NotificationTypeListCommand::new(filter.active_only.unwrap_or(true), auth_user);
    let service = NotificationService::from(&state);

    let types = service.list_type(command).await?;
    
    Ok(Json(types))
}

#[utoipa::path(
    post, path = "/api/services/notification/types",
    responses((status = StatusCode::CREATED, description = "Create Type (Admin lonly)", body = NotificationTypeResponse)),
    tag = "Notification Type"
)]
pub async fn create_notification_type(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(req): Json<NotificationTypeCreateRequest>
) -> Result<Json<NotificationTypeResponse>, AppError> {
    if auth_user.role != UserRole::ADMIN {
        return Err(AppError::Unauthorized("You're not authorized".to_string()));
    }
    
    let command = NotificationTypeCreateCommand::new(req, auth_user);
    let service = NotificationService::from(&state);

    let nt = service.create_type(command).await?;
    
    Ok(Json(nt))
}

#[utoipa::path(
    get, path = "/api/services/notification/types/{type_id}",
    params(("type_id", description = "type id in uuid")),
    responses((status = StatusCode::OK, description = "Get Type", body = NotificationTypeResponse)),
    tag = "Notification Type"
)]
pub async fn get_notification_type(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(type_id): Path<Uuid>
) -> Result<Json<NotificationTypeResponse>, AppError> {
    let command = NotificationTypeGetCommand::new(type_id, auth_user);
    let service = NotificationService::from(&state);

    let nt = service.get_type(command).await?
        .ok_or_else(|| AppError::NotFound(format!("Notification Type {} not found", type_id)))?;

    Ok(Json(nt))
}

#[utoipa::path(
    put, path = "/api/services/notification/types/{type_id}",
    params(("type_id", description = "type id in uuid")),
    responses((status = StatusCode::OK, description = "Update Type (Admin lonly)", body = NotificationTypeResponse)),
    tag = "Notification Type"
)]
pub async fn update_notification_type(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(type_id): Path<Uuid>,
    Json(req): Json<NotificationTypeUpdateRequest>
) -> Result<Json<NotificationTypeResponse>, AppError> {
    if auth_user.role != UserRole::ADMIN {
        return Err(AppError::Unauthorized("You're not authorized".to_string()));
    }
    
    let command = NotificationTypeUpdateCommand::new(type_id, req, auth_user);
    let service = NotificationService::from(&state);

    let nt = service.update_type(command).await?
        .ok_or_else(|| AppError::NotFound(format!("Notification Type {} not found", type_id)))?;

    Ok(Json(nt))
}


// ==========================================
//             NOTIFICATIONS
// ==========================================

#[utoipa::path(
    get, path = "/api/services/notifications",
    responses((status = StatusCode::OK, description = "List Notifications", body = Vec<NotificationResponse>)),
    tag = "Notification"
)]
pub async fn get_notifications(
    State(state): State<AppState>, 
    auth_user: AuthUser
) -> Result<Json<Vec<NotificationResponse>>, AppError> {
    let command = NotificationListCommand::new(None, auth_user);
    let service = NotificationService::from(&state);

    let notifications = service.get_all(command).await?;
    
    Ok(Json(notifications))
}

#[utoipa::path(
    get, path = "/api/services/notifications/{notification_id}",
    params(("notification_id", description = "notification id in uuid")),
    responses((status = StatusCode::OK, description = "Get Notification", body = NotificationResponse)),
    tag = "Notification"
)]
pub async fn get_notification(
    State(state): State<AppState>, 
    auth_user: AuthUser, 
    Path(notification_id): Path<Uuid>
) -> Result<Json<NotificationResponse>, AppError> {
    let command = NotificationGetCommand::new(notification_id, auth_user);
    let service = NotificationService::from(&state);

    let notif = service.get(command).await?
        .ok_or_else(|| AppError::NotFound(format!("Notification {} not found", notification_id)))?;

    Ok(Json(notif))
}

#[utoipa::path(
    put, path = "/api/services/notifications/{notification_id}/read",
    params(("notification_id", description = "notification id in uuid")),
    responses((status = StatusCode::OK, description = "Mark as read", body = NotificationResponse)),
    tag = "Notification"
)]
pub async fn mark_as_read(
    State(state): State<AppState>, 
    auth_user: AuthUser, 
    Path(notification_id): Path<Uuid>
) -> Result<Json<NotificationResponse>, AppError> {
    let command = NotificationMarkReadCommand::new(notification_id, auth_user);
    let service = NotificationService::from(&state);

    let notif = service.mark_read(command).await?
        .ok_or_else(|| AppError::NotFound(format!("Notification {} not found", notification_id)))?;

    Ok(Json(notif))
}

#[utoipa::path(
    put, path = "/api/services/notifications/{notification_id}/archive",
    params(("notification_id", description = "notification id in uuid")),
    responses((status = StatusCode::OK, description = "Archive notification")),
    tag = "Notification"
)]
pub async fn archive_notification(
    State(state): State<AppState>, 
    auth_user: AuthUser, 
    Path(notification_id): Path<Uuid>
) -> Result<StatusCode, AppError> {
    let command = NotificationArchiveCommand::new(notification_id, auth_user);
    let service = NotificationService::from(&state);

    service.archive(command).await?;
    
    Ok(StatusCode::OK)
}

// ==========================================
//             PREFERENCES
// ==========================================

#[utoipa::path(
    get, path = "/api/services/notifications/preferences",
    responses((status = StatusCode::OK, description = "Get User Preferences", body = Vec<NotificationPreferenceResponse>)),
    tag = "Notification"
)]
pub async fn get_preferences(
    State(state): State<AppState>, 
    auth_user: AuthUser
) -> Result<Json<Vec<NotificationPreferenceResponse>>, AppError> {
    let service = NotificationService::from(&state);
    
    let prefs = service.get_preferences(auth_user.user_id).await?;
    
    Ok(Json(prefs))
}

#[utoipa::path(
    put, path = "/api/services/notifications/preferences",
    responses((status = StatusCode::OK, description = "Update User Preference")),
    tag = "Notification"
)]
pub async fn put_preference(
    State(state): State<AppState>, 
    auth_user: AuthUser, 
    Json(request): Json<PreferenceUpdateRequest>
) -> Result<StatusCode, AppError> {
    let command = PreferenceUpdateCommand::new(request, auth_user);
    let service = NotificationService::from(&state);

    service.update_preference(command).await?;
    
    Ok(StatusCode::OK)
}

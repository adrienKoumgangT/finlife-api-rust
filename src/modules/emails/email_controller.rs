use axum::{extract::{Path, State}, routing::{get, put}, Json, Router};
use uuid::Uuid;

use crate::modules::emails::{
    email_command::*,
    email_dto::*,
    email_service::{EmailService, EmailInterface},
};
use crate::shared::{auth::jwt::AuthUser, state::AppState, errors::AppError};


pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/templates", get(get_templates).post(create_template))
        .route("/templates/{template_id}", put(update_template))
        
        .route("/messages", get(get_messages).post(create_message))
        .route("/messages/{message_id}", get(get_message))
        .route("/messages/{message_id}/status", put(update_message_status))
        .route("/messages/{message_id}/events", get(get_events).post(create_event))
}

// --- Templates ---
#[utoipa::path(
    get, 
    path = "/api/services/emails/templates",
    responses(
        (status = StatusCode::OK, description = "List Templates", body = Vec<EmailTemplateResponse>)
    ),
    tag = "Emails"
)]
pub async fn get_templates(
    State(state): State<AppState>, 
    auth: AuthUser
) -> Result<Json<Vec<EmailTemplateResponse>>, AppError> {
    if !auth.role.is_admin() {
        return Err(AppError::Unauthorized("You are not an admin".to_string()))
    }
    
    let service = EmailService::from(&state);
    
    let templates = service.get_templates().await?;
    
    Ok(Json(templates))
}

#[utoipa::path(
    post, 
    path = "/api/services/emails/templates",
    responses(
        (status = StatusCode::CREATED, description = "Create Template", body = EmailTemplateResponse)
    ),
    tag = "Emails"
)]
pub async fn create_template(
    State(state): State<AppState>,
    auth: AuthUser,
    Json(req): Json<EmailTemplateCreateRequest>
) -> Result<Json<EmailTemplateResponse>, AppError> {
    if !auth.role.is_admin() {
        return Err(AppError::Unauthorized("You are not an admin".to_string()))
    }
    
    let command = EmailTemplateCreateCommand::new(req, auth);
    let service = EmailService::from(&state);
    
    let template = service.create_template(command).await?;
    
    Ok(Json(template))
}

#[utoipa::path(
    put,
    path = "/api/services/emails/templates/{template_id}",
    params(
        ("template_id", description = "Template UUID")
    ),
    responses(
        (status = StatusCode::OK, description = "Update Template", body = EmailTemplateResponse)
    ),
    tag = "Emails"
)]
pub async fn update_template(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(template_id): Path<Uuid>,
    Json(req): Json<EmailTemplateUpdateRequest>
) -> Result<Json<EmailTemplateResponse>, AppError> {
    if !auth.role.is_admin() {
        return Err(AppError::Unauthorized("You are not an admin".to_string()))
    }
    
    let command = EmailTemplateUpdateCommand::new(template_id, req, auth);
    let service = EmailService::from(&state);
    
    let tpl = service.update_template(command).await?
        .ok_or_else(|| AppError::NotFound(format!("Template {} not found", template_id)))?;
    
    Ok(Json(tpl))
}

// --- Messages ---
#[utoipa::path(
    get, 
    path = "/api/services/emails/messages",
    responses(
        (status = StatusCode::OK, description = "List Messages", body = Vec<EmailMessageResponse>)
    ),
    tag = "Emails"
)]
pub async fn get_messages(
    State(state): State<AppState>, 
    auth: AuthUser
) -> Result<Json<Vec<EmailMessageResponse>>, AppError> {
    let command = ListMessagesByUserCommand { user_id: auth.user_id, pagination: None, auth_user: auth };
    let service = EmailService::from(&state);
    
    let messages = service.get_messages_by_user(command).await?;
    
    Ok(Json(messages))
}

#[utoipa::path(
    post,
    path = "/api/services/emails/messages",
    responses(
        (status = StatusCode::CREATED, description = "Create Message", body = EmailMessageResponse)
    ),
    tag = "Emails"
)]
pub async fn create_message(
    State(state): State<AppState>,
    auth: AuthUser,
    Json(req): Json<EmailMessageCreateRequest>
) -> Result<Json<EmailMessageResponse>, AppError> {
    let command = EmailMessageCreateCommand::new(req, auth);
    let service = EmailService::from(&state);
    
    let message = service.create_message(command).await?;
    
    Ok(Json(message))
}

#[utoipa::path(
    get, 
    path = "/api/services/emails/messages/{message_id}",
    params(
        ("message_id", description = "Message UUID")
    ),
    responses(
        (status = StatusCode::OK, description = "Get Message", body = EmailMessageResponse)
    ),
    tag = "Emails"
)]
pub async fn get_message(
    State(state): State<AppState>, 
    auth: AuthUser, 
    Path(message_id): Path<Uuid>
) -> Result<Json<EmailMessageResponse>, AppError> {
    let command = GetMessageCommand { message_id, auth_user: auth };
    let service = EmailService::from(&state);
    
    let message = service.get_message(command).await?
        .ok_or_else(|| AppError::NotFound(format!("Message {} not found", message_id)))?;
    
    Ok(Json(message))
}

#[utoipa::path(
    put, 
    path = "/api/services/emails/messages/{message_id}/status",
    params(
        ("message_id", description = "Message UUID")
    ),
    responses(
        (status = StatusCode::OK, description = "Update Message Status", body = EmailMessageResponse)
    ),
    tag = "Emails"
)]
pub async fn update_message_status(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(message_id): Path<Uuid>,
    Json(req): Json<EmailMessageUpdateStatusRequest>
) -> Result<Json<EmailMessageResponse>, AppError> {
    let command = EmailMessageUpdateStatusCommand::new(message_id, req, auth);
    let service = EmailService::from(&state);
    
    let msg = service.update_message_status(command).await?
        .ok_or_else(|| AppError::NotFound(format!("Message {} not found", message_id)))?;
    
    Ok(Json(msg))
}

// --- Events ---
#[utoipa::path(
    get, 
    path = "/api/services/emails/messages/{message_id}/events",
    params(
        ("message_id", description = "Message UUID")
    ),
    responses(
        (status = StatusCode::OK, description = "List Events", body = Vec<EmailEventResponse>)
    ),
    tag = "Emails"
)]
pub async fn get_events(
    State(state): State<AppState>, 
    _auth: AuthUser, 
    Path(message_id): Path<Uuid>
) -> Result<Json<Vec<EmailEventResponse>>, AppError> {
    let service = EmailService::from(&state);
    
    let events = service.get_events(message_id).await?;
    
    Ok(Json(events))
}

#[utoipa::path(
    post,
    path = "/api/services/emails/messages/{message_id}/events",
    params(
        ("message_id", description = "Message UUID")
    ),
    responses(
        (status = StatusCode::CREATED, description = "Create Event", body = EmailEventResponse)
    ),
    tag = "Emails"
)]
pub async fn create_event(
    State(state): State<AppState>, 
    auth: AuthUser, 
    Path(message_id): Path<Uuid>, 
    Json(mut req): Json<EmailEventCreateRequest>
) -> Result<Json<EmailEventResponse>, AppError> {
    req.email_message_id = message_id;
    
    let command = EmailEventCreateCommand::new(req, auth);
    let service = EmailService::from(&state);
    
    let event = service.create_event(command).await?;
    
    Ok(Json(event))
}

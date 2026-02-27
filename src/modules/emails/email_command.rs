use chrono::NaiveDateTime;
use uuid::Uuid;
use serde::{Serialize, Deserialize};

use crate::modules::emails::email_dto::*;
use crate::modules::emails::email_model::{EmailStatus, EmailPriority, EmailEventType};
use crate::shared::auth::jwt::AuthUser;
use crate::shared::response::PaginationRequest;


// --- TEMPLATES ---

#[derive(Debug, Serialize, Deserialize)]
pub struct EmailTemplateCreateCommand {
    pub code: String,
    pub locale: Option<String>,
    pub subject_tpl: String,
    pub body_text_tpl: Option<String>,
    pub body_html_tpl: Option<String>,
    pub description: Option<String>,
    pub variables: Option<serde_json::Value>,
    pub is_active: Option<bool>,
    pub auth_user: AuthUser,
}

impl EmailTemplateCreateCommand {
    pub fn new(req: EmailTemplateCreateRequest, auth_user: AuthUser) -> Self {
        Self {
            code: req.code, locale: req.locale, subject_tpl: req.subject_tpl,
            body_text_tpl: req.body_text_tpl, body_html_tpl: req.body_html_tpl,
            description: req.description, variables: req.variables, is_active: req.is_active,
            auth_user,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmailTemplateUpdateCommand {
    pub template_id: Uuid,
    pub subject_tpl: String,
    pub body_text_tpl: Option<String>,
    pub body_html_tpl: Option<String>,
    pub description: Option<String>,
    pub variables: Option<serde_json::Value>,
    pub is_active: bool,
    pub auth_user: AuthUser,
}

impl EmailTemplateUpdateCommand {
    pub fn new(template_id: Uuid, req: EmailTemplateUpdateRequest, auth_user: AuthUser) -> Self {
        Self {
            template_id,
            subject_tpl: req.subject_tpl,
            body_text_tpl: req.body_text_tpl,
            body_html_tpl: req.body_html_tpl,
            description: req.description,
            variables: req.variables,
            is_active: req.is_active,
            auth_user,
        }
    }
}



// --- MESSAGES ---


#[derive(Debug, Serialize, Deserialize)]
pub struct EmailMessageCreateCommand {
    pub user_id: Option<Uuid>,
    pub template_id: Option<Uuid>,
    pub to_email: String,
    pub to_name: Option<String>,
    pub from_email: Option<String>,
    pub from_name: Option<String>,
    pub reply_to: Option<String>,
    pub template_vars: Option<serde_json::Value>,
    pub subject: String,
    pub body_text: Option<String>,
    pub body_html: Option<String>,
    pub status: Option<EmailStatus>,
    pub priority: Option<EmailPriority>,
    pub max_attempts: Option<i32>,
    pub scheduled_at: Option<NaiveDateTime>,
    pub auth_user: AuthUser,
}

impl EmailMessageCreateCommand {
    pub fn new(req: EmailMessageCreateRequest, auth_user: AuthUser) -> Self {
        Self {
            user_id: req.user_id, template_id: req.template_id, to_email: req.to_email,
            to_name: req.to_name, from_email: req.from_email, from_name: req.from_name,
            reply_to: req.reply_to, template_vars: req.template_vars, subject: req.subject,
            body_text: req.body_text, body_html: req.body_html, status: req.status,
            priority: req.priority, max_attempts: req.max_attempts, scheduled_at: req.scheduled_at,
            auth_user,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmailMessageUpdateStatusCommand {
    pub message_id: Uuid,
    pub status: EmailStatus,
    pub attempt_count: i32,
    pub next_attempt_at: Option<NaiveDateTime>,
    pub last_error: Option<String>,
    pub provider: Option<String>,
    pub provider_msg_id: Option<String>,
    pub sent_at: Option<NaiveDateTime>,
    pub auth_user: AuthUser,
}

impl EmailMessageUpdateStatusCommand {
    pub fn new(message_id: Uuid, req: EmailMessageUpdateStatusRequest, auth_user: AuthUser) -> Self {
        Self {
            message_id,
            status: req.status,
            attempt_count: req.attempt_count,
            next_attempt_at: req.next_attempt_at,
            last_error: req.last_error,
            provider: req.provider,
            provider_msg_id: req.provider_msg_id,
            sent_at: req.sent_at,
            auth_user,
        }
    }
}



// --- EVENTS ---

#[derive(Debug, Serialize, Deserialize)]
pub struct EmailEventCreateCommand {
    pub email_message_id: Uuid,
    pub event_type: EmailEventType,
    pub event_at: Option<NaiveDateTime>,
    pub meta: Option<serde_json::Value>,
    pub auth_user: AuthUser,
}

impl EmailEventCreateCommand {
    pub fn new(req: EmailEventCreateRequest, auth_user: AuthUser) -> Self {
        Self {
            email_message_id: req.email_message_id, event_type: req.event_type,
            event_at: req.event_at, meta: req.meta, auth_user,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetMessageCommand { pub message_id: Uuid, pub auth_user: AuthUser }

#[derive(Debug, Serialize, Deserialize)]
pub struct ListMessagesByUserCommand {
    pub user_id: Uuid,
    pub pagination: Option<PaginationRequest>,
    pub auth_user: AuthUser
}

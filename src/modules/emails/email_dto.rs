use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::modules::emails::email_model::{EmailTemplate, EmailMessage, EmailEvent, EmailStatus, EmailPriority, EmailEventType};


#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct EmailTemplateResponse {
    pub template_id: Uuid,
    pub code: String,
    pub locale: String,
    pub subject_tpl: String,
    pub body_text_tpl: Option<String>,
    pub body_html_tpl: Option<String>,
    pub description: Option<String>,
    pub variables: Option<serde_json::Value>,
    pub is_active: bool,
}

impl From<EmailTemplate> for EmailTemplateResponse {
    fn from(tpl: EmailTemplate) -> Self {
        Self {
            template_id: tpl.id.unwrap(),
            code: tpl.code, locale: tpl.locale, subject_tpl: tpl.subject_tpl,
            body_text_tpl: tpl.body_text_tpl, body_html_tpl: tpl.body_html_tpl,
            description: tpl.description, variables: tpl.variables, is_active: tpl.is_active,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct EmailMessageResponse {
    pub message_id: Uuid,
    pub user_id: Option<Uuid>,
    pub template_id: Option<Uuid>,
    pub to_email: String,
    pub subject: String,
    pub status: EmailStatus,
    pub priority: EmailPriority,
    pub attempt_count: i32,
    pub sent_at: Option<NaiveDateTime>,
    pub created_at: Option<DateTime<Utc>>,
}

impl From<EmailMessage> for EmailMessageResponse {
    fn from(msg: EmailMessage) -> Self {
        Self {
            message_id: msg.id.unwrap(), user_id: msg.user_id, template_id: msg.template_id,
            to_email: msg.to_email, subject: msg.subject, status: msg.status,
            priority: msg.priority, attempt_count: msg.attempt_count,
            sent_at: msg.sent_at, created_at: msg.created_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct EmailEventResponse {
    pub event_id: Uuid,
    pub email_message_id: Uuid,
    pub event_type: EmailEventType,
    pub event_at: NaiveDateTime,
    pub meta: Option<serde_json::Value>,
}

impl From<EmailEvent> for EmailEventResponse {
    fn from(evt: EmailEvent) -> Self {
        Self {
            event_id: evt.id.unwrap(), email_message_id: evt.email_message_id,
            event_type: evt.event_type, event_at: evt.event_at, meta: evt.meta,
        }
    }
}

// --- Requests ---

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct EmailTemplateCreateRequest {
    pub code: String,
    pub locale: Option<String>,
    pub subject_tpl: String,
    pub body_text_tpl: Option<String>,
    pub body_html_tpl: Option<String>,
    pub description: Option<String>,
    pub variables: Option<serde_json::Value>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct EmailTemplateUpdateRequest {
    pub subject_tpl: String,
    pub body_text_tpl: Option<String>,
    pub body_html_tpl: Option<String>,
    pub description: Option<String>,
    pub variables: Option<serde_json::Value>,
    pub is_active: bool,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct EmailMessageCreateRequest {
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
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct EmailMessageUpdateStatusRequest {
    pub status: EmailStatus,
    pub attempt_count: i32,
    pub next_attempt_at: Option<NaiveDateTime>,
    pub last_error: Option<String>,
    pub provider: Option<String>,
    pub provider_msg_id: Option<String>,
    pub sent_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct EmailEventCreateRequest {
    pub email_message_id: Uuid,
    pub event_type: EmailEventType,
    pub event_at: Option<NaiveDateTime>,
    pub meta: Option<serde_json::Value>,
}



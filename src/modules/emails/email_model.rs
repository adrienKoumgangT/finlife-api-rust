use chrono::{DateTime, NaiveDateTime, Utc};
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::modules::emails::email_command::{EmailTemplateCreateCommand, EmailMessageCreateCommand, EmailEventCreateCommand};

// --- ENUMS ---

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EmailStatus {
    Queued, Sending, Sent, Failed, Cancelled,
}

impl From<String> for EmailStatus {
    fn from(value: String) -> Self {
        match value.to_uppercase().as_str() {
            "SENDING" => EmailStatus::Sending,
            "SENT" => EmailStatus::Sent,
            "FAILED" => EmailStatus::Failed,
            "CANCELLED" => EmailStatus::Cancelled,
            _ => EmailStatus::Queued,
        }
    }
}

impl EmailStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            EmailStatus::Queued => "QUEUED",
            EmailStatus::Sending => "SENDING",
            EmailStatus::Sent => "SENT",
            EmailStatus::Failed => "FAILED",
            EmailStatus::Cancelled => "CANCELLED",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EmailPriority {
    Low, Normal, High,
}

impl From<String> for EmailPriority {
    fn from(value: String) -> Self {
        match value.to_uppercase().as_str() {
            "LOW" => EmailPriority::Low,
            "HIGH" => EmailPriority::High,
            _ => EmailPriority::Normal,
        }
    }
}

impl EmailPriority {
    pub fn as_str(&self) -> &'static str {
        match self {
            EmailPriority::Low => "LOW",
            EmailPriority::Normal => "NORMAL",
            EmailPriority::High => "HIGH",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EmailEventType {
    Queued, Sending, Sent, Delivered, Bounced, Dropped, Opened, Clicked, Failed,
}

impl From<String> for EmailEventType {
    fn from(value: String) -> Self {
        match value.to_uppercase().as_str() {
            "SENDING" => EmailEventType::Sending,
            "SENT" => EmailEventType::Sent,
            "DELIVERED" => EmailEventType::Delivered,
            "BOUNCED" => EmailEventType::Bounced,
            "DROPPED" => EmailEventType::Dropped,
            "OPENED" => EmailEventType::Opened,
            "CLICKED" => EmailEventType::Clicked,
            "FAILED" => EmailEventType::Failed,
            _ => EmailEventType::Queued,
        }
    }
}

impl EmailEventType {
    pub fn as_str(&self) -> &'static str {
        match self {
            EmailEventType::Queued => "QUEUED",
            EmailEventType::Sending => "SENDING",
            EmailEventType::Sent => "SENT",
            EmailEventType::Delivered => "DELIVERED",
            EmailEventType::Bounced => "BOUNCED",
            EmailEventType::Dropped => "DROPPED",
            EmailEventType::Opened => "OPENED",
            EmailEventType::Clicked => "CLICKED",
            EmailEventType::Failed => "FAILED",
        }
    }
}

// --- MODELS ---

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct EmailTemplate {
    pub id: Option<Uuid>,
    pub code: String,
    pub locale: String,
    pub subject_tpl: String,
    pub body_text_tpl: Option<String>,
    pub body_html_tpl: Option<String>,
    pub description: Option<String>,
    pub variables: Option<serde_json::Value>,
    pub is_active: bool,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl From<EmailTemplateCreateCommand> for EmailTemplate {
    fn from(cmd: EmailTemplateCreateCommand) -> Self {
        Self {
            id: None,
            code: cmd.code,
            locale: cmd.locale.unwrap_or_else(|| "fr".to_string()),
            subject_tpl: cmd.subject_tpl,
            body_text_tpl: cmd.body_text_tpl,
            body_html_tpl: cmd.body_html_tpl,
            description: cmd.description,
            variables: cmd.variables,
            is_active: cmd.is_active.unwrap_or(true),
            created_at: None, updated_at: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct EmailMessage {
    pub id: Option<Uuid>,
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
    pub status: EmailStatus,
    pub priority: EmailPriority,
    pub provider: Option<String>,
    pub provider_msg_id: Option<String>,
    pub attempt_count: i32,
    pub max_attempts: i32,
    pub next_attempt_at: Option<NaiveDateTime>,
    pub last_error: Option<String>,
    pub scheduled_at: Option<NaiveDateTime>,
    pub sent_at: Option<NaiveDateTime>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl From<EmailMessageCreateCommand> for EmailMessage {
    fn from(cmd: EmailMessageCreateCommand) -> Self {
        Self {
            id: None,
            user_id: cmd.user_id,
            template_id: cmd.template_id,
            to_email: cmd.to_email,
            to_name: cmd.to_name,
            from_email: cmd.from_email,
            from_name: cmd.from_name,
            reply_to: cmd.reply_to,
            template_vars: cmd.template_vars,
            subject: cmd.subject,
            body_text: cmd.body_text,
            body_html: cmd.body_html,
            status: cmd.status.unwrap_or(EmailStatus::Queued),
            priority: cmd.priority.unwrap_or(EmailPriority::Normal),
            provider: None,
            provider_msg_id: None,
            attempt_count: 0,
            max_attempts: cmd.max_attempts.unwrap_or(5),
            next_attempt_at: cmd.scheduled_at,
            last_error: None,
            scheduled_at: cmd.scheduled_at,
            sent_at: None,
            created_at: None, updated_at: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct EmailEvent {
    pub id: Option<Uuid>,
    pub email_message_id: Uuid,
    pub event_type: EmailEventType,
    pub event_at: NaiveDateTime,
    pub meta: Option<serde_json::Value>,
    pub created_at: Option<DateTime<Utc>>,
}

impl From<EmailEventCreateCommand> for EmailEvent {
    fn from(cmd: EmailEventCreateCommand) -> Self {
        Self {
            id: None,
            email_message_id: cmd.email_message_id,
            event_type: cmd.event_type,
            event_at: cmd.event_at.unwrap_or_else(|| Utc::now().naive_utc()),
            meta: cmd.meta,
            created_at: None,
        }
    }
}

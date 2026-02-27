use chrono::{DateTime, NaiveDateTime, Utc};
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::modules::notifications::notification_command::NotificationTypeCreateCommand;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum NotificationSeverity {
    Info,
    Success,
    Warning,
    Error,
    Security,
}


impl From<String> for NotificationSeverity {
    fn from(value: String) -> Self {
        match value.to_uppercase().as_str() {
            "SUCCESS" => NotificationSeverity::Success,
            "WARNING" => NotificationSeverity::Warning,
            "ERROR" => NotificationSeverity::Error,
            "SECURITY" => NotificationSeverity::Security,
            _ => NotificationSeverity::Info,
        }
    }
}

impl NotificationSeverity {
    pub fn as_str(&self) -> &'static str {
        match self {
            NotificationSeverity::Info => "INFO",
            NotificationSeverity::Success => "SUCCESS",
            NotificationSeverity::Warning => "WARNING",
            NotificationSeverity::Error => "ERROR",
            NotificationSeverity::Security => "SECURITY",
        }
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum NotificationChannel {
    InApp,
    Email,
}

impl From<String> for NotificationChannel {
    fn from(value: String) -> Self {
        match value.to_uppercase().as_str() {
            "EMAIL" => NotificationChannel::Email,
            _ => NotificationChannel::InApp,
        }
    }
}

impl NotificationChannel {
    pub fn as_str(&self) -> &'static str {
        match self {
            NotificationChannel::InApp => "IN_APP",
            NotificationChannel::Email => "EMAIL",
        }
    }
}

// Models

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct NotificationType {
    pub id: Option<Uuid>,
    pub code: String,
    pub name: String,
    pub severity: NotificationSeverity,

    pub title_template: Option<String>,
    pub body_template: Option<String>,

    pub default_in_app: bool,
    pub default_email: bool,
    pub is_active: bool,

    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl From<NotificationTypeCreateCommand> for NotificationType {
    fn from(command: NotificationTypeCreateCommand) -> Self {
        Self {
            id: None,
            code: command.code,
            name: command.name,
            severity: command.severity.unwrap_or(NotificationSeverity::Info),
            title_template: command.title_template,
            body_template: command.body_template,
            default_in_app: command.default_in_app.unwrap_or(true),
            default_email: command.default_email.unwrap_or(false),
            is_active: command.is_active.unwrap_or(true),
            created_at: None,
            updated_at: None,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Notification {
    pub id: Option<Uuid>,
    pub user_id: Uuid,
    pub type_id: Uuid,

    pub title: String,
    pub body: Option<String>,
    pub data: Option<serde_json::Value>, // Natively handles JSON!

    pub entity_type: Option<String>,
    pub entity_id: Option<Uuid>,
    pub link_url: Option<String>,

    pub is_read: bool,
    pub read_at: Option<NaiveDateTime>,
    pub archived: bool,

    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct NotificationPreference {
    pub user_id: Uuid,
    pub type_id: Uuid,
    pub channel: NotificationChannel,
    pub enabled: bool,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

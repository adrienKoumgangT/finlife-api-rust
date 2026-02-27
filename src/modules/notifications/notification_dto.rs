use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::modules::notifications::notification_model::{NotificationType, Notification, NotificationPreference, NotificationChannel, NotificationSeverity};


#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ListFilter {
    pub active_only: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct NotificationTypeResponse {
    pub type_id: Uuid,
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

impl From<NotificationType> for NotificationTypeResponse {
    fn from(nt: NotificationType) -> Self {
        Self {
            type_id: nt.id.unwrap(),
            code: nt.code,
            name: nt.name,
            severity: nt.severity,
            title_template: nt.title_template,
            body_template: nt.body_template,
            default_in_app: nt.default_in_app,
            default_email: nt.default_email,
            is_active: nt.is_active,
            created_at: nt.created_at,
            updated_at: nt.updated_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct NotificationTypeCreateRequest {
    pub code: String,
    pub name: String,
    pub severity: Option<NotificationSeverity>,
    pub title_template: Option<String>,
    pub body_template: Option<String>,
    pub default_in_app: Option<bool>,
    pub default_email: Option<bool>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct NotificationTypeUpdateRequest {
    pub name: String,
    pub severity: NotificationSeverity,
    pub title_template: Option<String>,
    pub body_template: Option<String>,
    pub default_in_app: bool,
    pub default_email: bool,
    pub is_active: bool,
}




#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct NotificationResponse {
    pub notification_id: Uuid,
    pub user_id: Uuid,
    pub type_id: Uuid,

    pub title: String,
    pub body: Option<String>,
    pub data: Option<serde_json::Value>,

    pub entity_type: Option<String>,
    pub entity_id: Option<Uuid>,
    pub link_url: Option<String>,

    pub is_read: bool,
    pub read_at: Option<NaiveDateTime>,
    pub archived: bool,

    pub created_at: Option<DateTime<Utc>>,
}

impl From<Notification> for NotificationResponse {
    fn from(notif: Notification) -> Self {
        Self {
            notification_id: notif.id.unwrap(),
            user_id: notif.user_id,
            type_id: notif.type_id,
            title: notif.title,
            body: notif.body,
            data: notif.data,
            entity_type: notif.entity_type,
            entity_id: notif.entity_id,
            link_url: notif.link_url,
            is_read: notif.is_read,
            read_at: notif.read_at,
            archived: notif.archived,
            created_at: notif.created_at,
        }
    }
}




#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct NotificationPreferenceResponse {
    pub type_id: Uuid,
    pub channel: NotificationChannel,
    pub enabled: bool,
}

impl From<NotificationPreference> for NotificationPreferenceResponse {
    fn from(pref: NotificationPreference) -> Self {
        Self {
            type_id: pref.type_id,
            channel: pref.channel,
            enabled: pref.enabled,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PreferenceUpdateRequest {
    pub type_id: Uuid,
    pub channel: NotificationChannel,
    pub enabled: bool,
}

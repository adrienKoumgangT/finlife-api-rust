use uuid::Uuid;
use serde::{Serialize, Deserialize};

use crate::modules::notifications::notification_dto::*;
use crate::modules::notifications::notification_model::NotificationSeverity;
use crate::shared::auth::jwt::AuthUser;
use crate::shared::response::PaginationRequest;


#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationTypeGetCommand {
    pub type_id: Uuid,
    pub auth_user: AuthUser,
}

impl NotificationTypeGetCommand {
    pub fn new(type_id: Uuid, auth_user: AuthUser) -> Self {
        Self { type_id, auth_user }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationTypeCreateCommand {
    pub code: String,
    pub name: String,
    pub severity: Option<NotificationSeverity>,
    pub title_template: Option<String>,
    pub body_template: Option<String>,
    pub default_in_app: Option<bool>,
    pub default_email: Option<bool>,
    pub is_active: Option<bool>,
    pub auth_user: AuthUser, // Assumed to be checked for Admin role later
}

impl NotificationTypeCreateCommand {
    pub fn new(request: NotificationTypeCreateRequest, auth_user: AuthUser) -> Self {
        Self {
            code: request.code,
            name: request.name,
            severity: request.severity,
            title_template: request.title_template,
            body_template: request.body_template,
            default_in_app: request.default_in_app,
            default_email: request.default_email,
            is_active: request.is_active,
            auth_user,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationTypeUpdateCommand {
    pub type_id: Uuid,
    pub name: String,
    pub severity: NotificationSeverity,
    pub title_template: Option<String>,
    pub body_template: Option<String>,
    pub default_in_app: bool,
    pub default_email: bool,
    pub is_active: bool,
    pub auth_user: AuthUser,
}

impl NotificationTypeUpdateCommand {
    pub fn new(type_id: Uuid, request: NotificationTypeUpdateRequest, auth_user: AuthUser) -> Self {
        Self {
            type_id,
            name: request.name,
            severity: request.severity,
            title_template: request.title_template,
            body_template: request.body_template,
            default_in_app: request.default_in_app,
            default_email: request.default_email,
            is_active: request.is_active,
            auth_user,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationTypeListCommand {
    pub only_active: bool,
    pub auth_user: AuthUser,
}

impl NotificationTypeListCommand {
    pub fn new(only_active: bool, auth_user: AuthUser) -> Self {
        Self { only_active, auth_user }
    }
}



#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationGetCommand {
    pub notification_id: Uuid,
    pub auth_user: AuthUser,
}

impl NotificationGetCommand {
    pub fn new(notification_id: Uuid, auth_user: AuthUser) -> Self {
        Self { notification_id, auth_user }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationMarkReadCommand {
    pub notification_id: Uuid,
    pub auth_user: AuthUser,
}

impl NotificationMarkReadCommand {
    pub fn new(notification_id: Uuid, auth_user: AuthUser) -> Self {
        Self { notification_id, auth_user }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationArchiveCommand {
    pub notification_id: Uuid,
    pub auth_user: AuthUser,
}

impl NotificationArchiveCommand {
    pub fn new(notification_id: Uuid, auth_user: AuthUser) -> Self {
        Self { notification_id, auth_user }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationListCommand {
    pub pagination: Option<PaginationRequest>,
    pub auth_user: AuthUser,
}

impl NotificationListCommand {
    pub fn new(pagination: Option<PaginationRequest>, auth_user: AuthUser) -> Self {
        Self { pagination, auth_user }
    }
}




#[derive(Debug, Serialize, Deserialize)]
pub struct PreferenceUpdateCommand {
    pub type_id: Uuid,
    pub channel: String,
    pub enabled: bool,
    pub auth_user: AuthUser,
}

impl PreferenceUpdateCommand {
    pub fn new(request: PreferenceUpdateRequest, auth_user: AuthUser) -> Self {
        Self {
            type_id: request.type_id,
            channel: request.channel.as_str().to_string(),
            enabled: request.enabled,
            auth_user,
        }
    }
}

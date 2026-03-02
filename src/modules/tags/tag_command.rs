use uuid::Uuid;
use serde::{Serialize, Deserialize};
use crate::modules::tags::tag_dto::TagCreateRequest;
use crate::shared::auth::jwt::AuthUser;

#[derive(Debug, Serialize, Deserialize)]
pub struct TagGetCommand {
    pub tag_id: Uuid,
    pub auth_user: AuthUser,
}

impl TagGetCommand {
    pub fn new(tag_id: Uuid, auth_user: AuthUser) -> Self {
        Self { tag_id, auth_user }
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct TagCreateCommand {
    pub user_id: Uuid,

    pub name: String,
    pub auth_user: AuthUser,
}

impl TagCreateCommand {
    pub fn new(request: TagCreateRequest, auth_user: AuthUser) -> Self {
        Self {
            user_id: auth_user.user_id,
            name: request.name,
            auth_user
        }
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct TagDeleteCommand {
    pub tag_id: Uuid,

    pub auth_user: AuthUser,
}

impl TagDeleteCommand {
    pub fn new(tag_id: Uuid, auth_user: AuthUser) -> Self {
        Self { tag_id, auth_user }
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct TagListByUserCommand {
    pub user_id: Uuid,

    pub name: Option<String>,

    pub auth_user: AuthUser,
}

impl TagListByUserCommand {
    pub fn new(user_id: Uuid, name: Option<String>, auth_user: AuthUser) -> Self {
        Self { user_id, name, auth_user }
    }
}

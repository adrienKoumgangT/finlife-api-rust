use uuid::Uuid;
use serde::{Serialize, Deserialize};

use crate::modules::people::people_dto::{PeopleCreateRequest, PeopleUpdateArchivedRequest, PeopleUpdateRequest};
use crate::shared::auth::jwt::AuthUser;
use crate::shared::response::PaginationRequest;


#[derive(Debug, Serialize, Deserialize)]
pub struct PeopleGetCommand {
    pub people_id: Uuid,

    pub auth_user: AuthUser,
}

impl PeopleGetCommand {
    pub fn new(people_id: Uuid, auth_user: AuthUser) -> Self {
        Self { people_id, auth_user }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PeopleCreateCommand {
    pub user_id: Uuid,
    pub name: String,

    pub email: Option<String>,
    pub phone: Option<String>,
    pub image_url: Option<String>,
    pub note: Option<String>,

    pub auth_user: AuthUser,
}

impl PeopleCreateCommand {
    pub fn new(request: PeopleCreateRequest, auth_user: AuthUser) -> Self {
        Self {
            user_id: auth_user.user_id,
            name: request.name,
            email: request.email,
            phone: request.phone,
            image_url: request.image_url,
            note: request.note,
            auth_user,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PeopleUpdateImageCommand {
    pub people_id: Uuid,
    pub image_url: Option<String>,

    pub auth_user: AuthUser,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PeopleArchivedCommand {
    pub people_id: Uuid,
    pub archived: bool,

    pub auth_user: AuthUser,
}

impl PeopleArchivedCommand {
    pub fn new(people_id: Uuid, request: PeopleUpdateArchivedRequest, auth_user: AuthUser) -> Self {
        Self {
            people_id,
            archived: request.archived,
            auth_user,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PeopleUpdateCommand {
    pub people_id: Uuid,
    
    pub name: String,

    pub email: Option<String>,
    pub phone: Option<String>,
    pub note: Option<String>,

    pub auth_user: AuthUser,
}

impl PeopleUpdateCommand {
    pub fn new(people_id: Uuid, request: PeopleUpdateRequest, auth_user: AuthUser) -> Self {
        Self {
            people_id,
            name: request.name,
            email: request.email,
            phone: request.phone,
            note: request.note,
            auth_user,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PeopleDeleteCommand {
    pub people_id: Uuid,

    pub auth_user: AuthUser,
}

impl PeopleDeleteCommand {
    pub fn new(people_id: Uuid, auth_user: AuthUser) -> Self {
        Self { people_id, auth_user }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PeopleListCommand {
    pub pagination: Option<PaginationRequest>,

    pub auth_user: AuthUser,
}

impl PeopleListCommand {
    pub fn new(pagination: Option<PaginationRequest>, auth_user: AuthUser) -> Self {
        Self { pagination, auth_user }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PeopleListByUserCommand {
    pub user_id: Uuid,
    pub pagination: Option<PaginationRequest>,

    pub auth_user: AuthUser,
}

impl PeopleListByUserCommand {
    pub fn new(user_id: Uuid, pagination: Option<PaginationRequest>, auth_user: AuthUser) -> Self {
        Self { user_id, pagination, auth_user }
    }
}

use chrono::NaiveDate;
use rust_decimal::Decimal;
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
    pub people_name: String,

    pub people_email: Option<String>,
    pub people_phone: Option<String>,
    pub people_image_url: Option<String>,
    pub people_note: Option<String>,

    pub auth_user: AuthUser,
}

impl PeopleCreateCommand {
    pub fn new(request: PeopleCreateRequest, auth_user: AuthUser) -> Self {
        Self {
            user_id: auth_user.user_id,
            people_name: request.people_name,
            people_email: request.people_email,
            people_phone: request.people_phone,
            people_image_url: request.people_image_url,
            people_note: request.people_note,
            auth_user,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PeopleUpdateImageCommand {
    pub people_id: Uuid,
    pub people_image_url: Option<String>,

    pub auth_user: AuthUser,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PeopleArchivedCommand {
    pub people_id: Uuid,
    pub people_archived: bool,

    pub auth_user: AuthUser,
}

impl PeopleArchivedCommand {
    pub fn new(people_id: Uuid, request: PeopleUpdateArchivedRequest, auth_user: AuthUser) -> Self {
        Self {
            people_id,
            people_archived: request.people_archived,
            auth_user,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PeopleUpdateCommand {
    pub people_id: Uuid,
    
    pub people_name: String,

    pub people_email: Option<String>,
    pub people_phone: Option<String>,
    pub people_note: Option<String>,

    pub auth_user: AuthUser,
}

impl PeopleUpdateCommand {
    pub fn new(people_id: Uuid, request: PeopleUpdateRequest, auth_user: AuthUser) -> Self {
        Self {
            people_id,
            people_name: request.people_name,
            people_email: request.people_email,
            people_phone: request.people_phone,
            people_note: request.people_note,
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

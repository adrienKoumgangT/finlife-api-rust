use uuid::Uuid;
use rust_decimal::Decimal;
use serde::{Serialize, Deserialize};

use crate::modules::locations::location_dto::*;
use crate::shared::auth::jwt::AuthUser;
use crate::shared::response::PaginationRequest;

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationGetCommand {
    pub location_id: Uuid,

    pub auth_user: AuthUser,
}

impl LocationGetCommand {
    pub fn new(location_id: Uuid, auth_user: AuthUser) -> Self {
        Self { location_id, auth_user }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationCreateCommand {
    pub user_id: Uuid,

    pub name: String,

    pub address: Option<String>,
    pub city: Option<String>,
    pub region: Option<String>,
    pub postal_code: Option<String>,
    pub country_code: Option<String>,

    pub latitude: Option<Decimal>,
    pub longitude: Option<Decimal>,

    pub auth_user: AuthUser,
}

impl LocationCreateCommand {
    pub fn new(request: LocationCreateRequest, auth_user: AuthUser) -> Self {
        Self {
            user_id: auth_user.user_id,
            name: request.name,
            address: request.address,
            city: request.city,
            region: request.region,
            postal_code: request.postal_code,
            country_code: request.country_code,
            latitude: request.latitude,
            longitude: request.longitude,
            auth_user,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationUpdateCommand {
    pub location_id: Uuid,
    pub name: String,
    pub address: Option<String>,
    pub city: Option<String>,
    pub region: Option<String>,
    pub postal_code: Option<String>,
    pub country_code: Option<String>,
    pub auth_user: AuthUser,
}

impl LocationUpdateCommand {
    pub fn new(location_id: Uuid, request: LocationUpdateRequest, auth_user: AuthUser) -> Self {
        Self {
            location_id,
            name: request.name,
            address: request.address,
            city: request.city,
            region: request.region,
            postal_code: request.postal_code,
            country_code: request.country_code,
            auth_user,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationUpdateLatLongCommand {
    pub location_id: Uuid,

    pub latitude: Option<Decimal>,
    pub longitude: Option<Decimal>,

    pub auth_user: AuthUser,
}

impl LocationUpdateLatLongCommand {
    pub fn new(location_id: Uuid, request: LocationUpdateLatLongRequest, auth_user: AuthUser) -> Self {
        Self {
            location_id,
            latitude: request.latitude,
            longitude: request.longitude,
            auth_user,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationArchivedCommand {
    pub location_id: Uuid,

    pub archived: bool,

    pub auth_user: AuthUser,
}

impl LocationArchivedCommand {
    pub fn new(location_id: Uuid, request: LocationUpdateArchivedRequest, auth_user: AuthUser) -> Self {
        Self {
            location_id,
            archived: request.archived,
            auth_user,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationDeleteCommand {
    pub location_id: Uuid,

    pub auth_user: AuthUser,
}

impl LocationDeleteCommand {
    pub fn new(location_id: Uuid, auth_user: AuthUser) -> Self {
        Self { location_id, auth_user }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationListByUserCommand {
    pub user_id: Uuid,
    pub pagination: Option<PaginationRequest>,

    pub auth_user: AuthUser,
}

impl LocationListByUserCommand {
    pub fn new(user_id: Uuid, pagination: Option<PaginationRequest>, auth_user: AuthUser) -> Self {
        Self { user_id, pagination, auth_user }
    }
}

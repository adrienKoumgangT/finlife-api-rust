use chrono::NaiveDate;
use rust_decimal::Decimal;
use uuid::Uuid;
use serde::{Serialize, Deserialize};

use crate::modules::locations::location_dto::{LocationCreateRequest, LocationUpdateArchivedRequest, LocationUpdateRequest, LocationUpdateNameRequest};
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
    pub location_name: String,

    pub location_address: Option<String>,
    pub location_city: Option<String>,
    pub location_region: Option<String>,
    pub location_postal_code: Option<String>,
    pub location_country_code: Option<String>,

    pub location_latitude: Option<Decimal>,
    pub location_longitude: Option<Decimal>,

    pub auth_user: AuthUser,
}

impl LocationCreateCommand {
    pub fn new(request: LocationCreateRequest, auth_user: AuthUser) -> Self {
        Self {
            user_id: auth_user.user_id,
            location_name: request.location_name,
            location_address: request.location_address,
            location_city: request.location_city,
            location_region: request.location_region,
            location_postal_code: request.location_postal_code,
            location_country_code: request.location_country_code,
            location_latitude: request.location_latitude,
            location_longitude: request.location_longitude,
            auth_user
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationUpdateNameCommand {
    pub location_id: Uuid,
    pub location_name: String,

    pub auth_user: AuthUser,
}

impl LocationUpdateNameCommand {
    pub fn new(location_id: Uuid, request: LocationUpdateNameRequest, auth_user: AuthUser) -> Self {
        Self {
            location_id,
            location_name: request.location_name,
            auth_user,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationUpdateCommand {
    pub location_id: Uuid,

    pub location_address: Option<String>,
    pub location_city: Option<String>,
    pub location_region: Option<String>,
    pub location_postal_code: Option<String>,
    pub location_country_code: Option<String>,

    pub auth_user: AuthUser,
}

impl LocationUpdateCommand {
    pub fn new(location_id: Uuid, request: LocationUpdateRequest, auth_user: AuthUser) -> Self {
        Self {
            location_id,
            location_address: request.location_address,
            location_city: request.location_city,
            location_region: request.location_region,
            location_postal_code: request.location_postal_code,
            location_country_code: request.location_country_code,
            auth_user,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationUpdateLatLongCommand {
    pub location_id: Uuid,

    pub location_latitude: Option<Decimal>,
    pub location_longitude: Option<Decimal>,

    pub auth_user: AuthUser,
}

impl LocationUpdateLatLongCommand {
    pub fn new(location_id: Uuid, location_latitude: Option<Decimal>, location_longitude: Option<Decimal>, auth_user: AuthUser) -> Self {
        Self {
            location_id,
            location_latitude,
            location_longitude,
            auth_user,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationArchivedCommand {
    pub location_id: Uuid,

    pub location_archived: bool,

    pub auth_user: AuthUser,
}

impl LocationArchivedCommand {
    pub fn new(location_id: Uuid, request: LocationUpdateArchivedRequest, auth_user: AuthUser) -> Self {
        Self {
            location_id,
            location_archived: request.location_archived,
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
        Self {
            location_id,
            auth_user,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationListCommand {
    pub pagination: Option<PaginationRequest>,

    pub auth_user: AuthUser,
}

impl LocationListCommand {
    pub fn new(pagination: Option<PaginationRequest>, auth_user: AuthUser) -> Self {
        Self { pagination, auth_user }
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

use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Serialize, Deserialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use crate::modules::locations::location_model::Location;
use crate::shared::utils::bu;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct LocationResponse {
    pub location_id: Uuid,

    pub user_id: Uuid,
    pub location_name: String,

    pub location_address: Option<String>,
    pub location_city: Option<String>,
    pub location_region: Option<String>,
    pub location_postal_code: Option<String>,
    pub location_country_code: Option<String>,

    pub location_latitude: Option<Decimal>,
    pub location_longitude: Option<Decimal>,

    pub location_archived: bool,
    pub location_created_at: Option<DateTime<Utc>>,
    pub location_updated_at: Option<DateTime<Utc>>,
}

impl From<Location> for LocationResponse {
    fn from(location: Location) -> Self {
        Self {
            location_id: bu(location.id.unwrap().as_slice()),
            user_id: bu(location.user_id.as_slice()),
            location_name: location.name,
            location_address: location.address,
            location_city: location.city,
            location_region: location.region,
            location_postal_code: location.postal_code,
            location_country_code: location.country_code,
            location_latitude: location.latitude,
            location_longitude: location.longitude,
            location_archived: location.archived,
            location_created_at: location.created_at,
            location_updated_at: location.updated_at,
        }
    }
}

impl From<&Location> for LocationResponse {
    fn from(location: &Location) -> Self {
        Self {
            location_id: bu(location.id.clone().unwrap().as_slice()),
            user_id: bu(location.user_id.clone().as_slice()),
            location_name: location.name.clone(),
            location_address: location.address.clone(),
            location_city: location.city.clone(),
            location_region: location.region.clone(),
            location_postal_code: location.postal_code.clone(),
            location_country_code: location.country_code.clone(),
            location_latitude: location.latitude.clone(),
            location_longitude: location.longitude.clone(),
            location_archived: location.archived.clone(),
            location_created_at: location.created_at.clone(),
            location_updated_at: location.updated_at.clone(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct LocationCreateRequest {
    pub location_name: String,

    pub location_address: Option<String>,
    pub location_city: Option<String>,
    pub location_region: Option<String>,
    pub location_postal_code: Option<String>,
    pub location_country_code: Option<String>,

    pub location_latitude: Option<Decimal>,
    pub location_longitude: Option<Decimal>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct LocationUpdateNameRequest {
    pub location_name: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct LocationUpdateRequest {
    pub location_address: Option<String>,
    pub location_city: Option<String>,
    pub location_region: Option<String>,
    pub location_postal_code: Option<String>,
    pub location_country_code: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct LocationUpdateArchivedRequest {
    pub location_archived: bool,
}

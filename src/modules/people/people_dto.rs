use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Serialize, Deserialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use crate::modules::people::people_model::People;
use crate::shared::utils::bu;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PeopleResponse {
    pub people_id: Uuid,
    pub user_id: Uuid,
    pub people_name: String,

    pub people_email: Option<String>,
    pub people_phone: Option<String>,
    pub people_image_url: Option<String>,
    pub people_note: Option<String>,

    pub people_archived: bool,
    pub people_created_at: Option<DateTime<Utc>>,
    pub people_updated_at: Option<DateTime<Utc>>,
}

impl From<People> for PeopleResponse {
    fn from(people: People) -> Self {
        Self {
            people_id: bu(people.id.unwrap().as_slice()),
            user_id: bu(people.user_id.as_slice()),
            people_name: people.name,
            people_email: people.email,
            people_phone: people.phone,
            people_image_url: people.image_url,
            people_note: people.note,
            people_archived: people.archived,
            people_created_at: people.created_at,
            people_updated_at: people.updated_at,
        }
    }
}

impl From<&People> for PeopleResponse {
    fn from(people: &People) -> Self {
        Self {
            people_id: bu(people.id.clone().unwrap().as_slice()),
            user_id: bu(people.user_id.clone().as_slice()),
            people_name: people.name.clone(),
            people_email: people.email.clone(),
            people_phone: people.phone.clone(),
            people_image_url: people.image_url.clone(),
            people_note: people.note.clone(),
            people_archived: people.archived.clone(),
            people_created_at: people.created_at.clone(),
            people_updated_at: people.updated_at.clone(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PeopleCreateRequest {
    pub people_name: String,

    pub people_email: Option<String>,
    pub people_phone: Option<String>,
    pub people_image_url: Option<String>,
    pub people_note: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PeopleUpdateRequest {
    pub people_name: String,
    pub people_email: Option<String>,
    pub people_phone: Option<String>,
    pub people_note: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PeopleUpdateArchivedRequest {
    pub people_archived: bool,
}

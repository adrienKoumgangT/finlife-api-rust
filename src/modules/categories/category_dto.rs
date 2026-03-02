use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::modules::categories::category_model::{Category, CategoryKind};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CategoryResponse {
    pub category_id: Uuid,

    pub name: String,
    pub kind: CategoryKind,
    pub parent_id: Option<Uuid>,
    pub sort_order: i32,

    pub archived: bool,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl From<Category> for CategoryResponse {
    fn from(category: Category) -> Self {
        Self {
            category_id: category.id.unwrap(),
            name: category.name,
            kind: category.kind,
            parent_id: category.parent_id,
            sort_order: category.sort_order,
            archived: category.archived,
            created_at: category.created_at,
            updated_at: category.updated_at,
        }
    }
}

impl From<&Category> for CategoryResponse {
    fn from(category: &Category) -> Self {
        Self {
            category_id: category.id.clone().unwrap(),
            name: category.name.clone(),
            kind: category.kind.clone(),
            parent_id: category.parent_id.clone(),
            sort_order: category.sort_order,
            archived: category.archived.clone(),
            created_at: category.created_at.clone(),
            updated_at: category.updated_at.clone(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CategoryCreateRequest {
    pub name: String,
    pub kind: CategoryKind,
    pub parent_id: Option<Uuid>,
    pub sort_order: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CategoryUpdateRequest {
    pub name: String,
    pub parent_id: Option<Uuid>,
    pub sort_order: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CategoryUpdateArchivedRequest {
    pub archived: bool,
}

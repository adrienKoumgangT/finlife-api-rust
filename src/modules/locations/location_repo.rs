use anyhow::{Error, Result};
use async_trait::async_trait;
use rust_decimal::Decimal;
use uuid::Uuid;
use sqlx::MySqlPool;

use crate::modules::locations::location_model::Location;
use crate::shared::state::AppState;

#[async_trait]
pub trait LocationRepositoryInterface {

    async fn get(&self, location_id: Uuid, user_id: Option<Uuid>) -> Result<Option<Location>, Error>;

    async fn create(&self, location: Location, user_id: Option<Uuid>) -> Result<Location, Error>;
    
    async fn update(&self, location_id: Uuid, name: String, address: Option<String>, city: Option<String>, district: Option<String>, region: Option<String>, postal_code: Option<String>, country_code: Option<String>, user_id: Option<Uuid>) -> Result<Option<Location>, Error>;

    async fn update_lat_long(&self, location_id: Uuid, latitude: Option<Decimal>, longitude: Option<Decimal>, user_id: Option<Uuid>) -> Result<Option<Location>, Error>;

    async fn archived(&self, location_id: Uuid, archived: bool, user_id: Option<Uuid>) -> Result<Option<Location>, Error>;

    async fn delete(&self, location_id: Uuid, user_id: Option<Uuid>) -> Result<(), Error>;

    async fn search_by_user(&self, user_id: Uuid, query: String, limit: Option<u32>, offset: Option<u32>) -> Result<Vec<Location>, Error>;

    async fn get_by_user(&self, user_id: Uuid, limit: Option<u32>, offset: Option<u32>) -> Result<Vec<Location>, Error>;

}

#[derive(Clone)]
pub struct LocationRepository {
    pool: MySqlPool,
}

impl From<&AppState> for LocationRepository {
    fn from(app_state: &AppState) -> Self {
        Self { pool: app_state.mysql_pool.clone() }
    }
}

#[async_trait]
impl LocationRepositoryInterface for LocationRepository {

    async fn get(&self, location_id: Uuid, user_id: Option<Uuid>) -> Result<Option<Location>, Error> {
        let location = sqlx::query_as!(
            Location,
            r#"
            SELECT
                id AS "id: _", user_id AS "user_id: _",
                name, address, city, district, region, postal_code, country_code,
                latitude, longitude,
                archived AS "archived: bool",
                created_at, updated_at
            FROM locations
            WHERE id = ? AND user_id = ?
            "#,
            location_id,
            user_id
        )
            .fetch_optional(&self.pool)
            .await?;

        Ok(location)
    }

    async fn create(&self, location: Location, user_id: Option<Uuid>) -> Result<Location, Error> {
        let new_id = Uuid::new_v4();

        sqlx::query!(
            r#"
            INSERT INTO locations
                (id, user_id, name, address, city, district, region, postal_code, country_code, latitude, longitude, archived)
            VALUES
                (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
            new_id,
            location.user_id,
            location.name,
            location.address,
            location.city,
            location.district,
            location.region,
            location.postal_code,
            location.country_code,
            location.latitude,
            location.longitude,
            location.archived
        )
            .execute(&self.pool)
            .await?;

        let result = self.get(new_id, user_id).await?;
        result.ok_or_else(|| Error::msg("Location not found after creation"))
    }

    async fn update(&self, location_id: Uuid, name: String, address: Option<String>, city: Option<String>, district: Option<String>, region: Option<String>, postal_code: Option<String>, country_code: Option<String>, user_id: Option<Uuid>) -> Result<Option<Location>, Error> {
        sqlx::query!(
            "UPDATE locations SET name = ?, address = ?, city = ?, district = ?, region = ?, postal_code = ?, country_code = ? WHERE id = ? AND user_id = ?",
            name, address, city, district, region, postal_code, country_code, location_id, user_id
        )
            .execute(&self.pool)
            .await?;

        self.get(location_id, user_id).await
    }

    async fn update_lat_long(&self, location_id: Uuid, latitude: Option<Decimal>, longitude: Option<Decimal>, user_id: Option<Uuid>) -> Result<Option<Location>, Error> {
        sqlx::query!(
            "UPDATE locations SET latitude = ?, longitude = ? WHERE id = ? AND user_id = ?",
            latitude, longitude, location_id, user_id
        )
            .execute(&self.pool)
            .await?;

        self.get(location_id, user_id).await
    }

    async fn archived(&self, location_id: Uuid, archived: bool, user_id: Option<Uuid>) -> Result<Option<Location>, Error> {
        sqlx::query!(
            "UPDATE locations SET archived = ? WHERE id = ? AND user_id = ?",
            archived,
            location_id,
            user_id
        )
            .execute(&self.pool)
            .await?;

        self.get(location_id, user_id).await
    }

    async fn delete(&self, location_id: Uuid, user_id: Option<Uuid>) -> Result<(), Error> {
        sqlx::query!("DELETE FROM locations WHERE id = ? AND user_id = ?", location_id, user_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn search_by_user(&self, user_id: Uuid, query: String, limit: Option<u32>, offset: Option<u32>) -> Result<Vec<Location>, Error> {
        let limit_val = limit.unwrap_or(100) as i64;
        let offset_val = offset.unwrap_or(0) as i64;
        let search_term = format!("%{}%", query);

        let locations = sqlx::query_as!(
            Location,
            r#"
            SELECT
                id AS "id: _", user_id AS "user_id: _",
                name, address, city, district, region, postal_code, country_code,
                latitude, longitude,
                archived AS "archived: bool",
                created_at, updated_at
            FROM locations
            WHERE user_id = ? AND (name LIKE ? OR address LIKE ? OR city LIKE ?)
            LIMIT ? OFFSET ?
            "#,
            user_id,
            search_term,
            search_term,
            search_term,
            limit_val,
            offset_val
        )
            .fetch_all(&self.pool)
            .await?;

        Ok(locations)
    }

    async fn get_by_user(&self, user_id: Uuid, limit: Option<u32>, offset: Option<u32>) -> Result<Vec<Location>, Error> {
        let limit_val = limit.unwrap_or(100) as i64;
        let offset_val = offset.unwrap_or(0) as i64;

        let locations = sqlx::query_as!(
            Location,
            r#"
            SELECT
                id AS "id: _", user_id AS "user_id: _",
                name, address, city, district, region, postal_code, country_code,
                latitude, longitude,
                archived AS "archived: bool",
                created_at, updated_at
            FROM locations
            WHERE user_id = ?
            LIMIT ? OFFSET ?
            "#,
            user_id,
            limit_val,
            offset_val
        )
            .fetch_all(&self.pool)
            .await?;

        Ok(locations)
    }
}

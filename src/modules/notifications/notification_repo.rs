use anyhow::{Error, Result};
use async_trait::async_trait;
use uuid::Uuid;
use sqlx::MySqlPool;

use crate::modules::notifications::notification_model::{NotificationType, Notification, NotificationPreference, NotificationSeverity};
use crate::shared::state::AppState;


#[async_trait]
pub trait NotificationRepositoryInterface {

    // Types
    async fn get_type(&self, type_id: Uuid) -> Result<Option<NotificationType>, Error>;

    async fn create_type(&self, notif_type: NotificationType) -> Result<NotificationType, Error>;

    #[allow(clippy::too_many_arguments)]
    async fn update_type(&self, type_id: Uuid, name: String, severity: NotificationSeverity, title_template: Option<String>, body_template: Option<String>, default_in_app: bool, default_email: bool, is_active: bool) -> Result<Option<NotificationType>, Error>;

    async fn get_all_type(&self, only_active: bool) -> Result<Vec<NotificationType>, Error>;


    // Notifications

    async fn get(&self, notification_id: Uuid, user_id: Uuid) -> Result<Option<Notification>, Error>;

    async fn get_by_user(&self, user_id: Uuid, limit: Option<u32>, offset: Option<u32>) -> Result<Vec<Notification>, Error>;

    async fn mark_as_read(&self, notification_id: Uuid, user_id: Uuid) -> Result<Option<Notification>, Error>;

    async fn archive(&self, notification_id: Uuid, user_id: Uuid) -> Result<(), Error>;


    // Preferences
    async fn get_preferences(&self, user_id: Uuid) -> Result<Vec<NotificationPreference>, Error>;

    async fn upsert_preference(&self, user_id: Uuid, type_id: Uuid, channel: String, enabled: bool) -> Result<(), Error>;

}

#[derive(Clone)]
pub struct NotificationRepository {
    pool: MySqlPool,
}

impl From<&AppState> for NotificationRepository {
    fn from(app_state: &AppState) -> Self {
        Self { pool: app_state.mysql_pool.clone() }
    }
}

#[async_trait]
impl NotificationRepositoryInterface for NotificationRepository {

    async fn get_type(&self, type_id: Uuid) -> Result<Option<NotificationType>, Error> {
        let nt = sqlx::query_as!(
            NotificationType,
            r#"
            SELECT
                id AS "id: _", code, name, severity AS "severity: String",
                title_template, body_template,
                default_in_app AS "default_in_app: bool", default_email AS "default_email: bool",
                is_active AS "is_active: bool", created_at, updated_at
            FROM notification_types
            WHERE id = ?
            "#,
            type_id
        )
            .fetch_optional(&self.pool)
            .await?;

        Ok(nt)
    }

    async fn create_type(&self, nt: NotificationType) -> Result<NotificationType, Error> {
        let new_id = Uuid::new_v4();
        let severity_str = nt.severity.as_str();

        sqlx::query!(
            r#"
            INSERT INTO notification_types
                (id, code, name, severity, title_template, body_template, default_in_app, default_email, is_active)
            VALUES
                (?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
            new_id, nt.code, nt.name, severity_str, nt.title_template, nt.body_template, nt.default_in_app, nt.default_email, nt.is_active
        )
            .execute(&self.pool)
            .await?;

        let result = self.get_type(new_id).await?;
        result.ok_or_else(|| Error::msg("Notification Type not found after creation"))
    }

    async fn update_type(&self, type_id: Uuid, name: String, severity: NotificationSeverity, title_template: Option<String>, body_template: Option<String>, default_in_app: bool, default_email: bool, is_active: bool) -> Result<Option<NotificationType>, Error> {
        let severity_str = severity.as_str();

        sqlx::query!(
            r#"
            UPDATE notification_types SET
                name = ?, severity = ?, title_template = ?, body_template = ?,
                default_in_app = ?, default_email = ?, is_active = ?
            WHERE id = ?
            "#,
            name, severity_str, title_template, body_template, default_in_app, default_email, is_active, type_id
        )
            .execute(&self.pool)
            .await?;

        self.get_type(type_id).await
    }

    async fn get_all_type(&self, only_active: bool) -> Result<Vec<NotificationType>, Error> {
        let nts = if only_active {
            sqlx::query_as!(
                NotificationType,
                r#"
                SELECT
                    id AS "id: _", code, name, severity AS "severity: String",
                    title_template, body_template,
                    default_in_app AS "default_in_app: bool", default_email AS "default_email: bool",
                    is_active AS "is_active: bool", created_at, updated_at
                FROM notification_types
                WHERE is_active = 1
                ORDER BY name ASC
                "#
            )
                .fetch_all(&self.pool)
                .await?
        } else {
            sqlx::query_as!(
                NotificationType,
                r#"
                SELECT
                    id AS "id: _", code, name, severity AS "severity: String",
                    title_template, body_template,
                    default_in_app AS "default_in_app: bool", default_email AS "default_email: bool",
                    is_active AS "is_active: bool", created_at, updated_at
                FROM notification_types
                ORDER BY name ASC
                "#
            )
                .fetch_all(&self.pool)
                .await?
        };

        Ok(nts)
    }



    async fn get(&self, notification_id: Uuid, user_id: Uuid) -> Result<Option<Notification>, Error> {
        let notif = sqlx::query_as!(
            Notification,
            r#"
            SELECT
                id AS "id: _", user_id AS "user_id: _", type_id AS "type_id: _",
                title, body, data, entity_type, entity_id AS "entity_id: _", link_url,
                is_read AS "is_read: bool", read_at, archived AS "archived: bool", created_at
            FROM notifications
            WHERE id = ? AND user_id = ? AND archived = 0
            "#,
            notification_id, user_id
        )
            .fetch_optional(&self.pool)
            .await?;

        Ok(notif)
    }

    async fn get_by_user(&self, user_id: Uuid, limit: Option<u32>, offset: Option<u32>) -> Result<Vec<Notification>, Error> {
        let limit_val = limit.unwrap_or(100) as i64;
        let offset_val = offset.unwrap_or(0) as i64;

        let notifs = sqlx::query_as!(
            Notification,
            r#"
            SELECT
                id AS "id: _", user_id AS "user_id: _", type_id AS "type_id: _",
                title, body, data, entity_type, entity_id AS "entity_id: _", link_url,
                is_read AS "is_read: bool", read_at, archived AS "archived: bool", created_at
            FROM notifications
            WHERE user_id = ? AND archived = 0
            ORDER BY created_at DESC
            LIMIT ? OFFSET ?
            "#,
            user_id, limit_val, offset_val
        )
            .fetch_all(&self.pool)
            .await?;

        Ok(notifs)
    }

    async fn mark_as_read(&self, notification_id: Uuid, user_id: Uuid) -> Result<Option<Notification>, Error> {
        sqlx::query!(
            "UPDATE notifications SET is_read = 1, read_at = NOW() WHERE id = ? AND user_id = ?",
            notification_id, user_id
        )
            .execute(&self.pool)
            .await?;

        self.get(notification_id, user_id).await
    }

    async fn archive(&self, notification_id: Uuid, user_id: Uuid) -> Result<(), Error> {
        sqlx::query!(
            "UPDATE notifications SET archived = 1 WHERE id = ? AND user_id = ?",
            notification_id, user_id
        )
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn get_preferences(&self, user_id: Uuid) -> Result<Vec<NotificationPreference>, Error> {
        let prefs = sqlx::query_as!(
            NotificationPreference,
            r#"
            SELECT
                user_id AS "user_id: _", type_id AS "type_id: _",
                channel AS "channel: String", enabled AS "enabled: bool",
                created_at, updated_at
            FROM notification_preferences
            WHERE user_id = ?
            "#,
            user_id
        )
            .fetch_all(&self.pool)
            .await?;

        Ok(prefs)
    }

    async fn upsert_preference(&self, user_id: Uuid, type_id: Uuid, channel: String, enabled: bool) -> Result<(), Error> {
        sqlx::query!(
            r#"
            INSERT INTO notification_preferences (user_id, type_id, channel, enabled)
            VALUES (?, ?, ?, ?)
            ON DUPLICATE KEY UPDATE enabled = VALUES(enabled)
            "#,
            user_id, type_id, channel, enabled
        )
            .execute(&self.pool)
            .await?;

        Ok(())
    }

}

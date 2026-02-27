use anyhow::{Error, Result};
use async_trait::async_trait;
use chrono::NaiveDateTime;
use uuid::Uuid;
use sqlx::MySqlPool;

use crate::modules::emails::email_model::{EmailTemplate, EmailMessage, EmailEvent, EmailStatus};
use crate::shared::state::AppState;

#[async_trait]
pub trait EmailRepositoryInterface {

    // --- TEMPLATES ---

    async fn create_template(&self, tpl: EmailTemplate) -> Result<EmailTemplate, Error>;

    async fn update_template(&self, template_id: Uuid, subject_tpl: String, body_text_tpl: Option<String>, body_html_tpl: Option<String>, description: Option<String>, variables: Option<serde_json::Value>, is_active: bool) -> Result<Option<EmailTemplate>, Error>;

    async fn get_templates(&self) -> Result<Vec<EmailTemplate>, Error>;


    // --- MESSAGES ---

    async fn create_message(&self, msg: EmailMessage) -> Result<EmailMessage, Error>;

    #[allow(clippy::too_many_arguments)]
    async fn update_message_status(&self, message_id: Uuid, status: EmailStatus, attempt_count: i32, next_attempt_at: Option<NaiveDateTime>, last_error: Option<String>, provider: Option<String>, provider_msg_id: Option<String>, sent_at: Option<NaiveDateTime>) -> Result<Option<EmailMessage>, Error>;

    async fn get_message(&self, message_id: Uuid) -> Result<Option<EmailMessage>, Error>;

    async fn get_messages_by_user(&self, user_id: Uuid, limit: Option<u32>, offset: Option<u32>) -> Result<Vec<EmailMessage>, Error>;


    // --- EVENTS ---

    async fn create_event(&self, evt: EmailEvent) -> Result<EmailEvent, Error>;

    async fn get_events_by_message(&self, message_id: Uuid) -> Result<Vec<EmailEvent>, Error>;

}

#[derive(Clone)]
pub struct EmailRepository {
    pool: MySqlPool,
}

impl From<&AppState> for EmailRepository {
    fn from(app_state: &AppState) -> Self {
        Self { pool: app_state.mysql_pool.clone() }
    }
}

#[async_trait]
impl EmailRepositoryInterface for EmailRepository {

    // --- TEMPLATES ---
    async fn create_template(&self, tpl: EmailTemplate) -> Result<EmailTemplate, Error> {
        let new_id = Uuid::new_v4();
        sqlx::query!(
            "INSERT INTO email_templates (id, code, locale, subject_tpl, body_text_tpl, body_html_tpl, description, variables, is_active) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
            new_id, tpl.code, tpl.locale, tpl.subject_tpl, tpl.body_text_tpl, tpl.body_html_tpl, tpl.description, tpl.variables, tpl.is_active
        ).execute(&self.pool).await?;

        let res = sqlx::query_as!(
            EmailTemplate,
            r#"SELECT id AS "id: _",
                    code, locale,
                    subject_tpl, body_text_tpl, body_html_tpl,
                    description, variables,
                    is_active AS "is_active: bool",
                    created_at, updated_at
            FROM email_templates
            WHERE id = ?"#,
            new_id
        ).fetch_one(&self.pool).await?;

        Ok(res)
    }

    async fn update_template(&self, template_id: Uuid, subject_tpl: String, body_text_tpl: Option<String>, body_html_tpl: Option<String>, description: Option<String>, variables: Option<serde_json::Value>, is_active: bool) -> Result<Option<EmailTemplate>, Error> {
        sqlx::query!(
            "UPDATE email_templates
            SET subject_tpl = ?, body_text_tpl = ?, body_html_tpl = ?, description = ?, variables = ?, is_active = ?
            WHERE id = ?",
            subject_tpl, body_text_tpl, body_html_tpl, description, variables, is_active, template_id
        ).execute(&self.pool).await?;

        let res = sqlx::query_as!(EmailTemplate, r#"SELECT id AS "id: _", code, locale, subject_tpl, body_text_tpl, body_html_tpl, description, variables, is_active AS "is_active: bool", created_at, updated_at FROM email_templates WHERE id = ?"#, template_id)
            .fetch_optional(&self.pool).await?;
        Ok(res)
    }

    async fn get_templates(&self) -> Result<Vec<EmailTemplate>, Error> {
        let res = sqlx::query_as!(
            EmailTemplate,
            r#"SELECT id AS "id: _",
                    code, locale,
                    subject_tpl, body_text_tpl, body_html_tpl,
                    description, variables,
                    is_active AS "is_active: bool",
                    created_at, updated_at
            FROM email_templates"#
        ).fetch_all(&self.pool).await?;
        Ok(res)
    }

    // --- MESSAGES ---
    async fn create_message(&self, msg: EmailMessage) -> Result<EmailMessage, Error> {
        let new_id = Uuid::new_v4();
        let status = msg.status.as_str();
        let priority = msg.priority.as_str();

        sqlx::query!(
            "INSERT INTO email_messages (id, user_id, template_id, to_email, to_name, from_email, from_name, reply_to, template_vars, subject, body_text, body_html, status, priority, attempt_count, max_attempts, scheduled_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            new_id, msg.user_id, msg.template_id, msg.to_email, msg.to_name, msg.from_email, msg.from_name, msg.reply_to, msg.template_vars, msg.subject, msg.body_text, msg.body_html, status, priority, msg.attempt_count, msg.max_attempts, msg.scheduled_at
        ).execute(&self.pool).await?;

        self.get_message(new_id).await?.ok_or_else(|| Error::msg("Message not found"))
    }

    async fn update_message_status(&self, message_id: Uuid, status: EmailStatus, attempt_count: i32, next_attempt_at: Option<NaiveDateTime>, last_error: Option<String>, provider: Option<String>, provider_msg_id: Option<String>, sent_at: Option<NaiveDateTime>) -> Result<Option<EmailMessage>, Error> {
        let status_str = status.as_str();

        sqlx::query!(
            "UPDATE email_messages
            SET status = ?, attempt_count = ?, next_attempt_at = ?, last_error = ?, provider = ?, provider_msg_id = ?, sent_at = ?
            WHERE id = ?",
            status_str, attempt_count, next_attempt_at, last_error, provider, provider_msg_id, sent_at, message_id
        ).execute(&self.pool).await?;

        let res = sqlx::query_as!(
            EmailMessage,
            r#"SELECT id AS "id: _",
                    user_id AS "user_id: _",
                    template_id AS "template_id: _",
                    to_email, to_name,
                    from_email, from_name,
                    reply_to,
                    template_vars, subject, body_text, body_html,
                    status AS "status: String",
                    priority AS "priority: String",
                    provider, provider_msg_id,
                    attempt_count, max_attempts, next_attempt_at, last_error,
                    scheduled_at, sent_at,
                    created_at, updated_at
            FROM email_messages
            WHERE id = ?"#,
            message_id
        ).fetch_optional(&self.pool).await?;
        Ok(res)
    }

    async fn get_message(&self, message_id: Uuid) -> Result<Option<EmailMessage>, Error> {
        let msg = sqlx::query_as!(
            EmailMessage,
            r#"SELECT id AS "id: _",
                    user_id AS "user_id: _",
                    template_id AS "template_id: _",
                    to_email, to_name,
                    from_email, from_name,
                    reply_to,
                    template_vars,
                    subject, body_text, body_html,
                    status AS "status: String",
                    priority AS "priority: String",
                    provider, provider_msg_id,
                    attempt_count, max_attempts, next_attempt_at, last_error,
                    scheduled_at, sent_at,
                    created_at, updated_at
            FROM email_messages
            WHERE id = ?"#,
            message_id
        ).fetch_optional(&self.pool).await?;
        Ok(msg)
    }

    async fn get_messages_by_user(&self, user_id: Uuid, limit: Option<u32>, offset: Option<u32>) -> Result<Vec<EmailMessage>, Error> {
        let l = limit.unwrap_or(100) as i64; let o = offset.unwrap_or(0) as i64;
        let res = sqlx::query_as!(
            EmailMessage,
            r#"SELECT id AS "id: _",
                    user_id AS "user_id: _",
                    template_id AS "template_id: _",
                    to_email, to_name,
                    from_email, from_name,
                    reply_to,
                    template_vars,
                    subject, body_text, body_html,
                    status AS "status: String",
                    priority AS "priority: String",
                    provider, provider_msg_id,
                    attempt_count, max_attempts, next_attempt_at, last_error,
                    scheduled_at, sent_at,
                    created_at, updated_at
            FROM email_messages
            WHERE user_id = ?
            ORDER BY created_at DESC
            LIMIT ? OFFSET ?"#,
            user_id, l, o
        ).fetch_all(&self.pool).await?;
        Ok(res)
    }

    // --- EVENTS ---
    async fn create_event(&self, evt: EmailEvent) -> Result<EmailEvent, Error> {
        let new_id = Uuid::new_v4();
        let evt_type = evt.event_type.as_str();

        sqlx::query!(
            "INSERT INTO email_events (id, email_message_id, event_type, event_at, meta) VALUES (?, ?, ?, ?, ?)",
            new_id, evt.email_message_id, evt_type, evt.event_at, evt.meta
        ).execute(&self.pool).await?;

        let res = sqlx::query_as!(
            EmailEvent,
            r#"SELECT id AS "id: _",
                    email_message_id AS "email_message_id: _",
                    event_type AS "event_type: String",
                    event_at, meta, created_at
            FROM email_events
            WHERE id = ?"#,
            new_id
        ).fetch_one(&self.pool).await?;
        Ok(res)
    }

    async fn get_events_by_message(&self, message_id: Uuid) -> Result<Vec<EmailEvent>, Error> {
        let res = sqlx::query_as!(
            EmailEvent,
            r#"SELECT id AS "id: _",
                    email_message_id AS "email_message_id: _",
                    event_type AS "event_type: String",
                    event_at, meta, created_at
            FROM email_events
            WHERE email_message_id = ?
            ORDER BY event_at ASC"#,
            message_id
        ).fetch_all(&self.pool).await?;
        Ok(res)
    }
}

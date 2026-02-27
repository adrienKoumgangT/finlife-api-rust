use anyhow::Result;
use async_trait::async_trait;
use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use uuid::Uuid;

use crate::modules::emails::{
    email_command::*,
    email_dto::*,
    email_model::{EmailTemplate, EmailMessage, EmailEvent},
    email_repo::{EmailRepository, EmailRepositoryInterface}
};
use crate::shared::{
    db::redis::{delete_key, get_key, set_key},
    errors::AppError,
    state::AppState,
    utils::extract_pagination_data
};

#[async_trait]
pub trait EmailInterface {

    // --- TEMPLATES ---

    async fn create_template(&self, command: EmailTemplateCreateCommand) -> Result<EmailTemplateResponse, AppError>;

    async fn update_template(&self, command: EmailTemplateUpdateCommand) -> Result<Option<EmailTemplateResponse>, AppError>;

    async fn get_templates(&self) -> Result<Vec<EmailTemplateResponse>, AppError>;


    // --- MESSAGES ---

    async fn create_message(&self, command: EmailMessageCreateCommand) -> Result<EmailMessageResponse, AppError>;

    async fn update_message_status(&self, command: EmailMessageUpdateStatusCommand) -> Result<Option<EmailMessageResponse>, AppError>;

    async fn get_message(&self, command: GetMessageCommand) -> Result<Option<EmailMessageResponse>, AppError>;

    async fn get_messages_by_user(&self, command: ListMessagesByUserCommand) -> Result<Vec<EmailMessageResponse>, AppError>;


    // --- EVENTS ---

    async fn create_event(&self, command: EmailEventCreateCommand) -> Result<EmailEventResponse, AppError>;

    async fn get_events(&self, message_id: Uuid) -> Result<Vec<EmailEventResponse>, AppError>;

}

#[derive(Clone)]
pub struct EmailService {
    email_repo: EmailRepository,
    redis_pool: Option<Pool<RedisConnectionManager>>,
}

impl From<&AppState> for EmailService {
    fn from(app_state: &AppState) -> Self {
        Self { email_repo: EmailRepository::from(app_state), redis_pool: Option::from(app_state.redis_pool.clone()) }
    }
}

impl EmailService {
    fn ttl(&self) -> Option<u64> { Some(60 * 60) }
    fn key_templates(&self) -> String { "emails:templates".to_string() }
    fn key_msg_list(&self, user_id: &Uuid) -> String { format!("user:{}:emails", user_id) }
    fn key_evt_list(&self, msg_id: &Uuid) -> String { format!("email:{}:events", msg_id) }
}

#[async_trait]
impl EmailInterface for EmailService {

    // --- TEMPLATES ---

    async fn create_template(&self, command: EmailTemplateCreateCommand) -> Result<EmailTemplateResponse, AppError> {
        let tpl = self.email_repo.create_template(EmailTemplate::from(command)).await
            .map_err(AppError::Internal)?;

        if let Some(r) = &self.redis_pool {
            let _: () = delete_key(r, &self.key_templates()).await
                .map_err(AppError::Internal)?;
        }

        Ok(EmailTemplateResponse::from(tpl))
    }

    async fn update_template(&self, command: EmailTemplateUpdateCommand) -> Result<Option<EmailTemplateResponse>, AppError> {
        let tpl = self.email_repo.update_template(
            command.template_id, command.subject_tpl, command.body_text_tpl,
            command.body_html_tpl, command.description, command.variables, command.is_active
        ).await.map_err(AppError::Internal)?;

        // Invalidate global templates cache
        if let Some(r) = &self.redis_pool {
            let _: () = delete_key(r, &self.key_templates()).await
                .map_err(AppError::Internal)?;
        }

        Ok(tpl.map(EmailTemplateResponse::from))
    }

    async fn get_templates(&self) -> Result<Vec<EmailTemplateResponse>, AppError> {
        if let Some(r) = &self.redis_pool {
            let cache: Option<Vec<EmailTemplateResponse>> = get_key(r, &self.key_templates()).await
                .map_err(AppError::Internal)?;
            if let Some(c) = cache { return Ok(c); }
        }

        let tpls = self.email_repo.get_templates().await
            .map_err(AppError::Internal)?;
        let res: Vec<EmailTemplateResponse> = tpls.into_iter().map(EmailTemplateResponse::from).collect();

        if let Some(r) = &self.redis_pool {
            let _: () = set_key(r, &self.key_templates(), &res, self.ttl()).await
                .map_err(AppError::Internal)?;
        }

        Ok(res)
    }


    // --- MESSAGES ---

    async fn create_message(&self, command: EmailMessageCreateCommand) -> Result<EmailMessageResponse, AppError> {
        let user_id = command.auth_user.user_id.clone();
        let msg = self.email_repo.create_message(EmailMessage::from(command)).await
            .map_err(AppError::Internal)?;

        if let Some(r) = &self.redis_pool {
            let _: () = delete_key(r, &self.key_msg_list(&user_id)).await
                .map_err(AppError::Internal)?;
        }

        Ok(EmailMessageResponse::from(msg))
    }

    async fn update_message_status(&self, command: EmailMessageUpdateStatusCommand) -> Result<Option<EmailMessageResponse>, AppError> {
        let msg = self.email_repo.update_message_status(
            command.message_id, command.status, command.attempt_count,
            command.next_attempt_at, command.last_error, command.provider,
            command.provider_msg_id, command.sent_at
        ).await.map_err(AppError::Internal)?;

        if let Some(m) = &msg {
            // Invalidate the specific user's message outbox cache if it belongs to a user
            if let Some(user_id) = m.user_id {
                if let Some(r) = &self.redis_pool {
                    let _: () = delete_key(r, &self.key_msg_list(&user_id)).await
                        .map_err(AppError::Internal)?;
                }
            }
        }

        Ok(msg.map(EmailMessageResponse::from))
    }

    async fn get_message(&self, command: GetMessageCommand) -> Result<Option<EmailMessageResponse>, AppError> {
        let msg = self.email_repo.get_message(command.message_id).await
            .map_err(AppError::Internal)?;

        Ok(msg.map(EmailMessageResponse::from))
    }

    async fn get_messages_by_user(&self, command: ListMessagesByUserCommand) -> Result<Vec<EmailMessageResponse>, AppError> {
        let (limit, offset, _) = extract_pagination_data(command.pagination);

        if let Some(r) = &self.redis_pool {
            let cache: Option<Vec<EmailMessageResponse>> = get_key(r, &self.key_msg_list(&command.user_id)).await
                .map_err(AppError::Internal)?;
            if let Some(c) = cache { return Ok(c); }
        }

        let msgs = self.email_repo.get_messages_by_user(command.user_id, limit, offset).await
            .map_err(AppError::Internal)?;
        let res: Vec<EmailMessageResponse> = msgs.into_iter().map(EmailMessageResponse::from).collect();

        if let Some(r) = &self.redis_pool {
            let _: () = set_key(r, &self.key_msg_list(&command.user_id), &res, self.ttl()).await
                .map_err(AppError::Internal)?;
        }

        Ok(res)
    }


    // --- EVENTS ---

    async fn create_event(&self, command: EmailEventCreateCommand) -> Result<EmailEventResponse, AppError> {
        let msg_id = command.email_message_id.clone();
        let evt = self.email_repo.create_event(EmailEvent::from(command)).await
            .map_err(AppError::Internal)?;

        if let Some(r) = &self.redis_pool {
            let _: () = delete_key(r, &self.key_evt_list(&msg_id)).await
                .map_err(AppError::Internal)?;
        }

        Ok(EmailEventResponse::from(evt))
    }

    async fn get_events(&self, message_id: Uuid) -> Result<Vec<EmailEventResponse>, AppError> {
        if let Some(r) = &self.redis_pool {
            let cache: Option<Vec<EmailEventResponse>> = get_key(r, &self.key_evt_list(&message_id)).await
                .map_err(AppError::Internal)?;
            if let Some(c) = cache { return Ok(c); }
        }

        let evts = self.email_repo.get_events_by_message(message_id).await
            .map_err(AppError::Internal)?;
        let res: Vec<EmailEventResponse> = evts.into_iter().map(EmailEventResponse::from).collect();

        if let Some(r) = &self.redis_pool {
            let _: () = set_key(r, &self.key_evt_list(&message_id), &res, self.ttl()).await
                .map_err(AppError::Internal)?;
        }

        Ok(res)
    }
}

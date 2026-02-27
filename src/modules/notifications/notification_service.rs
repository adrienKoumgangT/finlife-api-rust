use anyhow::Result;
use async_trait::async_trait;
use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use uuid::Uuid;

use crate::modules::notifications::{
    notification_command::*,
    notification_dto::*,
    notification_repo::{NotificationRepository, NotificationRepositoryInterface}
};
use crate::modules::notifications::notification_model::NotificationType;
use crate::shared::{
    db::redis::{delete_key, get_key, set_key},
    errors::AppError,
    state::AppState,
    utils::extract_pagination_data
};

#[async_trait]
pub trait NotificationInterface {

    async fn get_type(&self, command: NotificationTypeGetCommand) -> Result<Option<NotificationTypeResponse>, AppError>;

    async fn create_type(&self, command: NotificationTypeCreateCommand) -> Result<NotificationTypeResponse, AppError>;

    async fn update_type(&self, command: NotificationTypeUpdateCommand) -> Result<Option<NotificationTypeResponse>, AppError>;

    async fn list_type(&self, command: NotificationTypeListCommand) -> Result<Vec<NotificationTypeResponse>, AppError>;



    async fn get(&self, command: NotificationGetCommand) -> Result<Option<NotificationResponse>, AppError>;

    async fn get_all(&self, command: NotificationListCommand) -> Result<Vec<NotificationResponse>, AppError>;

    async fn mark_read(&self, command: NotificationMarkReadCommand) -> Result<Option<NotificationResponse>, AppError>;

    async fn archive(&self, command: NotificationArchiveCommand) -> Result<(), AppError>;



    async fn get_preferences(&self, user_id: Uuid) -> Result<Vec<NotificationPreferenceResponse>, AppError>;

    async fn update_preference(&self, command: PreferenceUpdateCommand) -> Result<(), AppError>;

}

#[derive(Clone)]
pub struct NotificationService {
    notif_repo: NotificationRepository,
    redis_pool: Option<Pool<RedisConnectionManager>>,
}

impl From<&AppState> for NotificationService {
    fn from(app_state: &AppState) -> Self {
        let notif_repo = NotificationRepository::from(app_state);
        let redis_pool = app_state.redis_pool.clone();
        Self { notif_repo, redis_pool: Option::from(redis_pool) }
    }
}

impl NotificationService {

    fn redis_key_ttl(&self) -> Option<u64> { Some(60 * 60) }

    fn form_redis_key_types(&self, active_only: bool) -> String { format!("system:notification:types:list:active_{}", active_only) }
    fn form_redis_key_list(&self, user_id: &Uuid) -> String { format!("user:{}:notifications", user_id) }
    fn form_redis_key_prefs(&self, user_id: &Uuid) -> String { format!("user:{}:notification_prefs", user_id) }

    async fn invalidate_list_caches(&self) -> Result<(), AppError> {
        if let Some(redis_pool) = &self.redis_pool {
            let _: () = delete_key(redis_pool, self.form_redis_key_types(true).as_str()).await
                .map_err(AppError::Internal)?;
            let _: () = delete_key(redis_pool, self.form_redis_key_types(false).as_str()).await
                .map_err(AppError::Internal)?;
        }
        Ok(())
    }

    async fn invalidate_user_cache(&self, user_id: &Uuid) -> Result<(), AppError> {
        if let Some(redis_pool) = &self.redis_pool {
            let _: () = delete_key(redis_pool, self.form_redis_key_list(user_id).as_str()).await
                .map_err(AppError::Internal)?;
        }
        Ok(())
    }
}

#[async_trait]
impl NotificationInterface for NotificationService {

    async fn get_type(&self, command: NotificationTypeGetCommand) -> Result<Option<NotificationTypeResponse>, AppError> {
        let nt = self.notif_repo.get_type(command.type_id).await.map_err(AppError::Internal)?;
        Ok(nt.map(NotificationTypeResponse::from))
    }

    async fn create_type(&self, command: NotificationTypeCreateCommand) -> Result<NotificationTypeResponse, AppError> {
        let nt_create = NotificationType::from(command);
        let nt = self.notif_repo.create_type(nt_create).await.map_err(AppError::Internal)?;

        self.invalidate_list_caches().await?;

        Ok(NotificationTypeResponse::from(nt))
    }

    async fn update_type(&self, command: NotificationTypeUpdateCommand) -> Result<Option<NotificationTypeResponse>, AppError> {
        let nt = self.notif_repo.update_type(
            command.type_id, command.name, command.severity, command.title_template,
            command.body_template, command.default_in_app, command.default_email, command.is_active
        ).await.map_err(AppError::Internal)?;

        self.invalidate_list_caches().await?;

        Ok(nt.map(NotificationTypeResponse::from))
    }

    async fn list_type(&self, command: NotificationTypeListCommand) -> Result<Vec<NotificationTypeResponse>, AppError> {
        let cache_key = self.form_redis_key_types(command.only_active);

        if let Some(redis_pool) = &self.redis_pool {
            let cache: Option<Vec<NotificationTypeResponse>> = get_key(redis_pool, cache_key.as_str()).await.map_err(AppError::Internal)?;
            if let Some(nts) = cache { return Ok(nts); }
        }

        let nts = self.notif_repo.get_all_type(command.only_active).await.map_err(AppError::Internal)?;
        let response: Vec<NotificationTypeResponse> = nts.into_iter().map(NotificationTypeResponse::from).collect();

        if let Some(redis_pool) = &self.redis_pool {
            let _: () = set_key(redis_pool, cache_key.as_str(), &response, self.redis_key_ttl()).await.map_err(AppError::Internal)?;
        }

        Ok(response)
    }



    async fn get(&self, command: NotificationGetCommand) -> Result<Option<NotificationResponse>, AppError> {
        let notif = self.notif_repo.get(command.notification_id, command.auth_user.user_id).await
            .map_err(AppError::Internal)?;
        Ok(notif.map(NotificationResponse::from))
    }

    async fn get_all(&self, command: NotificationListCommand) -> Result<Vec<NotificationResponse>, AppError> {
        let (limit, offset, _) = extract_pagination_data(command.pagination);

        if let Some(redis_pool) = &self.redis_pool {
            let cache: Option<Vec<NotificationResponse>> = get_key(
                redis_pool, self.form_redis_key_list(&command.auth_user.user_id).as_str()
            ).await.map_err(AppError::Internal)?;

            if let Some(notifs) = cache { return Ok(notifs); }
        }

        let notifs = self.notif_repo.get_by_user(command.auth_user.user_id, limit, offset).await
            .map_err(AppError::Internal)?;
        let response: Vec<NotificationResponse> = notifs.into_iter().map(NotificationResponse::from).collect();

        if let Some(redis_pool) = &self.redis_pool {
            let _: () = set_key(
                redis_pool, self.form_redis_key_list(&command.auth_user.user_id).as_str(),
                &response, self.redis_key_ttl()
            ).await.map_err(AppError::Internal)?;
        }

        Ok(response)
    }

    async fn mark_read(&self, command: NotificationMarkReadCommand) -> Result<Option<NotificationResponse>, AppError> {
        let notif = self.notif_repo.mark_as_read(command.notification_id, command.auth_user.user_id).await
            .map_err(AppError::Internal)?;
        self.invalidate_user_cache(&command.auth_user.user_id).await?;
        Ok(notif.map(NotificationResponse::from))
    }

    async fn archive(&self, command: NotificationArchiveCommand) -> Result<(), AppError> {
        self.notif_repo.archive(command.notification_id, command.auth_user.user_id).await
            .map_err(AppError::Internal)?;
        self.invalidate_user_cache(&command.auth_user.user_id).await?;
        Ok(())
    }

    // --- Preferences ---
    async fn get_preferences(&self, user_id: Uuid) -> Result<Vec<NotificationPreferenceResponse>, AppError> {
        if let Some(redis_pool) = &self.redis_pool {
            let cache: Option<Vec<NotificationPreferenceResponse>> = get_key(
                redis_pool, self.form_redis_key_prefs(&user_id).as_str()
            ).await.map_err(AppError::Internal)?;

            if let Some(prefs) = cache { return Ok(prefs); }
        }

        let prefs = self.notif_repo.get_preferences(user_id).await
            .map_err(AppError::Internal)?;
        let response: Vec<NotificationPreferenceResponse> = prefs.into_iter().map(NotificationPreferenceResponse::from).collect();

        if let Some(redis_pool) = &self.redis_pool {
            let _: () = set_key(
                redis_pool, self.form_redis_key_prefs(&user_id).as_str(),
                &response, self.redis_key_ttl()
            ).await.map_err(AppError::Internal)?;
        }

        Ok(response)
    }

    async fn update_preference(&self, command: PreferenceUpdateCommand) -> Result<(), AppError> {
        self.notif_repo.upsert_preference(command.auth_user.user_id, command.type_id, command.channel, command.enabled).await
            .map_err(AppError::Internal)?;

        if let Some(redis_pool) = &self.redis_pool {
            let _: () = delete_key(redis_pool, self.form_redis_key_prefs(&command.auth_user.user_id).as_str()).await
                .map_err(AppError::Internal)?;
        }
        Ok(())
    }

}

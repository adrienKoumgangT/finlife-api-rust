use anyhow::Result;
use async_trait::async_trait;
use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use std::sync::Arc;
use std::time::Duration;
use uuid::Uuid;

use crate::modules::files::{
    file_command::*,
    file_dto::*,
    file_model::AppFile,
    file_repo::{FileRepository, FileRepositoryInterface}
};
use crate::shared::{
    db::redis::{delete_key, get_key, set_key},
    errors::AppError,
    state::AppState,
    storage::StorageProvider,
    utils::extract_pagination_data
};

#[async_trait]
pub trait FileInterface {
    
    async fn get(&self, command: FileGetCommand) -> Result<Option<FileResponse>, AppError>;

    async fn create_upload(&self, command: FileCreateCommand) -> Result<FileUploadInitResponse, AppError>;

    async fn get_download_url(&self, command: FileGetCommand) -> Result<FileDownloadResponse, AppError>;
    
    async fn update_status(&self, command: FileUpdateStatusCommand) -> Result<Option<FileResponse>, AppError>;
    
    async fn delete(&self, command: FileDeleteCommand) -> Result<(), AppError>;
    
    async fn get_by_user(&self, command: FileListByUserCommand) -> Result<Vec<FileResponse>, AppError>;
    
}

#[derive(Clone)]
pub struct FileService {
    file_repo: FileRepository,
    redis_pool: Option<Pool<RedisConnectionManager>>,
    storage_provider: Arc<dyn StorageProvider>,
}

impl From<&AppState> for FileService {
    fn from(app_state: &AppState) -> Self {
        let file_repo = FileRepository::from(app_state);
        let redis_pool = app_state.redis_pool.clone();
        let storage_provider = app_state.storage_provider.clone();

        Self { file_repo, redis_pool: Option::from(redis_pool), storage_provider }
    }
}

impl FileService {
    fn redis_key_ttl(&self) -> Option<u64> { Some(60 * 60) }

    fn form_redis_key_file(&self, key: &Uuid) -> String { format!("file:{}", key) }
    fn form_redis_key_list_by_user(&self, user: &Uuid) -> String { format!("user:{}:files", user) }

    async fn cache_file(&self, file: &FileResponse) -> Result<(), AppError> {
        if let Some(redis_pool) = &self.redis_pool {
            let _: () = set_key(redis_pool, self.form_redis_key_file(&file.file_id).as_str(), &file, self.redis_key_ttl()).await
                .map_err(AppError::Internal)?;
        }
        Ok(())
    }

    async fn cache_files_by_user(&self, user: &Uuid, files: &Vec<FileResponse>) -> Result<(), AppError> {
        if let Some(redis_pool) = &self.redis_pool {
            let _: () = set_key(redis_pool, self.form_redis_key_list_by_user(user).as_str(), &files, self.redis_key_ttl()).await
                .map_err(AppError::Internal)?;
        }
        Ok(())
    }

    async fn delete_cache(&self, key: &Uuid, user: &Uuid) -> Result<(), AppError> {
        if let Some(redis_pool) = &self.redis_pool {
            let _: () = delete_key(redis_pool, self.form_redis_key_file(key).as_str()).await
                .map_err(AppError::Internal)?;
            let _: () = delete_key(redis_pool, self.form_redis_key_list_by_user(user).as_str()).await
                .map_err(AppError::Internal)?;
        }
        Ok(())
    }

    async fn handle_res_opt_file(&self, file: anyhow::Result<Option<AppFile>>, auth_user: &Uuid) -> Result<Option<FileResponse>, AppError> {
        let file = file.map_err(AppError::Internal)?;

        if let Some(f) = file {
            let response = FileResponse::from(f);
            self.cache_file(&response).await?;
            if let Some(redis_pool) = &self.redis_pool {
                let _: () = delete_key(redis_pool, self.form_redis_key_list_by_user(auth_user).as_str()).await
                    .map_err(AppError::Internal)?;
            }
            Ok(Some(response))
        } else {
            Ok(None)
        }
    }
}

#[async_trait]
impl FileInterface for FileService {
    async fn get(&self, command: FileGetCommand) -> Result<Option<FileResponse>, AppError> {
        if let Some(redis_pool) = &self.redis_pool {
            if let Ok(Some(file)) = get_key(redis_pool, self.form_redis_key_file(&command.file_id).as_str()).await { return Ok(Some(file)); }
        }

        let file = self.file_repo.get(command.file_id, command.auth_user.user_id).await;
        self.handle_res_opt_file(file, &command.auth_user.user_id).await
    }

    async fn create_upload(&self, command: FileCreateCommand) -> Result<FileUploadInitResponse, AppError> {
        let meta_user = command.auth_user.user_id.clone();
        let mime_type = command.mime_type.clone();
        let storage_path = command.storage_path.clone();

        let file_create = AppFile::from(command);

        let file = self.file_repo.create(file_create, meta_user).await
            .map_err(AppError::Internal)?;
        let response = FileResponse::from(file);

        let upload_url = self.storage_provider
            .generate_upload_presigned_url(&storage_path, &mime_type, Duration::from_secs(3600)).await
            .map_err(AppError::Internal)?;

        if let Some(redis_pool) = &self.redis_pool {
            let _: () = delete_key(redis_pool, format!("user:{}:files", meta_user).as_str()).await
                .map_err(AppError::Internal)?;
        }

        Ok(FileUploadInitResponse { file: response, upload_url })
    }

    async fn get_download_url(&self, command: FileGetCommand) -> Result<FileDownloadResponse, AppError> {
        let file = self.file_repo.get(command.file_id, command.auth_user.user_id).await
            .map_err(AppError::Internal)?
            .ok_or_else(|| AppError::NotFound("File not found".to_string()))?;

        let download_url = self.storage_provider
            .generate_download_presigned_url(&file.storage_path, Duration::from_secs(3600)).await
            .map_err(AppError::Internal)?;

        Ok(FileDownloadResponse { download_url })
    }

    async fn update_status(&self, command: FileUpdateStatusCommand) -> Result<Option<FileResponse>, AppError> {
        let file = self.file_repo.update_status(command.file_id, command.status, command.auth_user.user_id).await;
        
        self.handle_res_opt_file(file, &command.auth_user.user_id).await
    }

    async fn delete(&self, command: FileDeleteCommand) -> Result<(), AppError> {
        self.file_repo.delete(command.file_id.clone(), command.auth_user.user_id).await
            .map_err(AppError::Internal)?;
        
        self.delete_cache(&command.file_id, &command.auth_user.user_id).await?;
        
        Ok(())
    }

    async fn get_by_user(&self, command: FileListByUserCommand) -> Result<Vec<FileResponse>, AppError> {
        let (limit, offset, _search) = extract_pagination_data(command.pagination);

        if let Some(redis_pool) = &self.redis_pool {
            let cache: Option<Vec<FileResponse>> = get_key(redis_pool, self.form_redis_key_list_by_user(&command.user_id).as_str()).await
                .map_err(AppError::Internal)?;
            if let Some(files) = cache { return Ok(files); }
        }

        let files = self.file_repo.get_by_user(command.user_id, limit, offset).await
            .map_err(AppError::Internal)?;

        let response: Vec<FileResponse> = files.into_iter().map(FileResponse::from).collect();
        self.cache_files_by_user(&command.user_id, &response).await?;

        Ok(response)
    }
}

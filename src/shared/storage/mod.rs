pub mod s3_provider;

use anyhow::Result;
use async_trait::async_trait;
use std::time::Duration;

#[async_trait]
pub trait StorageProvider: Send + Sync {

    /// Generates a temporary URL that the frontend can use to upload a file directly.
    async fn generate_upload_presigned_url(&self, file_path: &str, content_type: &str, expiry: Duration) -> Result<String>;

    /// Generates a temporary URL to view/download a private file.
    async fn generate_download_presigned_url(&self, file_path: &str, expiry: Duration) -> Result<String>;

    /// Deletes a file directly from the storage.
    async fn delete_file(&self, file_path: &str) -> Result<()>;
}

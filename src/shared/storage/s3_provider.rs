use anyhow::{Error, Result};
use async_trait::async_trait;
use aws_sdk_s3::presigning::PresigningConfig;
use aws_sdk_s3::Client;
use std::time::Duration;
use aws_config::BehaviorVersion;
use crate::shared::config::AppStorageConfig;
use crate::shared::storage::StorageProvider;

#[derive(Clone)]
pub struct S3StorageProvider {
    client: Client,
    bucket_name: String,
}

impl S3StorageProvider {
    pub async fn new(storage_config: &AppStorageConfig) -> Self {
        // Load AWS config from environment variables (AWS_ACCESS_KEY_ID, AWS_SECRET_ACCESS_KEY, AWS_REGION)
        let mut config_loader = aws_config::defaults(BehaviorVersion::latest());

        // If an endpoint URL is provided (e.g., http://localhost:9000 for Minio), override the default AWS endpoint
        if let Some(endpoint) = storage_config.storage_endpoint_url.clone() {
            config_loader = config_loader.endpoint_url(endpoint);
        }

        let sdk_config = config_loader.load().await;
        let client = Client::new(&sdk_config);
        
        let bucket_name = storage_config.bucket_name.clone();

        Self { client, bucket_name }
    }
}

#[async_trait]
impl StorageProvider for S3StorageProvider {

    async fn generate_upload_presigned_url(&self, file_path: &str, content_type: &str, expiry: Duration) -> Result<String> {
        let presigning_config = PresigningConfig::expires_in(expiry)
            .map_err(|e| Error::msg(format!("Failed to configure presigning: {}", e)))?;

        let presigned_req = self.client
            .put_object()
            .bucket(&self.bucket_name)
            .key(file_path)
            .content_type(content_type)
            .presigned(presigning_config)
            .await
            .map_err(|e| Error::msg(format!("Failed to generate upload URL: {}", e)))?;

        Ok(presigned_req.uri().to_string())
    }

    async fn generate_download_presigned_url(&self, file_path: &str, expiry: Duration) -> Result<String> {
        let presigning_config = PresigningConfig::expires_in(expiry)
            .map_err(|e| Error::msg(format!("Failed to configure presigning: {}", e)))?;

        let presigned_req = self.client
            .get_object()
            .bucket(&self.bucket_name)
            .key(file_path)
            .presigned(presigning_config)
            .await
            .map_err(|e| Error::msg(format!("Failed to generate download URL: {}", e)))?;

        Ok(presigned_req.uri().to_string())
    }

    async fn delete_file(&self, file_path: &str) -> Result<()> {
        self.client
            .delete_object()
            .bucket(&self.bucket_name)
            .key(file_path)
            .send()
            .await
            .map_err(|e| Error::msg(format!("Failed to delete file from S3: {}", e)))?;

        Ok(())
    }
}

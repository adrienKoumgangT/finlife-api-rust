use anyhow::Result;
use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use sqlx::MySqlPool;
use std::fs;

use crate::shared::auth::jwt::JwtVerifier;
use crate::shared::config::AppConfig;
use crate::shared::db::mysql as my_mysql;
use crate::shared::db::redis as my_redis;
// use crate::shared::metrics::prometheus::Metrics;

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub jwt: JwtVerifier,
    pub mysql_pool: MySqlPool,
    pub redis_pool: Pool<RedisConnectionManager>,
    // pub metrics: Metrics,
}

impl AppState {
    pub async fn new(config: AppConfig) -> Result<Self> {
        let config_clone = config.clone();

        let public_pem_content = fs::read_to_string(&config.jwt.public_secret_pem_path)
            .map_err(|e| anyhow::anyhow!("Failed to read public key at {:?}: {}", &config.jwt.public_secret_pem_path, e))?;

        let private_pem_content = fs::read_to_string(&config.jwt.private_secret_pem_path)
            .map_err(|e| anyhow::anyhow!("Failed to read public key at {:?}: {}", &config.jwt.private_secret_pem_path, e))?;

        let jwt = JwtVerifier::new(&public_pem_content, &private_pem_content, config.jwt.issuer.as_str(), config.jwt.audience.as_str())?;
        let mysql_pool = my_mysql::connect(&config_clone.database.mysql.unwrap()).await?;
        let redis_pool = my_redis::connect(&config_clone.database.redis.unwrap()).await?;
        // let metrics = Metrics::new();

        Ok(Self {
            config,
            jwt,
            mysql_pool,
            redis_pool,
            // metrics,
        })
    }
}

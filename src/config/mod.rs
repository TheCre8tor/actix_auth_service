use std::time::Duration;

use color_eyre::Result;
use dotenv::dotenv;
use eyre::WrapErr;
use serde::Deserialize;
use sqlx::PgPool;
use tracing::{info, instrument};
use tracing_subscriber::EnvFilter;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub host: String,
    pub port: i32,
    pub database_url: String,
}

impl Config {
    #[instrument]
    pub fn from_env() -> Result<Config> {
        dotenv().ok();

        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .init();

        info!("Loading configuration");

        let mut configuration = config::Config::new();
        configuration.merge(config::Environment::default())?;
        configuration
            .try_into()
            .context("Loading configuration from environment")
    }

    pub async fn db_pool(&self) -> Result<PgPool> {
        info!("Creating database connection pool.");

        PgPool::builder()
            .connect_timeout(Duration::from_secs(30))
            .build(&self.database_url)
            .await
            .context("Failed to create database connection pool.")
    }
}

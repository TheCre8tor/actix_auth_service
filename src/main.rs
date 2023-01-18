#[macro_use]
extern crate validator_derive;

mod config;
mod database;
mod handlers;
mod models;

use crate::{config::Config, handlers::app_config};
use actix_web::{middleware::Logger, App, HttpServer};
use color_eyre::Result;
use tracing::info;

#[actix_rt::main]
async fn main() -> Result<()> {
    let configuration = Config::from_env().expect("Failed to load server configuration");

    let pool = configuration
        .db_pool()
        .await
        .expect("Database configuration");

    let crypto_service = configuration.crypto_service();

    info!(
        "Starting server at http://{}:{}/",
        configuration.host, configuration.port
    );

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .data(pool.clone())
            .data(crypto_service.clone())
            .configure(app_config)
    })
    .bind(format!("{}:{}", configuration.host, configuration.port))?
    .run()
    .await?;

    Ok(())
}

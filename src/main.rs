mod config;
mod handlers;

use crate::{config::Config, handlers::app_config};
use actix_web::{middleware::Logger, App, HttpServer};
use color_eyre::Result;
use tracing::info;

#[actix_rt::main]
async fn main() -> Result<()> {
    let configuration = Config::from_env().expect("Failed to load server configuration");

    info!(
        "Starting server at http://{}:{}/",
        configuration.host, configuration.port
    );

    HttpServer::new(move || App::new().wrap(Logger::default()).configure(app_config))
        .bind(format!("{}:{}", configuration.host, configuration.port))?
        .run()
        .await?;

    Ok(())
}

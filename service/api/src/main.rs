mod config;
mod db;
mod http_handler;
use config::AppConfig;
use http_handler::function_handler;
use lambda_http::{run, service_fn, tracing, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenvy::dotenv().ok();
    tracing::init_default_subscriber();
    let config = AppConfig::from_env()?;

    tracing::info!(
        "loaded database config (runtime_url_set={}, migration_url_set={})",
        !config.database_url.is_empty(),
        config.migration_database_url.is_some()
    );

    run(service_fn(function_handler)).await
}

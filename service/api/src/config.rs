use std::env;

use lambda_http::Error;

#[derive(Clone, Debug)]
pub(crate) struct AppConfig {
    pub(crate) database_url: String,
    pub(crate) migration_database_url: Option<String>,
}

impl AppConfig {
    pub(crate) fn from_env() -> Result<Self, Error> {
        let database_url =
            env::var("DATABASE_URL").map_err(|_| "DATABASE_URL must be set for the API runtime")?;
        let migration_database_url = env::var("MIGRATION_DATABASE_URL").ok();

        Ok(Self {
            database_url,
            migration_database_url,
        })
    }
}

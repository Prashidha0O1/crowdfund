use dotenvy::dotenv;
use std::env;

/// Holds the application's configuration, loaded from environment variables.
#[derive(Clone, Debug)]
pub struct Config {
    pub database_url: String,
    pub google_client_id: String,
    pub google_client_secret: String,
    pub google_redirect_uri: String,
}

impl Config {
    /// Loads configuration from environment variables.
    /// Panics if any required variable is missing.
    pub fn from_env() -> Result<Self, env::VarError> {
        // Load .env file if it exists
        dotenv().ok();

        Ok(Self {
            database_url: env::var("DATABASE_URL")?,
            google_client_id: env::var("GOOGLE_CLIENT_ID")?,
            google_client_secret: env::var("GOOGLE_CLIENT_SECRET")?,
            google_redirect_uri: env::var("GOOGLE_REDIRECT_URI")?,
        })
    }
}

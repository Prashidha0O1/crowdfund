use sqlx::mysql::MySqlPool;
use tera::Tera;
use lazy_static::lazy_static;

use crate::config::Config;

// Publicly export handler modules
pub mod auth_handlers;
pub mod home_handlers;
pub mod user_handlers;
pub mod dashboard_handlers;

/// Shared application state accessible by all handlers.
pub struct AppState {
    pub db_pool: MySqlPool,
    pub config: Config,
}

// Lazily initialize the Tera templating engine.
// This parses all .html files in the `frontend` directory once on startup.
lazy_static! {
    pub static ref TEMPLATES: Tera = {
        // We now include the `templates` directory in the path
        match Tera::new("frontend/**/*.html") {
            Ok(t) => t,
            Err(e) => {
                println!("Template parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        }
    };
}

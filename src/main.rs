use axum::{
    routing::{get},
    Router,
};
use log::info;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;

mod config;
mod error;
mod handlers;
mod models;
mod auth;

use crate::config::Config;
// Corrected the handler imports
use crate::handlers::{auth_handlers, dashboard_handlers, home_handlers, user_handlers, AppState};

#[tokio::main]
async fn main() {
    // Initialize logging
    env_logger::init();

    // Load configuration from environment variables
    let config = Config::from_env().expect("Failed to load configuration from environment variables");
    
    // Create a database connection pool
    let db_pool = mysql_async::Pool::new(config.database_url.as_str());
    info!("Database connection pool created.");

    // Create the shared application state
    let app_state = Arc::new(AppState { db_pool, config });
    info!("Application state created.");

    // Define the application routes
    let app = Router::new()
        // Serve static files from the `frontend` directory
        .nest_service("/static", ServeDir::new("frontend"))
        
        // Application routes
        .route("/", get(home_handlers::landing_page))
        .route("/login", get(auth_handlers::login_page))
        .route("/dashboard", get(dashboard_handlers::dashboard_page))
        .route("/:username", get(user_handlers::profile_page))
        
        // Authentication routes
        .nest("/auth", auth_handlers::auth_router())

        // Add middleware for logging
        .layer(TraceLayer::new_for_http())
        
        // Provide the application state to all handlers
        .with_state(app_state);

    // Start the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 8081));
    info!("Server listening on http://{}", addr);
    let listener = TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

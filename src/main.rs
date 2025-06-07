mod auth;
mod routes;
use axum::{
    extract::{FromRef, Extension},
    response::Html,
    routing::get,
    Router,
};
use axum_extra::extract::cookie::Key;
use std::error::Error;
use std::net::SocketAddr;
use std::env;
use reqwest::Client as ReqwestClient;
use tokio::net::TcpListener;
use sqlx::mysql::MySqlPool;

#[derive(Clone)]
pub struct AppState {
    db: MySqlPool,
    ctx: ReqwestClient,
    key: Key,
    oauth_client_id: String,
}

impl FromRef<AppState> for Key {
    fn from_ref(state: &AppState) -> Self {
        state.key.clone()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in environment variables");
    let db = MySqlPool::connect(&database_url)
        .await
        .expect("Failed to connect to the database");

    sqlx::migrate!()
        .run(&db)
        .await
        .expect("Failed to run migrations");

    let ctx = ReqwestClient::new();
    let oauth_client_id = env::var("GOOGLE_OAUTH_CLIENT_ID")
        .expect("GOOGLE_OAUTH_CLIENT_ID must be set");

    let state = AppState {
        db,
        ctx,
        key: Key::generate(),
        oauth_client_id: oauth_client_id.clone(),
    };

    let router = Router::new()
        .route("/", get(routes::landing_page))
        .route("/login", get(routes::login_page))
        .route("/home", get(|Extension(oauth_id): Extension<String>| routes::homepage(oauth_id)))
        .route("/auth/google", get(auth::oauth::google_login))
        .route("/auth/google/callback", get(auth::oauth::google_callback))
        .with_state(state)
        .layer(Extension(oauth_client_id));

    // server address
    let addr = SocketAddr::from(([127, 0, 0, 1], 8081));
    let listener = TcpListener::bind(addr).await?;

    // Start the Axum server
    println!("Server running at http://{}", addr);
    axum::serve(listener, router).await?;

    Ok(())
}

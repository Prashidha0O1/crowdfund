use axum::{
    extract::FromRef,
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
}

impl FromRef<AppState> for Key {
    fn from_ref(state: &AppState) -> Self {
        state.key.clone()
    }
}

async fn hello_crowd() -> &'static str {
    "Hello crowds"
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
    let state = AppState {
        db,
        ctx,
        key: Key::generate(),
    };

    let router = Router::new()
        .route("/", get(hello_crowd))
        .with_state(state);

    // server address
    let addr = SocketAddr::from(([127, 0, 0, 1], 8081));
    let listener = TcpListener::bind(addr).await?;

    // Start the Axum server
    println!("Server running at http://{}", addr);
    axum::serve(listener, router).await?;

    Ok(())
}

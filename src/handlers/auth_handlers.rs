use axum::{
    extract::{Query, State},
    response::{Html, IntoResponse, Redirect},
    routing::get,
    Router,
};
use serde::Deserialize;
use std::sync::Arc;
use tera::Context;

// Corrected the module imports
use crate::auth::google_oauth::{get_google_user, request_token};
use crate::error::AppError;
use crate::models::user::User;
use crate::handlers::{AppState, TEMPLATES};

/// Structures the authentication routes.
pub fn auth_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/google", get(google_login_handler))
        .route("/google/callback", get(google_callback_handler))
}

/// Renders the login page.
pub async fn login_page() -> Result<impl IntoResponse, AppError> {
    let context = Context::new();
    let rendered = TEMPLATES.render("login.html", &context)?;
    Ok(Html(rendered))
}

/// Redirects the user to Google's OAuth 2.0 consent screen.
pub async fn google_login_handler(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let google_auth_url = format!(
        "https://accounts.google.com/o/oauth2/v2/auth?response_type=code&client_id={}&redirect_uri={}&scope=openid%20email%20profile",
        state.config.google_client_id, state.config.google_redirect_uri
    );
    Redirect::to(&google_auth_url)
}

#[derive(Deserialize)]
pub struct AuthCallback {
    code: String,
}

/// Handles the callback from Google after the user grants permission.
pub async fn google_callback_handler(
    Query(query): Query<AuthCallback>,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, AppError> {
    // Exchange the authorization code for an access token
    let token_response = request_token(&query.code, &state.config).await?;
    
    // Use the access token to get user info from Google
    let google_user = get_google_user(&token_response.access_token).await?;

    // Check if the user already exists, otherwise create them
    let _user = match User::find_by_google_id(&state.db_pool, &google_user.sub).await? {
        Some(existing_user) => existing_user,
        None => {
            // If user doesn't exist, create a new one
            User::create_from_google_user(&state.db_pool, &google_user).await?
        }
    };

    // Redirect to the user's dashboard
    // NOTE: In a real app, you would set a session cookie here.
    Ok(Redirect::to("/dashboard"))
}

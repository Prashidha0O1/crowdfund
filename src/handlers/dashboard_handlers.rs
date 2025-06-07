use axum::{
    extract::State,
    response::{Html, IntoResponse},
};
use mysql_async::prelude::*;
use std::sync::Arc;
use tera::Context;

use crate::{
    error::AppError,
    models::user::User,
    handlers::{AppState, TEMPLATES},
};

/// Renders the user's dashboard page.
///
/// NOTE: This is a placeholder. In a real application, you would get the
/// authenticated user's ID from a session cookie, not fetch a hardcoded user.
pub async fn dashboard_page(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, AppError> {
    let mut conn = state.db_pool.get_conn().await?;
    
    //
    // !! IMPORTANT !!
    // This fetches the *first* user from the database as a placeholder.
    // To make this work correctly, you need to implement session management
    // (e.g., using cookies) to identify the currently logged-in user.
    // Corrected the query to use mysql_async instead of sqlx
    let user: Option<User> = "SELECT * FROM users ORDER BY id DESC LIMIT 1"
        .first(&mut conn)
        .await?;

    let user = user.ok_or_else(|| AppError::AuthError("Not logged in. No users found.".to_string()))?;

    let mut context = Context::new();
    context.insert("user", &user);

    let rendered = TEMPLATES.render("dashboard.html", &context)?;
    Ok(Html(rendered))
}

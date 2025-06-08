use axum::{
    extract::State,
    response::{Html, IntoResponse},
};
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
    // Fetch the first user from the database as a placeholder
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT id, google_id, username, email, avatar_url, created_at
        FROM users
        ORDER BY id DESC
        LIMIT 1
        "#
    )
    .fetch_optional(&state.db_pool)
    .await?
    .ok_or_else(|| AppError::AuthError("Not logged in. No users found.".to_string()))?;

    let mut context = Context::new();
    context.insert("user", &user);

    let rendered = TEMPLATES.render("dashboard.html", &context)?;
    Ok(Html(rendered))
}

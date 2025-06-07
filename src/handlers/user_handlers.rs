use axum::{
    extract::{Path, State},
    response::Html,
};
use std::sync::Arc;
use tera::Context;

use crate::{
    error::AppError,
    models::user::User,
    handlers::{AppState, TEMPLATES},
};

/// Renders a creator's public profile page.
pub async fn profile_page(
    Path(username): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<Html<String>, AppError> {
    // Get a connection from the database pool
    let mut conn = state.db_pool.get_conn().await?;
    
    // Find the user by their username
    let user = User::find_by_username(&mut conn, &username)
        .await?
        .ok_or(AppError::UserNotFound)?;

    // Create a context and render the profile template
    let mut context = Context::new();
    context.insert("user", &user);

    let rendered = TEMPLATES.render("profile.html", &context)?;
    Ok(Html(rendered))
}

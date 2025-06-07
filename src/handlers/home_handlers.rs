use axum::response::{Html, IntoResponse};
use tera::Context;
use crate::error::AppError;
use crate::handlers::TEMPLATES;

/// Renders the landing page.
pub async fn landing_page() -> Result<impl IntoResponse, AppError> {
    let context = Context::new();
    let rendered = TEMPLATES.render("landing.html", &context)?;
    Ok(Html(rendered))
}

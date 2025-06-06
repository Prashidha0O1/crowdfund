use axum::{
    extract::Query,
    response::Redirect,
};
use oauth2::{
    basic::BasicClient,
    AuthUrl,
    ClientId,
    ClientSecret,
    RedirectUrl,
    TokenUrl,
    TokenResponse,
};
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
pub struct AuthRequest {
    code: String,
}

pub fn build_oauth_client() -> BasicClient {
    let client_id = ClientId::new(
        env::var("GOOGLE_OAUTH_CLIENT_ID").expect("Missing GOOGLE_OAUTH_CLIENT_ID"),
    );
    let client_secret = ClientSecret::new(
        env::var("GOOGLE_OAUTH_CLIENT_SECRET").expect("Missing GOOGLE_OAUTH_CLIENT_SECRET"),
    );
    let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())
        .expect("Invalid authorization endpoint URL");
    let token_url = TokenUrl::new("https://www.googleapis.com/oauth2/v3/token".to_string())
        .expect("Invalid token endpoint URL");
    let redirect_url = RedirectUrl::new("http://localhost:8081/auth/google/callback".to_string())
        .expect("Invalid redirect URL");

    BasicClient::new(client_id, Some(client_secret), auth_url, Some(token_url))
        .set_redirect_uri(redirect_url)
}

pub async fn google_login() -> Redirect {
    let client = build_oauth_client();
    let (auth_url, _csrf_token) = client
        .authorize_url(oauth2::CsrfToken::new_random)
        .add_scope(oauth2::Scope::new("profile".to_string()))
        .add_scope(oauth2::Scope::new("email".to_string()))
        .url();

    Redirect::to(auth_url.as_str())
}

pub async fn google_callback(Query(query): Query<AuthRequest>) -> Result<Redirect, String> {
    let client = build_oauth_client();
    let token = client
        .exchange_code(oauth2::AuthorizationCode::new(query.code))
        .request_async(oauth2::reqwest::async_http_client)
        .await
        .map_err(|e| e.to_string())?;

    let _user_info = reqwest::Client::new()
        .get("https://www.googleapis.com/oauth2/v3/userinfo")
        .bearer_auth(token.access_token().secret())
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json::<serde_json::Value>()
        .await
        .map_err(|e| e.to_string())?;

    // TODO: Create or update user in database
    // TODO: Create session
    // TODO: Set session cookie

    Ok(Redirect::to("/dashboard"))
}

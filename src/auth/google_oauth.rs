use reqwest::Client;
use serde::Deserialize;

use crate::{config::Config, error::AppError};

/// Represents the token response from Google's token endpoint.
#[derive(Deserialize, Debug)]
pub struct TokenResponse {
    pub access_token: String,
}

/// Represents the user information returned from Google's userinfo endpoint.
#[derive(Deserialize, Debug)]
pub struct GoogleUser {
    pub sub: String, // Google's unique ID for the user
    pub name: String,
    pub email: String,
    pub picture: String, // URL to the user's profile picture
}

/// Exchanges an authorization code for an access token.
pub async fn request_token(code: &str, config: &Config) -> Result<TokenResponse, AppError> {
    let client = Client::new();
    let params = [
        ("code", code),
        ("client_id", &config.google_client_id),
        ("client_secret", &config.google_client_secret),
        ("redirect_uri", &config.google_redirect_uri),
        ("grant_type", "authorization_code"),
    ];

    let response = client
        .post("https://oauth2.googleapis.com/token")
        .form(&params)
        .send()
        .await?
        .error_for_status()? // Ensure we have a successful status code
        .json::<TokenResponse>()
        .await?;

    Ok(response)
}

/// Fetches user information from Google using an access token.
pub async fn get_google_user(access_token: &str) -> Result<GoogleUser, AppError> {
    let client = Client::new();
    let response = client
        .get("https://www.googleapis.com/oauth2/v3/userinfo")
        .bearer_auth(access_token)
        .send()
        .await?
        .error_for_status()?
        .json::<GoogleUser>()
        .await?;
    Ok(response)
}

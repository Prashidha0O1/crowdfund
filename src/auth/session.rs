use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime};

#[derive(Debug, Serialize, Deserialize)]
pub struct Session {
    pub user_id: i32,
    pub email: String,
    pub expires_at: SystemTime,
}

impl Session {
    pub fn new(user_id: i32, email: String) -> Self {
        Self {
            user_id,
            email,
            expires_at: SystemTime::now() + Duration::from_secs(24 * 60 * 60), // 24 hours
        }
    }

    pub fn is_expired(&self) -> bool {
        SystemTime::now() > self.expires_at
    }
}

#[axum::async_trait]
impl<S> FromRequestParts<S> for Session
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(_parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // TODO: Implement session extraction from cookie
        // For now, return unauthorized
        Err(StatusCode::UNAUTHORIZED)
    }
} 
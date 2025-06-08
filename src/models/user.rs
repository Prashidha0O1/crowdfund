use sqlx::{FromRow, MySqlPool};
// Added missing imports from the `rand` crate
use rand::{distributions::Alphanumeric, Rng};
use serde::{Deserialize, Serialize};

// Corrected the module imports by splitting them into separate lines
use crate::auth::google_oauth::GoogleUser;
use crate::error::AppError;

/// Represents a user (creator) in the database.
#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct User {
    pub id: u64,
    pub google_id: String,
    pub username: String,
    pub email: String,
    pub avatar_url: Option<String>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl User {
    /// Creates a new user from Google OAuth info, ensuring the username is unique.
    pub async fn create_from_google_user(
        pool: &MySqlPool,
        google_user: &GoogleUser,
    ) -> Result<User, AppError> {
        let base_username = google_user.name.to_lowercase().replace(|c: char| !c.is_alphanumeric(), "");
        let mut username = base_username.clone();

        // Ensure username is unique
        let mut counter = 1;
        while Self::find_by_username(pool, &username).await?.is_some() {
            if counter == 1 {
                // For the first collision, append a short random string
                let suffix: String = rand::thread_rng()
                    .sample_iter(&Alphanumeric)
                    .take(4)
                    .map(char::from)
                    .collect();
                username = format!("{}_{}", base_username, suffix);
            } else {
                // For subsequent collisions, just increment
                username = format!("{}_{}", base_username, counter);
            }
            counter += 1;
        }

        // Insert the new user
        sqlx::query!(
            r#"
            INSERT INTO users (google_id, username, email, avatar_url)
            VALUES (?, ?, ?, ?)
            "#,
            google_user.sub,
            username,
            google_user.email,
            google_user.picture
        )
        .execute(pool)
        .await?;
        
        // Fetch the newly created user to return it
        Self::find_by_google_id(pool, &google_user.sub)
            .await?
            .ok_or(AppError::UserNotFound) // Should not happen
    }

    /// Finds a single user by their unique username.
    pub async fn find_by_username(
        pool: &MySqlPool,
        username: &str,
    ) -> Result<Option<User>, AppError> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, google_id, username, email, avatar_url, created_at
            FROM users
            WHERE username = ?
            "#,
            username
        )
        .fetch_optional(pool)
        .await?;
        
        Ok(user)
    }

    /// Finds a single user by their unique Google ID.
    pub async fn find_by_google_id(
        pool: &MySqlPool,
        google_id: &str,
    ) -> Result<Option<User>, AppError> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, google_id, username, email, avatar_url, created_at
            FROM users
            WHERE google_id = ?
            "#,
            google_id
        )
        .fetch_optional(pool)
        .await?;
        
        Ok(user)
    }
}
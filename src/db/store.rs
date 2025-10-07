use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use chrono::Utc;
use crate::models::user::User;
use crate::error::AppError;

#[derive(Clone)]
pub struct Store {
    users: Arc<RwLock<Vec<User>>>,
}

impl Store {
    pub fn new() -> Self {
        Self {
            users: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub async fn create_user(
        &self,
        email: String,
        name: String,
        password_hash: Option<String>,
        oauth_provider: Option<String>,
        oauth_id: Option<String>,
    ) -> Result<User, AppError> {
        let mut users = self.users.write().await;
        
        if users.iter().any(|u| u.email == email) {
            return Err(AppError::BadRequest("Email already exists".to_string()));
        }

        let now = Utc::now().naive_utc();
        let user = User {
            id: Uuid::new_v4(),
            email,
            name,
            password_hash,
            oauth_provider,
            oauth_id,
            created_at: now,
            updated_at: now,
        };

        users.push(user.clone());
        Ok(user)
    }

    pub async fn find_user_by_email(&self, email: &str) -> Result<Option<User>, AppError> {
        let users = self.users.read().await;
        Ok(users.iter().find(|u| u.email == email).cloned())
    }

    pub async fn find_user_by_oauth(
        &self,
        oauth_provider: &str,
        oauth_id: &str,
    ) -> Result<Option<User>, AppError> {
        let users = self.users.read().await;
        Ok(users
            .iter()
            .find(|u| {
                u.oauth_provider.as_ref().map(|p| p.as_str()) == Some(oauth_provider)
                    && u.oauth_id.as_ref().map(|i| i.as_str()) == Some(oauth_id)
            })
            .cloned())
    }

    pub async fn find_user_by_id(&self, id: Uuid) -> Result<Option<User>, AppError> {
        let users = self.users.read().await;
        Ok(users.iter().find(|u| u.id == id).cloned())
    }
}

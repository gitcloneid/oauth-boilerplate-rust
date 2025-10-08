use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use uuid::Uuid;
use chrono::{Utc, NaiveDateTime};
use crate::models::user::User;
use crate::error::AppError;
use crate::schema::users;
use crate::db::DbPool;

#[derive(Clone)]
pub struct DieselStore {
    pool: DbPool,
}

impl DieselStore {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    pub async fn create_user(
        &self,
        email: String,
        name: String,
        password_hash: Option<String>,
        oauth_provider: Option<String>,
        oauth_id: Option<String>,
    ) -> Result<User, AppError> {
        let pool_start = std::time::Instant::now();
        let mut conn = self.pool.get().await
            .map_err(|e| AppError::Pool(e.to_string()))?;
        tracing::debug!("DB: pool.get() took {}ms", pool_start.elapsed().as_millis());

        // Check if user already exists
        let check_start = std::time::Instant::now();
        let existing = users::table
            .filter(users::email.eq(&email))
            .first::<User>(&mut conn)
            .await
            .optional()
            .map_err(|e| AppError::Database(e))?;
        tracing::debug!("DB: check email exists took {}ms", check_start.elapsed().as_millis());

        if existing.is_some() {
            return Err(AppError::BadRequest("Email already exists".to_string()));
        }

        let now = Utc::now().naive_utc();
        let new_user = NewUser {
            id: Uuid::new_v4(),
            email,
            name,
            password_hash,
            oauth_provider,
            oauth_id,
            created_at: now,
            updated_at: now,
        };

        let insert_start = std::time::Instant::now();
        let user = diesel::insert_into(users::table)
            .values(&new_user)
            .get_result::<User>(&mut conn)
            .await
            .map_err(|e| AppError::Database(e))?;
        tracing::debug!("DB: insert user took {}ms", insert_start.elapsed().as_millis());

        Ok(user)
    }

    pub async fn find_user_by_email(&self, email: &str) -> Result<Option<User>, AppError> {
        let pool_start = std::time::Instant::now();
        let mut conn = self.pool.get().await
            .map_err(|e| AppError::Pool(e.to_string()))?;
        tracing::debug!("DB: pool.get() for find_user took {}ms", pool_start.elapsed().as_millis());

        let query_start = std::time::Instant::now();
        let user = users::table
            .filter(users::email.eq(email))
            .first::<User>(&mut conn)
            .await
            .optional()
            .map_err(|e| AppError::Database(e))?;
        tracing::debug!("DB: find user query took {}ms", query_start.elapsed().as_millis());

        Ok(user)
    }

    pub async fn find_user_by_oauth(
        &self,
        oauth_provider: &str,
        oauth_id: &str,
    ) -> Result<Option<User>, AppError> {
        let mut conn = self.pool.get().await
            .map_err(|e| AppError::Pool(e.to_string()))?;

        let user = users::table
            .filter(users::oauth_provider.eq(oauth_provider))
            .filter(users::oauth_id.eq(oauth_id))
            .first::<User>(&mut conn)
            .await
            .optional()
            .map_err(|e| AppError::Database(e))?;

        Ok(user)
    }

    pub async fn find_user_by_id(&self, id: Uuid) -> Result<Option<User>, AppError> {
        let mut conn = self.pool.get().await
            .map_err(|e| AppError::Pool(e.to_string()))?;

        let user = users::table
            .find(id)
            .first::<User>(&mut conn)
            .await
            .optional()
            .map_err(|e| AppError::Database(e))?;

        Ok(user)
    }

    pub async fn update_user_password(&self, id: Uuid, new_password_hash: String) -> Result<User, AppError> {
        let mut conn = self.pool.get().await
            .map_err(|e| AppError::Pool(e.to_string()))?;

        let now = Utc::now().naive_utc();

        let user = diesel::update(users::table.filter(users::id.eq(id)))
            .set((
                users::password_hash.eq(Some(new_password_hash)),
                users::updated_at.eq(now),
            ))
            .get_result::<User>(&mut conn)
            .await
            .map_err(|e| AppError::Database(e))?;

        Ok(user)
    }

    pub async fn delete_user(&self, id: Uuid) -> Result<(), AppError> {
        let mut conn = self.pool.get().await
            .map_err(|e| AppError::Pool(e.to_string()))?;

        diesel::delete(users::table.filter(users::id.eq(id)))
            .execute(&mut conn)
            .await
            .map_err(|e| AppError::Database(e))?;

        Ok(())
    }
}

#[derive(Insertable)]
#[diesel(table_name = users)]
struct NewUser {
    id: Uuid,
    email: String,
    name: String,
    password_hash: Option<String>,
    oauth_provider: Option<String>,
    oauth_id: Option<String>,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

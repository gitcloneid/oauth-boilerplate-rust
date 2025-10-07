use axum::{Json, extract::State, http::header};
use serde::Serialize;
use uuid::Uuid;
use crate::error::AppError;
use crate::handlers::auth_handler::AppState;
use crate::utils::jwt;

#[derive(Debug, Serialize)]
pub struct ProfileResponse {
    pub id: String,
    pub email: String,
    pub name: String,
    pub oauth_provider: Option<String>,
    pub created_at: String,
}

pub async fn get_profile(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
) -> Result<Json<ProfileResponse>, AppError> {
    let auth_header = headers
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .ok_or(AppError::Unauthorized)?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(AppError::Unauthorized)?;

    let claims = jwt::verify_token(token, &state.config.jwt.secret)?;
    
    let user_id = Uuid::parse_str(&claims.sub)
        .map_err(|_| AppError::Unauthorized)?;

    let user = state.store.find_user_by_id(user_id).await?
        .ok_or(AppError::NotFound)?;

    Ok(Json(ProfileResponse {
        id: user.id.to_string(),
        email: user.email,
        name: user.name,
        oauth_provider: user.oauth_provider,
        created_at: user.created_at.format("%Y-%m-%d %H:%M:%S").to_string(),
    }))
}

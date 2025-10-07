use axum::{
    Json,
    extract::{State, Query},
    response::{IntoResponse, Redirect},
};
use serde::{Deserialize, Serialize};
use oauth2::{
    AuthorizationCode, AuthUrl, ClientId, ClientSecret, CsrfToken, RedirectUrl,
    TokenUrl, TokenResponse, basic::BasicClient, reqwest::async_http_client,
};
use crate::error::AppError;
use crate::config::AppConfig;
use crate::db::DieselStore;
use crate::utils::{hashing, jwt};

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub store: DieselStore,
}

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserInfo,
}

#[derive(Debug, Serialize)]
pub struct UserInfo {
    pub id: String,
    pub email: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct GoogleUserInfo {
    pub id: String,
    pub email: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct OAuthCallbackQuery {
    pub code: String,
    pub state: String,
}

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    let password_hash = hashing::hash_password(&payload.password)?;

    let user = state.store.create_user(
        payload.email.clone(),
        payload.name.clone(),
        Some(password_hash),
        None,
        None,
    ).await?;

    let token = jwt::generate_token(
        user.id,
        &user.email,
        &user.name,
        &state.config.jwt.secret,
        state.config.jwt.expiration,
    )?;

    Ok(Json(AuthResponse {
        token,
        user: UserInfo {
            id: user.id.to_string(),
            email: user.email,
            name: user.name,
        },
    }))
}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    let user = state
        .store
        .find_user_by_email(&payload.email)
        .await?
        .ok_or(AppError::Unauthorized)?;

    if let Some(password_hash) = &user.password_hash {
        if !hashing::verify_password(&payload.password, password_hash)? {
            return Err(AppError::Unauthorized);
        }
    } else {
        return Err(AppError::BadRequest(
            "This account uses OAuth login".to_string()
        ));
    }

    let token = jwt::generate_token(
        user.id,
        &user.email,
        &user.name,
        &state.config.jwt.secret,
        state.config.jwt.expiration,
    )?;

    Ok(Json(AuthResponse {
        token,
        user: UserInfo {
            id: user.id.to_string(),
            email: user.email,
            name: user.name,
        },
    }))
}

pub async fn google_oauth(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let client = BasicClient::new(
        ClientId::new(state.config.google_oauth.client_id.clone()),
        Some(ClientSecret::new(state.config.google_oauth.client_secret.clone())),
        AuthUrl::new(state.config.google_oauth.auth_url.clone())
            .map_err(|_| AppError::Internal)?,
        Some(TokenUrl::new(state.config.google_oauth.token_url.clone())
            .map_err(|_| AppError::Internal)?),
    )
    .set_redirect_uri(
        RedirectUrl::new(state.config.google_oauth.redirect_url.clone())
            .map_err(|_| AppError::Internal)?,
    );

    let (auth_url, _csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(oauth2::Scope::new("https://www.googleapis.com/auth/userinfo.email".to_string()))
        .add_scope(oauth2::Scope::new("https://www.googleapis.com/auth/userinfo.profile".to_string()))
        .url();

    Ok(Redirect::to(auth_url.as_str()))
}

pub async fn google_oauth_callback(
    State(state): State<AppState>,
    Query(query): Query<OAuthCallbackQuery>,
) -> Result<Json<AuthResponse>, AppError> {
    let client = BasicClient::new(
        ClientId::new(state.config.google_oauth.client_id.clone()),
        Some(ClientSecret::new(state.config.google_oauth.client_secret.clone())),
        AuthUrl::new(state.config.google_oauth.auth_url.clone())
            .map_err(|_| AppError::Internal)?,
        Some(TokenUrl::new(state.config.google_oauth.token_url.clone())
            .map_err(|_| AppError::Internal)?),
    )
    .set_redirect_uri(
        RedirectUrl::new(state.config.google_oauth.redirect_url.clone())
            .map_err(|_| AppError::Internal)?,
    );

    let token = client
        .exchange_code(AuthorizationCode::new(query.code))
        .request_async(async_http_client)
        .await
        .map_err(|_| AppError::Unauthorized)?;

    let client = reqwest::Client::new();
    let google_user: GoogleUserInfo = client
        .get("https://www.googleapis.com/oauth2/v2/userinfo")
        .bearer_auth(token.access_token().secret())
        .send()
        .await
        .map_err(|_| AppError::Internal)?
        .json()
        .await
        .map_err(|_| AppError::Internal)?;

    let user = match state
        .store
        .find_user_by_oauth("google", &google_user.id)
        .await?
    {
        Some(user) => user,
        None => {
            state.store.create_user(
                google_user.email.clone(),
                google_user.name.clone(),
                None,
                Some("google".to_string()),
                Some(google_user.id.clone()),
            ).await?
        }
    };

    let jwt_token = jwt::generate_token(
        user.id,
        &user.email,
        &user.name,
        &state.config.jwt.secret,
        state.config.jwt.expiration,
    )?;

    Ok(Json(AuthResponse {
        token: jwt_token,
        user: UserInfo {
            id: user.id.to_string(),
            email: user.email,
            name: user.name,
        },
    }))
}

pub async fn logout() -> Result<Json<serde_json::Value>, AppError> {
    Ok(Json(serde_json::json!({
        "message": "Logged out successfully"
    })))
}

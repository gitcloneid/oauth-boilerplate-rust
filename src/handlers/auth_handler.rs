use axum::{
    Json,
    extract::{State, Query, ConnectInfo},
    response::{IntoResponse, Redirect},
};
use serde::{Deserialize, Serialize};
use oauth2::{
    AuthorizationCode, AuthUrl, ClientId, ClientSecret, CsrfToken, RedirectUrl,
    TokenUrl, TokenResponse, basic::BasicClient, reqwest::async_http_client,
};
use std::net::SocketAddr;
use crate::error::AppError;
use crate::config::AppConfig;
use crate::db::DieselStore;
use crate::utils::{hashing, jwt};
use crate::middleware::rate_limit::RateLimiter;

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub store: DieselStore,
    pub rate_limiter: RateLimiter,
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
    let start = std::time::Instant::now();
    let password_hash = hashing::hash_password(&payload.password)?;
    tracing::debug!("Register: hashing took {}ms", start.elapsed().as_millis());

    let db_start = std::time::Instant::now();
    let user = state.store.create_user(
        payload.email.clone(),
        payload.name.clone(),
        Some(password_hash),
        None,
        None,
    ).await?;
    tracing::debug!("Register: DB create_user took {}ms", db_start.elapsed().as_millis());

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
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    let client_ip = addr.ip().to_string();
    
    // Check IP-based rate limit (prevent brute force from single IP)
    if let Err(msg) = state.rate_limiter.check_ip_limit(&client_ip) {
        tracing::warn!("Rate limit exceeded for IP: {} - {}", client_ip, msg);
        return Err(AppError::TooManyRequests(msg));
    }
    
    // Check email-based rate limit (prevent credential stuffing)
    if let Err(msg) = state.rate_limiter.check_email_limit(&payload.email) {
        tracing::warn!("Rate limit exceeded for email: {} - {}", payload.email, msg);
        return Err(AppError::TooManyRequests(msg));
    }

    let db_start = std::time::Instant::now();
    let user = state
        .store
        .find_user_by_email(&payload.email)
        .await?
        .ok_or_else(|| {
            tracing::warn!("Login attempt for non-existent email: {}", payload.email);
            AppError::Unauthorized
        })?;
    tracing::debug!("Login: DB find_user took {}ms", db_start.elapsed().as_millis());

    if let Some(password_hash) = &user.password_hash {
        if !hashing::verify_password(&payload.password, password_hash)? {
            tracing::warn!(
                "Failed login attempt for email: {} from IP: {}", 
                payload.email, 
                client_ip
            );
            return Err(AppError::Unauthorized);
        }
    } else {
        return Err(AppError::BadRequest(
            "This account uses OAuth login".to_string()
        ));
    }

    // Successful login - reset email rate limit
    state.rate_limiter.reset_email_limit(&payload.email);
    tracing::info!("Successful login for email: {} from IP: {}", payload.email, client_ip);

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
            .map_err(|e| AppError::Internal(format!("Invalid auth URL: {}", e)))?,
        Some(TokenUrl::new(state.config.google_oauth.token_url.clone())
            .map_err(|e| AppError::Internal(format!("Invalid token URL: {}", e)))?),
    )
    .set_redirect_uri(
        RedirectUrl::new(state.config.google_oauth.redirect_url.clone())
            .map_err(|e| AppError::Internal(format!("Invalid redirect URL: {}", e)))?,
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
            .map_err(|e| AppError::Internal(format!("Invalid auth URL: {}", e)))?,
        Some(TokenUrl::new(state.config.google_oauth.token_url.clone())
            .map_err(|e| AppError::Internal(format!("Invalid token URL: {}", e)))?),
    )
    .set_redirect_uri(
        RedirectUrl::new(state.config.google_oauth.redirect_url.clone())
            .map_err(|e| AppError::Internal(format!("Invalid redirect URL: {}", e)))?,
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
        .map_err(|e| AppError::Internal(format!("Failed to fetch Google user info: {}", e)))?
        .json()
        .await
        .map_err(|e| AppError::Internal(format!("Failed to parse Google user info: {}", e)))?;

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

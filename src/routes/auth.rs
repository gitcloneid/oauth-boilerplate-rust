use axum::{routing::{post, get}, Router};
use crate::handlers::auth_handler::{self, AppState};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/auth/register", post(auth_handler::register))
        .route("/api/auth/login", post(auth_handler::login))
        .route("/api/auth/logout", post(auth_handler::logout))
        .route("/api/auth/google", get(auth_handler::google_oauth))
        .route("/api/auth/google/callback", get(auth_handler::google_oauth_callback))
}

use axum::{routing::get, Router};
use crate::handlers::user_handler;
use crate::handlers::auth_handler::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/profile", get(user_handler::get_profile))
}

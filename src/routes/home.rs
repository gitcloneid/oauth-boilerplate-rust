use axum::{routing::get, Router, Json};
use serde_json::json;
use crate::handlers::auth_handler::AppState;

async fn index() -> Json<serde_json::Value> {
    Json(json!({
        "message": "Welcome to auth_session API"
    }))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(index))
}

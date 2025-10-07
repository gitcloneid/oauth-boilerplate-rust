pub mod auth;
pub mod home;
pub mod profile;

use axum::Router;
use crate::handlers::auth_handler::AppState;

pub fn app_routes() -> Router<AppState> {
    Router::new()
        .merge(home::routes())
        .merge(auth::routes())
        .merge(profile::routes())
}

use auth_session::{
    config::AppConfig,
    db::{DieselStore, create_pool},
    handlers::auth_handler::AppState,
    routes,
};
use axum::Router;

pub async fn setup_test_app() -> Router {
    dotenvy::dotenv().ok();
    
    let config = AppConfig::new().expect("Failed to load test config");
    
    let pool = create_pool(
        &config.database.url,
        config.database.max_connections
    ).await.expect("Failed to create pool");
    
    let diesel_store = DieselStore::new(pool);
    
    let app_state = AppState {
        config: config.clone(),
        store: diesel_store,
    };

    routes::app_routes().with_state(app_state)
}

pub fn get_test_email() -> String {
    format!("test_{}@example.com", uuid::Uuid::new_v4())
}

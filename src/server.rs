use axum::Router;
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;
use crate::config::AppConfig;
use crate::error::AppError;
use crate::routes;
use crate::db::{DieselStore, create_pool};
use crate::handlers::auth_handler::AppState;

pub async fn run(config: AppConfig) -> Result<(), AppError> {
    tracing::info!("Creating database connection pool...");
    let pool = create_pool(
        &config.database.url,
        config.database.max_connections
    ).await?;
    
    let diesel_store = DieselStore::new(pool);
    
    let app_state = AppState {
        config: config.clone(),
        store: diesel_store,
    };

    let app = Router::new()
        .nest_service("/static", ServeDir::new("src/static"))
        .merge(routes::app_routes())
        .layer(CorsLayer::permissive())
        .with_state(app_state);

    let addr = format!("{}:{}", config.server.host, config.server.port)
        .parse::<SocketAddr>()
        .map_err(|e| AppError::BadRequest(format!("Invalid server address: {}", e)))?;
    let listener = tokio::net::TcpListener::bind(addr).await?;

    tracing::info!("Server running on {}", addr);
    tracing::info!("Using PostgreSQL database at: {}", config.database.url.split('@').last().unwrap_or("unknown"));
    
    axum::serve(listener, app).await?;

    Ok(())
}

use axum::Router;
use std::net::SocketAddr;
use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;
use tower::ServiceBuilder;
use crate::config::AppConfig;
use crate::error::AppError;
use crate::routes;
use crate::db::{DieselStore, create_pool};
use crate::handlers::auth_handler::AppState;
use crate::middleware::{timing, rate_limit::RateLimiter};

pub async fn run(config: AppConfig) -> Result<(), AppError> {
    tracing::debug!("Creating database connection pool...");
    let pool = create_pool(
        &config.database.url,
        config.database.max_connections
    ).await?;


    //warm up the database connections
    let warm_start = std::time::Instant::now();
    let mut handles = vec![];
    for _ in 0..3 {
        let pool_clone = pool.clone();
        handles.push(tokio::spawn(async move {
            let _ = pool_clone.get().await;
        }));
    }
    for handle in handles {
        let _ = handle.await;
    }
    tracing::debug!("Connection pool pre-warmed in {}ms", warm_start.elapsed().as_millis());

    let diesel_store = DieselStore::new(pool);

    // Initialize rate limiter
    // 10 attempts per IP per 3 minutes
    // 5 attempts per email per 3 minutes
    let rate_limiter = RateLimiter::new(
        10,   // max attempts per IP
        5,    // max attempts per email
        180,  // window in seconds (3 minutes)
    );
    tracing::info!("Rate limiter initialized: 10 attempts/IP, 5 attempts/email per 3 minutes");

    let app_state = AppState {
        config: config.clone(),
        store: diesel_store,
        rate_limiter,
    };

    let app = Router::new()
        .nest_service("/static", ServeDir::new("src/static"))
        .merge(routes::app_routes())
        .layer(
            ServiceBuilder::new()
                .layer(CorsLayer::permissive())
                .layer(axum::middleware::from_fn(timing::timing_middleware))
        )
        .with_state(app_state)
        .into_make_service_with_connect_info::<SocketAddr>();

    let addr = format!("{}:{}", config.server.host, config.server.port)
        .parse::<SocketAddr>()
        .map_err(|e| AppError::BadRequest(format!("Invalid server address: {}", e)))?;
    tracing::info!("Database Connected Succesfully");

    // Check if TLS certificates exist for HTTPS + HTTP/2
    if std::path::Path::new("certs/server.crt").exists() && std::path::Path::new("certs/server.key").exists() {
        let tls_config = load_tls_config()?;
        let rustls_config = axum_server::tls_rustls::RustlsConfig::from_config(Arc::new(tls_config));

        tracing::info!("🚀 HTTPS + HTTP/2 server running on https://{}", addr);

        axum_server::bind_rustls(addr, rustls_config)
            .serve(app)
            .await?;
    } else {
        tracing::warn!("⚠️  No TLS certificates found!");
        tracing::warn!("   Run 'powershell .\\generate_pem_key.ps1' to enable HTTPS + HTTP/2");
        let listener = tokio::net::TcpListener::bind(addr).await?;
        tracing::info!("🚀 HTTP server running on http://{}", addr);
        axum::serve(listener, app).await?;
    }

    Ok(())
}

fn load_tls_config() -> Result<rustls::ServerConfig, AppError> {
    // Load certificate file
    let cert_file = File::open("certs/server.crt")
        .map_err(|e| AppError::Internal(format!("Failed to open certificate file: {}", e)))?;
    let mut cert_reader = BufReader::new(cert_file);

    // Parse certificates - rustls_pemfile::certs returns Result<Vec<CertificateDer>, io::Error>
    let certs = rustls_pemfile::certs(&mut cert_reader)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| AppError::Internal(format!("Failed to parse certificates: {}", e)))?;

    let key_file = File::open("certs/server.key")
        .map_err(|e| AppError::Internal(format!("Failed to open private key file: {}", e)))?;
    let mut key_reader = BufReader::new(key_file);

    let key = rustls_pemfile::private_key(&mut key_reader)
        .map_err(|e| AppError::Internal(format!("Failed to parse private key: {}", e)))?
        .ok_or(AppError::Internal("No private key found".to_string()))?;

    let config = rustls::ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(certs, key)
        .map_err(|e| AppError::Internal(format!("Failed to create TLS config: {}", e)))?;

    tracing::debug!("TLS configuration loaded successfully");
    Ok(config)
}


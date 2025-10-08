use axum::{
    extract::Request,
    middleware::Next,
    response::Response,
};
use std::time::Instant;

pub async fn timing_middleware(req: Request, next: Next) -> Response {
    let start = Instant::now();
    let method = req.method().clone();
    let uri = req.uri().clone();
    
    let response = next.run(req).await;
    
    let duration = start.elapsed();
    let status = response.status();
    tracing::info!(
        method = %method,
        uri = %uri,
        status = %status,
        duration_ms = %duration.as_millis(),
        "Request completed"
    );
    
    response
}

use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::AsyncPgConnection;
use crate::error::AppError;

pub type DbPool = Pool<AsyncPgConnection>;

pub async fn create_pool(database_url: &str, max_connections: u32) -> Result<DbPool, AppError> {
    let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(database_url);
    
    let pool = Pool::builder(config)
        .max_size(max_connections as usize)
        .build()
        .map_err(|e| AppError::Pool(e.to_string()))?;

    Ok(pool)
}

use std::env;
use anyhow::{Context, Result};

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub jwt: JwtConfig,
    pub google_oauth: GoogleOAuthConfig,
}

#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
}

#[derive(Debug, Clone)]
pub struct JwtConfig {
    pub secret: String,
    pub expiration: i64,
}

#[derive(Debug, Clone)]
pub struct GoogleOAuthConfig {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_url: String,
    pub auth_url: String,
    pub token_url: String,
}

impl AppConfig {
    pub fn new() -> Result<Self> {
        Ok(AppConfig {
            server: ServerConfig {
                host: env::var("SERVER_HOST").context("SERVER_HOST must be set")?,
                port: env::var("SERVER_PORT")
                    .context("SERVER_PORT must be set")?
                    .parse()
                    .context("SERVER_PORT must be a valid number")?,
            },
            database: DatabaseConfig {
                url: env::var("DATABASE_URL").context("DATABASE_URL must be set")?,
                max_connections: env::var("DATABASE_MAX_CONNECTIONS")
                    .context("DATABASE_MAX_CONNECTIONS must be set")?
                    .parse()
                    .context("DATABASE_MAX_CONNECTIONS must be a valid number")?,
            },
            jwt: JwtConfig {
                secret: env::var("JWT_SECRET").context("JWT_SECRET must be set")?,
                expiration: env::var("JWT_EXPIRATION")
                    .context("JWT_EXPIRATION must be set")?
                    .parse()
                    .context("JWT_EXPIRATION must be a valid number")?,
            },
            google_oauth: GoogleOAuthConfig {
                client_id: env::var("GOOGLE_OAUTH_CLIENT_ID")
                    .context("GOOGLE_OAUTH_CLIENT_ID must be set")?,
                client_secret: env::var("GOOGLE_OAUTH_CLIENT_SECRET")
                    .context("GOOGLE_OAUTH_CLIENT_SECRET must be set")?,
                redirect_url: env::var("GOOGLE_OAUTH_REDIRECT_URL")
                    .context("GOOGLE_OAUTH_REDIRECT_URL must be set")?,
                auth_url: env::var("GOOGLE_OAUTH_AUTH_URL")
                    .context("GOOGLE_OAUTH_AUTH_URL must be set")?,
                token_url: env::var("GOOGLE_OAUTH_TOKEN_URL")
                    .context("GOOGLE_OAUTH_TOKEN_URL must be set")?,
            },
        })
    }
}

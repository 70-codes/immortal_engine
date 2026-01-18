//! Configuration and error template generation for Immortal Engine
//!
//! This module provides utilities for generating configuration and error
//! handling code for generated Rust projects.

use crate::rust::{AuthFramework, DatabaseBackend};

/// Generate config.rs content
pub fn generate_config(_framework: AuthFramework, db_backend: DatabaseBackend) -> String {
    let db_env = match db_backend {
        DatabaseBackend::Postgres => "DATABASE_URL",
        DatabaseBackend::Sqlite => "DATABASE_URL",
        DatabaseBackend::Mysql => "DATABASE_URL",
    };

    let db_pool_type = match db_backend {
        DatabaseBackend::Postgres => "sqlx::PgPool",
        DatabaseBackend::Sqlite => "sqlx::SqlitePool",
        DatabaseBackend::Mysql => "sqlx::MySqlPool",
    };

    format!(
        r#"//! Application configuration
//!
//! Configuration is loaded from environment variables.

use std::env;

/// Application configuration
#[derive(Debug, Clone)]
pub struct Config {{
    /// Server host address
    pub host: String,
    /// Server port
    pub port: u16,
    /// Database connection URL
    pub database_url: String,
    /// JWT secret for authentication
    pub jwt_secret: Option<String>,
    /// JWT token expiry in hours
    pub jwt_expiry_hours: u64,
    /// Environment (development, production, test)
    pub environment: Environment,
}}

/// Runtime environment
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Environment {{
    Development,
    Production,
    Test,
}}

impl Environment {{
    pub fn from_str(s: &str) -> Self {{
        match s.to_lowercase().as_str() {{
            "production" | "prod" => Self::Production,
            "test" => Self::Test,
            _ => Self::Development,
        }}
    }}

    pub fn is_development(&self) -> bool {{
        matches!(self, Self::Development)
    }}

    pub fn is_production(&self) -> bool {{
        matches!(self, Self::Production)
    }}

    pub fn is_test(&self) -> bool {{
        matches!(self, Self::Test)
    }}
}}

impl Config {{
    /// Load configuration from environment variables
    pub fn from_env() -> anyhow::Result<Self> {{
        Ok(Self {{
            host: env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            port: env::var("PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()?,
            database_url: env::var("{}")
                .expect("{} must be set"),
            jwt_secret: env::var("JWT_SECRET").ok(),
            jwt_expiry_hours: env::var("JWT_EXPIRY_HOURS")
                .unwrap_or_else(|_| "24".to_string())
                .parse()
                .unwrap_or(24),
            environment: Environment::from_str(
                &env::var("ENVIRONMENT").unwrap_or_else(|_| "development".to_string())
            ),
        }})
    }}

    /// Get the full server address
    pub fn server_addr(&self) -> String {{
        format!("{{}}:{{}}", self.host, self.port)
    }}

    /// Check if running in development mode
    pub fn is_development(&self) -> bool {{
        self.environment.is_development()
    }}

    /// Check if running in production mode
    pub fn is_production(&self) -> bool {{
        self.environment.is_production()
    }}
}}

/// Database connection pool type alias
pub type DatabasePool = {db_pool_type};

/// Initialize the database connection pool
pub async fn init_database(database_url: &str) -> anyhow::Result<DatabasePool> {{
    let pool = DatabasePool::connect(database_url).await?;

    // Run any pending migrations
    // sqlx::migrate!("./migrations").run(&pool).await?;

    Ok(pool)
}}

impl Default for Config {{
    fn default() -> Self {{
        Self {{
            host: "127.0.0.1".to_string(),
            port: 3000,
            database_url: String::new(),
            jwt_secret: None,
            jwt_expiry_hours: 24,
            environment: Environment::Development,
        }}
    }}
}}

#[cfg(test)]
mod tests {{
    use super::*;

    #[test]
    fn test_environment_from_str() {{
        assert_eq!(Environment::from_str("development"), Environment::Development);
        assert_eq!(Environment::from_str("production"), Environment::Production);
        assert_eq!(Environment::from_str("prod"), Environment::Production);
        assert_eq!(Environment::from_str("test"), Environment::Test);
        assert_eq!(Environment::from_str("unknown"), Environment::Development);
    }}

    #[test]
    fn test_config_server_addr() {{
        let config = Config {{
            host: "0.0.0.0".to_string(),
            port: 8080,
            ..Default::default()
        }};
        assert_eq!(config.server_addr(), "0.0.0.0:8080");
    }}

    #[test]
    fn test_environment_checks() {{
        assert!(Environment::Development.is_development());
        assert!(!Environment::Development.is_production());
        assert!(Environment::Production.is_production());
        assert!(Environment::Test.is_test());
    }}
}}
"#,
        db_env,
        db_env,
        db_pool_type = db_pool_type,
    )
}

/// Generate error.rs content
pub fn generate_error(framework: AuthFramework) -> String {
    match framework {
        AuthFramework::Axum => generate_axum_error(),
        AuthFramework::Actix => generate_actix_error(),
        AuthFramework::Custom => generate_custom_error(),
    }
}

fn generate_axum_error() -> String {
    r#"//! Application error types
//!
//! Centralized error handling for the application.

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

/// Application error type
#[derive(Debug, Error)]
pub enum AppError {
    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Forbidden")]
    Forbidden,

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Internal server error: {0}")]
    Internal(String),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg.clone()),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized".to_string()),
            AppError::Forbidden => (StatusCode::FORBIDDEN, "Forbidden".to_string()),
            AppError::Conflict(msg) => (StatusCode::CONFLICT, msg.clone()),
            AppError::Validation(msg) => (StatusCode::UNPROCESSABLE_ENTITY, msg.clone()),
            AppError::Database(e) => {
                tracing::error!("Database error: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Database error".to_string())
            }
            AppError::Internal(msg) => {
                tracing::error!("Internal error: {}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string())
            }
            AppError::Other(e) => {
                tracing::error!("Unexpected error: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string())
            }
        };

        let body = Json(json!({
            "error": {
                "message": message,
                "code": status.as_u16(),
            }
        }));

        (status, body).into_response()
    }
}

/// Result type alias for handlers
pub type AppResult<T> = Result<T, AppError>;

/// Helper to convert Option to NotFound error
pub trait OptionExt<T> {
    fn or_not_found(self, message: impl Into<String>) -> AppResult<T>;
}

impl<T> OptionExt<T> for Option<T> {
    fn or_not_found(self, message: impl Into<String>) -> AppResult<T> {
        self.ok_or_else(|| AppError::NotFound(message.into()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_option_ext() {
        let some: Option<i32> = Some(42);
        assert!(some.or_not_found("not found").is_ok());

        let none: Option<i32> = None;
        assert!(matches!(
            none.or_not_found("not found"),
            Err(AppError::NotFound(_))
        ));
    }
}
"#.to_string()
}

fn generate_actix_error() -> String {
    r#"//! Application error types
//!
//! Centralized error handling for the application.

use actix_web::{
    http::StatusCode,
    HttpResponse,
    ResponseError,
};
use serde_json::json;
use thiserror::Error;

/// Application error type
#[derive(Debug, Error)]
pub enum AppError {
    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Forbidden")]
    Forbidden,

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Internal server error: {0}")]
    Internal(String),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            AppError::Unauthorized => StatusCode::UNAUTHORIZED,
            AppError::Forbidden => StatusCode::FORBIDDEN,
            AppError::Conflict(_) => StatusCode::CONFLICT,
            AppError::Validation(_) => StatusCode::UNPROCESSABLE_ENTITY,
            AppError::Database(_) | AppError::Internal(_) | AppError::Other(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }

    fn error_response(&self) -> HttpResponse {
        let message = match self {
            AppError::NotFound(msg) => msg.clone(),
            AppError::BadRequest(msg) => msg.clone(),
            AppError::Unauthorized => "Unauthorized".to_string(),
            AppError::Forbidden => "Forbidden".to_string(),
            AppError::Conflict(msg) => msg.clone(),
            AppError::Validation(msg) => msg.clone(),
            AppError::Database(e) => {
                tracing::error!("Database error: {:?}", e);
                "Database error".to_string()
            }
            AppError::Internal(msg) => {
                tracing::error!("Internal error: {}", msg);
                "Internal server error".to_string()
            }
            AppError::Other(e) => {
                tracing::error!("Unexpected error: {:?}", e);
                "Internal server error".to_string()
            }
        };

        HttpResponse::build(self.status_code()).json(json!({
            "error": {
                "message": message,
                "code": self.status_code().as_u16(),
            }
        }))
    }
}

/// Result type alias for handlers
pub type AppResult<T> = Result<T, AppError>;

/// Helper to convert Option to NotFound error
pub trait OptionExt<T> {
    fn or_not_found(self, message: impl Into<String>) -> AppResult<T>;
}

impl<T> OptionExt<T> for Option<T> {
    fn or_not_found(self, message: impl Into<String>) -> AppResult<T> {
        self.ok_or_else(|| AppError::NotFound(message.into()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_codes() {
        assert_eq!(AppError::NotFound("x".into()).status_code(), StatusCode::NOT_FOUND);
        assert_eq!(AppError::Unauthorized.status_code(), StatusCode::UNAUTHORIZED);
        assert_eq!(AppError::BadRequest("x".into()).status_code(), StatusCode::BAD_REQUEST);
    }
}
"#.to_string()
}

fn generate_custom_error() -> String {
    r#"//! Application error types
//!
//! Centralized error handling for the application.

use thiserror::Error;

/// Application error type
#[derive(Debug, Error)]
pub enum AppError {
    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Forbidden")]
    Forbidden,

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Database error: {0}")]
    Database(String),

    #[error("Internal server error: {0}")]
    Internal(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

/// Result type alias
pub type AppResult<T> = Result<T, AppError>;

/// Helper to convert Option to NotFound error
pub trait OptionExt<T> {
    fn or_not_found(self, message: impl Into<String>) -> AppResult<T>;
}

impl<T> OptionExt<T> for Option<T> {
    fn or_not_found(self, message: impl Into<String>) -> AppResult<T> {
        self.ok_or_else(|| AppError::NotFound(message.into()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = AppError::NotFound("User".to_string());
        assert_eq!(err.to_string(), "Not found: User");
    }

    #[test]
    fn test_option_ext() {
        let some: Option<i32> = Some(42);
        assert!(some.or_not_found("not found").is_ok());

        let none: Option<i32> = None;
        assert!(matches!(
            none.or_not_found("not found"),
            Err(AppError::NotFound(_))
        ));
    }
}
"#.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_config_postgres() {
        let config = generate_config(AuthFramework::Axum, DatabaseBackend::Postgres);
        assert!(config.contains("PgPool"));
        assert!(config.contains("DATABASE_URL"));
    }

    #[test]
    fn test_generate_config_sqlite() {
        let config = generate_config(AuthFramework::Axum, DatabaseBackend::Sqlite);
        assert!(config.contains("SqlitePool"));
    }

    #[test]
    fn test_generate_axum_error() {
        let error = generate_error(AuthFramework::Axum);
        assert!(error.contains("IntoResponse"));
        assert!(error.contains("StatusCode"));
    }

    #[test]
    fn test_generate_actix_error() {
        let error = generate_error(AuthFramework::Actix);
        assert!(error.contains("ResponseError"));
    }
}

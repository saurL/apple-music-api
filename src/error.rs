//! Error types for the Apple Music API client

use std::fmt;

/// Main error type for Apple Music API operations
#[derive(thiserror::Error, Debug)]
pub enum AppleMusicError {
    /// HTTP-related errors
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    /// Authentication errors
    #[error("Authentication error: {0}")]
    Auth(String),

    /// API errors returned by Apple Music
    #[error("API error: {status} - {message}")]
    Api { status: u16, message: String },

    /// JSON serialization/deserialization errors
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// Invalid request parameters
    #[error("Invalid request: {0}")]
    InvalidRequest(String),

    /// JWT token creation/verification errors
    #[error("JWT error: {0}")]
    Jwt(#[from] jsonwebtoken::errors::Error),

    /// URL parsing errors
    #[error("URL error: {0}")]
    Url(#[from] url::ParseError),

    /// Configuration errors
    #[error("Configuration error: {0}")]
    Config(String),

    /// Timeout errors
    #[error("Request timeout: {0}")]
    Timeout(String),

    /// Rate limiting errors
    #[error("Rate limit exceeded: {0}")]
    RateLimit(String),
}

/// Result type alias for Apple Music operations
pub type Result<T> = std::result::Result<T, AppleMusicError>;

/// Apple Music API error response structure
#[derive(serde::Deserialize, Debug)]
pub struct ApiErrorResponse {
    pub errors: Vec<ApiError>,
}

/// Individual API error from Apple Music
#[derive(serde::Deserialize, Debug)]
pub struct ApiError {
    /// Error code
    pub code: String,

    /// Human-readable error message
    pub detail: String,

    /// HTTP status code
    pub status: String,

    /// Error title
    pub title: String,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {} ({})", self.title, self.detail, self.code)
    }
}

impl AppleMusicError {
    /// Create a new authentication error
    pub fn auth<S: Into<String>>(message: S) -> Self {
        Self::Auth(message.into())
    }

    /// Create a new configuration error
    pub fn config<S: Into<String>>(message: S) -> Self {
        Self::Config(message.into())
    }

    /// Create a new invalid request error
    pub fn invalid_request<S: Into<String>>(message: S) -> Self {
        Self::InvalidRequest(message.into())
    }

    /// Check if the error is retryable
    pub fn is_retryable(&self) -> bool {
        match self {
            Self::Http(err) => {
                err.is_timeout()
                    || err.is_connect()
                    || err.status().map_or(false, |s| s.is_server_error())
            }
            Self::RateLimit(_) => true,
            Self::Timeout(_) => true,
            _ => false,
        }
    }

    /// Get the HTTP status code if available
    pub fn status_code(&self) -> Option<u16> {
        match self {
            Self::Http(err) => err.status().map(|s| s.as_u16()),
            Self::Api { status, .. } => Some(*status),
            _ => None,
        }
    }
}

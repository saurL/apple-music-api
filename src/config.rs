//! Configuration for the Apple Music API client

use crate::error::{AppleMusicError, Result};
use derive_builder::Builder;
use std::time::Duration;

#[derive(Debug, Builder, Clone, Default)]
#[builder(pattern = "owned", default)]
pub struct ClientConfig {
    #[builder(default = "\"https://api.music.apple.com\".to_string()")]
    pub base_url: String,

    #[builder(default)]
    pub user_token: Option<String>,

    #[builder(default = "Duration::from_secs(30)")]
    pub timeout: Duration,

    #[builder(default = "3")]
    pub max_retries: u32,

    #[builder(default = "Duration::from_millis(100)")]
    pub retry_delay: Duration,

    #[builder(default = "format!(\"apple-music-api/{}\", env!(\"CARGO_PKG_VERSION\"))")]
    pub user_agent: String,

    #[builder(default = "\"us\".to_string()")]
    pub storefront: String,

    // Champs obligatoires (pas de #[builder(default)])
    pub team_id: String,
    pub key_id: String,

    pub developer_token: String,
}

impl ClientConfig {
    /// Create a new client configuration with a developer token
    pub fn new<S: Into<String>>(key_id: S, team_id: S, private_key: S) -> Result<Self> {
        let key_id = key_id.into();
        let team_id = team_id.into();
        let private_key = private_key.into();

        // Generate the developer token
        let developer_token = crate::auth::create_developer_token(&team_id, &key_id, &private_key)?;

        Ok(ClientConfigBuilder::default()
            .developer_token(developer_token)
            .key_id(key_id)
            .team_id(team_id)
            .build()
            .unwrap())
    }

    /// Validate the configuration
    pub fn validate(&self) -> Result<()> {
        if self.developer_token.is_empty() {
            return Err(AppleMusicError::config("Developer token is required"));
        }

        if self.base_url.is_empty() {
            return Err(AppleMusicError::config("Base URL cannot be empty"));
        }

        if self.storefront.is_empty() {
            return Err(AppleMusicError::config("Storefront cannot be empty"));
        }

        // Validate base URL format
        if !self.base_url.starts_with("http") {
            return Err(AppleMusicError::config(
                "Base URL must start with http:// or https://",
            ));
        }

        Ok(())
    }
}

/// Media types supported by the Apple Music API
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaType {
    Songs,
    Albums,
    Artists,
    Playlists,
    MusicVideos,
    Stations,
    AppleCurators,
    Curators,
}

impl MediaType {
    /// Get the string representation for API requests
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Songs => "songs",
            Self::Albums => "albums",
            Self::Artists => "artists",
            Self::Playlists => "playlists",
            Self::MusicVideos => "music-videos",
            Self::Stations => "stations",
            Self::AppleCurators => "apple-curators",
            Self::Curators => "curators",
        }
    }

    /// Get all media types as a slice
    pub fn all() -> &'static [MediaType] {
        &[
            Self::Songs,
            Self::Albums,
            Self::Artists,
            Self::Playlists,
            Self::MusicVideos,
            Self::Stations,
            Self::AppleCurators,
            Self::Curators,
        ]
    }
}

impl std::fmt::Display for MediaType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Search options for API requests
#[derive(Debug, Clone, Default)]
pub struct SearchOptions {
    /// Limit the number of results
    pub limit: Option<u32>,

    /// Offset for pagination
    pub offset: Option<u32>,

    /// Media types to search in
    pub types: Vec<MediaType>,
}

impl SearchOptions {
    /// Create new search options
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the result limit
    pub fn with_limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Set the offset for pagination
    pub fn with_offset(mut self, offset: u32) -> Self {
        self.offset = Some(offset);
        self
    }

    /// Add media types to search in
    pub fn with_types(mut self, types: Vec<MediaType>) -> Self {
        self.types = types;
        self
    }
}
